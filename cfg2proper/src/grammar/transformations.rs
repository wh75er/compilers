use crate::grammar::{Grammar, Production, Symbol, SymbolsKind, EPSILON_SYMBOL, U_CODEPOINT};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter;

pub fn get_productive(g: &Grammar) -> HashSet<String> {
    let mut old_set: HashSet<String> = HashSet::new();
    old_set.insert(EPSILON_SYMBOL.to_string());

    _handle_n_e_loop(
        |union: &mut HashSet<String>, old_set: &HashSet<String>| {
            *union = iter::empty::<&String>()
                .chain(union.union(old_set))
                .chain(union.union(&g.terms))
                .cloned()
                .collect();
        },
        g,
        &mut old_set,
    );

    old_set
}

// Provides non-terminals which lead to epsilon symbol
pub fn get_nullable_non_terms(g: &Grammar) -> HashSet<String> {
    let mut old_set: HashSet<_> = HashSet::new();
    old_set.insert(EPSILON_SYMBOL.to_string());

    _handle_n_e_loop(
        |union: &mut HashSet<String>, old_set: &HashSet<String>| {
            *union = union.union(old_set).cloned().collect();
        },
        g,
        &mut old_set,
    );

    old_set
}

fn _handle_n_e_loop<UnionClosure>(
    union_closure: UnionClosure,
    g: &Grammar,
    old_set: &mut HashSet<String>,
) where
    UnionClosure: Fn(&mut HashSet<String>, &HashSet<String>),
{
    loop {
        let mut new_set: HashSet<_> = HashSet::new();

        let mut union: HashSet<String> = HashSet::new();
        union_closure(&mut union, old_set);

        for prod in g.productions.iter() {
            let right = prod
                .expression
                .iter()
                .map(|symbol| symbol.value.clone())
                .collect::<HashSet<_>>();

            if right.is_subset(&union) {
                let left = &prod.replaced_symbol;
                new_set.insert(left.value.clone());
            }
        }

        new_set = new_set.union(old_set).cloned().collect();

        if new_set == *old_set {
            break;
        }

        *old_set = new_set;
    }
}

pub fn remove_useless_symbols(g: &Grammar) -> Grammar {
    let n_e = get_productive(g);
    let n1: HashSet<_> = g.non_terms.intersection(&n_e).cloned().collect();

    let mut union: HashSet<_> = HashSet::new();
    union = iter::empty::<&String>()
        .chain(union.union(&n_e))
        .chain(union.union(&g.terms))
        .cloned()
        .collect();

    let p1: Vec<_> = g
        .productions
        .iter()
        .filter(|prod| {
            let mut prod_symbols_set: HashSet<_> =
                prod.expression.iter().map(|v| v.value.clone()).collect();

            prod_symbols_set.insert(prod.replaced_symbol.value.clone());

            prod_symbols_set.is_subset(&union)
        })
        .cloned()
        .collect();

    let g1 = Grammar::new(n1, g.terms.clone(), p1, g.start.clone());

    remove_unreachable(&g1)
}

pub fn remove_unreachable(g: &Grammar) -> Grammar {
    let mut old_set: HashSet<String> = HashSet::new();
    let mut new_set: HashSet<String> = [g.start.clone()].iter().cloned().collect();

    while old_set != new_set {
        old_set = new_set.clone();
        for v in old_set.iter().cloned() {
            for prod in g.productions.iter() {
                if prod.replaced_symbol.value == v {
                    new_set.extend(
                        prod.expression
                            .iter()
                            .cloned()
                            .map(|symbol| symbol.value)
                            .collect::<HashSet<String>>(),
                    )
                }
            }
        }
    }

    let new_productions: Vec<Production> = g
        .productions
        .iter()
        .filter(|prod| {
            let mut production_symbols: HashSet<String> = HashSet::new();
            production_symbols.insert(prod.replaced_symbol.value.clone());
            production_symbols.extend(
                prod.expression
                    .iter()
                    .cloned()
                    .map(|v| v.value)
                    .collect::<HashSet<String>>(),
            );

            production_symbols.is_subset(&new_set)
        })
        .cloned()
        .collect();

    let new_non_terms = new_set
        .intersection(&g.non_terms)
        .cloned()
        .collect::<HashSet<String>>();
    let new_terms = new_set
        .intersection(&g.terms)
        .cloned()
        .collect::<HashSet<String>>();

    Grammar::new(new_non_terms, new_terms, new_productions, g.start.clone())
}

pub fn to_e_free(g: &Grammar) -> Grammar {
    let n_e = get_nullable_non_terms(g);

    let mut new_start: String = g.start.clone();

    // Removing epsilon productions
    let productions_without_epsilon = remove_epsilon_productions(&g.productions);

    let mut new_productions: Vec<Production> = vec![];

    // Building new productions with compensated deleted non-terminals
    for prod in productions_without_epsilon.iter() {
        compensate_epsilon_deletion(&mut new_productions, &prod, &n_e);
    }

    let mut new_non_terms: HashSet<String> = g.non_terms.clone();

    // S' -> S | e
    if n_e.contains(&g.start) {
        new_start = String::from(&g.start) + U_CODEPOINT;
        new_productions.push(Production::new(vec![
            (SymbolsKind::NONTERM, new_start.clone()),
            (SymbolsKind::NONTERM, g.start.clone()),
        ]));
        new_productions.push(Production::new(vec![
            (SymbolsKind::NONTERM, new_start.clone()),
            (SymbolsKind::EPSILON, EPSILON_SYMBOL.to_string()),
        ]));

        new_non_terms.insert(new_start.clone());
    }

    Grammar::new(
        new_non_terms,
        g.terms.clone(),
        new_productions,
        new_start.into(),
    )
}

fn compensate_epsilon_deletion(
    new_productions: &mut Vec<Production>,
    prod: &Production,
    n_e: &HashSet<String>,
) {
    let nullable_idxs: Vec<_> = prod
        .expression
        .iter()
        .map(|symbol| symbol.value.clone())
        .enumerate()
        .filter(|numeration| n_e.contains(&numeration.1))
        .map(|numeration| numeration.0)
        .collect();

    //  Since all symbols in the production are nullable
    // Ignore it
    if nullable_idxs.len() == prod.expression.len() {
        return;
    }

    //  In case we do not have any nullable terms in production
    // Just append it to result
    if nullable_idxs.is_empty() {
        new_productions.push(prod.clone());

        return;
    }

    for i in 0..nullable_idxs.len() + 1 {
        let mut prod_symbols: Vec<Symbol> = vec![prod.replaced_symbol.clone()];
        prod_symbols.extend_from_slice(&prod.expression);
        for comb in nullable_idxs.iter().combinations(i) {
            let mut new_prod = prod_symbols.clone();
            for idx in comb.into_iter() {
                (new_prod.as_mut_slice())[1..][*idx].value = " ".to_string();
            }

            let new_prod: Vec<(SymbolsKind, String)> = new_prod
                .into_iter()
                .filter(|symbol| symbol.value != String::from(" "))
                .map(|symbol| (symbol.kind, symbol.value))
                .collect();
            new_productions.push(Production::new(new_prod));
        }
    }
}

fn remove_epsilon_productions(prods: &Vec<Production>) -> Vec<Production> {
    prods
        .iter()
        .filter(|prod| {
            let expression_symbols: Vec<String> = prod
                .expression
                .iter()
                .map(|symbol| symbol.value.clone())
                .collect();

            !(expression_symbols.len() == 1
                && expression_symbols.contains(&EPSILON_SYMBOL.to_string()))
        })
        .cloned()
        .collect()
}

pub fn remove_unit_productions(g: &Grammar) -> Grammar {
    let unit_chains = detect_unit_productions(g);

    println!("Unit chains: {:?}", unit_chains);

    let mut new_productions: Vec<Production> = vec![];

    for prod in g.productions.iter() {
        if !is_unit_production(prod) {
            extend_productions(&mut new_productions, &prod, &unit_chains)
        }
    }

    Grammar::new(
        g.non_terms.clone(),
        g.terms.clone(),
        new_productions,
        g.start.clone(),
    )
}

fn extend_productions(
    new_prods: &mut Vec<Production>,
    prod: &Production,
    unit_chains: &HashMap<String, HashSet<String>>,
) {
    let mut _new_prods = vec![];
    //  iterates over each non-term's unit chain to find out
    // if production is an end of the chain
    for (non_term, chain) in unit_chains.into_iter() {
        if non_term != &prod.replaced_symbol.value && chain.contains(&prod.replaced_symbol.value) {
            _new_prods.push(Production {
                replaced_symbol: Symbol {
                    kind: SymbolsKind::NONTERM,
                    value: (*non_term).clone(),
                },
                expression: prod.expression.clone(),
            });
        }
    }

    if !_new_prods.is_empty() {
        new_prods.extend(_new_prods.into_iter());
        return;
    }

    new_prods.push(prod.clone());
}

fn detect_unit_productions(g: &Grammar) -> HashMap<String, HashSet<String>> {
    // Contains unit non-terminals and unit production index
    let mut unit_chains: HashMap<String, HashSet<String>> = HashMap::new();

    // Build non-terms chain for each non-term in G
    for non_term in g.non_terms.iter() {
        let mut old_set: HashSet<String> = HashSet::new();
        old_set.insert((*non_term).clone());

        // Build non-term chain of unit production
        loop {
            let mut new_set: HashSet<String> = HashSet::new();

            for prod in g.productions.iter() {
                if is_unit_production(&prod) && old_set.contains(&prod.replaced_symbol.value) {
                    new_set.insert(prod.expression[0].value.clone());
                }
            }

            new_set = new_set.union(&old_set).cloned().collect();

            if new_set == old_set {
                break;
            }

            old_set = new_set;
        }

        unit_chains.insert((*non_term).clone(), old_set);
    }

    unit_chains
}

fn is_unit_production(prod: &Production) -> bool {
    prod.expression.len() == 1 && prod.expression[0].kind == SymbolsKind::NONTERM
}

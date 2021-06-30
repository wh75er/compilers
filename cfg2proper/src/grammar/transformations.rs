use crate::grammar::{Grammar, Production, Symbol, SymbolsKind, EPSILON_SYMBOL, U_CODEPOINTS};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter;

const MAP_ELEMENT_NOT_FOUND_MSG: &str = "There must be at least one production for requested grammar non-terminal";

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
        new_start = get_new_out_of(&g.start);
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

fn get_new_out_of(s: &String) -> String {
    for codepoint in U_CODEPOINTS.iter() {
        if s.chars().find(|c| c == codepoint).is_none() {
            return s.to_string() + &codepoint.to_string();
        }
    }

    return s.to_string() + &String::from('\'');
}

// Removes immediate left recursion within A productions
// Returns Optional result with substituted symbol A' and
// corresponding productions
fn eliminate_immediate_lr(a_prods: &mut Vec<Production>) -> Option<(String, Vec<Production>)> {
    // Drain left recursive productions out of A productions
    let recursive_prods: Vec<_> = a_prods.drain_filter(|prod|
        prod.replaced_symbol.value == prod.expression[0].value
    ).collect();

    // There is no immediate left recursion
    if recursive_prods.is_empty() {
        return None
    }

    // Create substitution symbol for A
    let a_sub_symbol = get_new_out_of(&recursive_prods[0].replaced_symbol.value);

    // Extend beta productions with substituted A
    // beta1 | beta2 ... -> beta1 | beta2 | beta1 A' | beta2 A'...
    let beta_prods_extension: Vec<Production> = get_content_extended_by_sym(a_prods, &a_sub_symbol);
    a_prods.extend_from_slice(&beta_prods_extension);

    // Rebuild left recursion productions:
    // A alpha1 | A alpha2 ... -> alpha1 | alpha2 | alpha1 A' | alpha2 A'
    let alpha_prods: Vec<Production> = recursive_prods
        .into_iter()
        .map(|mut prod| {
            prod.replaced_symbol.value = a_sub_symbol.to_string();
            prod.expression.remove(0);
            prod
        })
        .collect();
    let alpha_prods_extension: Vec<Production> = get_content_extended_by_sym(&alpha_prods, &a_sub_symbol);

    // Contains A' productions
    let mut sub_prods = vec![];

    sub_prods.extend_from_slice(&alpha_prods);
    sub_prods.extend_from_slice(&alpha_prods_extension);

    Some((a_sub_symbol, sub_prods))
}

fn get_content_extended_by_sym(prods: &Vec<Production>, symbol: &String) -> Vec<Production> {
    let mut prods_extension: Vec<Production> = vec![];
    for prod in prods.iter() {
        let mut extension = prod.clone();
        extension.expression
            .push(Symbol { kind: SymbolsKind::NONTERM, value: symbol.to_string() });
        prods_extension.push(extension);
    }

    prods_extension
}

pub fn eliminate_indirect_lr(g: &Grammar) -> Grammar {
    let mut mapping = map_productions_to_non_term(g);
    let mut new_non_terms: Vec<String> = vec![];

    let map_keys = mapping.keys().cloned().collect::<Vec<String>>();

    for (i, i_value) in map_keys.iter().enumerate() {
        let mut ai_productions = mapping.get(i_value)
            .expect(MAP_ELEMENT_NOT_FOUND_MSG).clone();
        for j_value in map_keys.iter().take(i) {
            let aj_productions = mapping.get(j_value)
                .expect(MAP_ELEMENT_NOT_FOUND_MSG);
            let ai2aj_prods = ai_productions
                .drain_filter(|prod| prod.expression[0].value == *j_value)
                .collect::<Vec<Production>>();
            for mut prod in ai2aj_prods.into_iter() {
                prod.expression.remove(0);
                let extension = extend_from_front(aj_productions, &prod);
                ai_productions.extend_from_slice(&extension);
            }
        }
        let new_non_term = eliminate_immediate_lr(&mut ai_productions);
        match new_non_term {
            Some((new_non_term, prods)) => {
                new_non_terms.push(new_non_term.to_string());
                mapping.insert(new_non_term.to_string(), prods);
            },
            _ => ()
        }

        mapping.insert(i_value.to_string(), ai_productions.clone());
    }

    convert_mapping_to_grammar(&mapping, &g.terms, &g.start)
}

fn convert_mapping_to_grammar(mapping: &HashMap<String, Vec<Production>>, terms: &HashSet<String>, start: &String) -> Grammar {
    let mut non_terms: HashSet<String> = HashSet::new();
    let mut productions: Vec<Production> = vec![];
    for (non_term, prods) in mapping {
        non_terms.insert(non_term.to_string());
        productions.extend_from_slice(prods);
    }

    Grammar::new(non_terms, terms.clone(), productions, start.clone())
}

// Inserting l production to the front of all r productions
// A -> l and B -> r1 | r2 | r3
// A -> l r1 | l r2 | l r3
fn extend_from_front(l: &Vec<Production>, r: &Production) -> Vec<Production> {
    let mut extension: Vec<Production> = vec![];
    for prod in l {
        let extended_prod: Vec<Symbol> = vec![]
            .iter()
            .chain(prod.expression.iter())
            .chain(r.expression.iter())
            .cloned()
            .collect();
        extension.push(Production {
            replaced_symbol: r.replaced_symbol.clone(),
            expression: extended_prod,
        });
    }

    extension
}

fn map_productions_to_non_term(g: &Grammar) -> HashMap<String, Vec<Production>> {
    let mut h: HashMap<String, Vec<_>> = HashMap::new();

    for non_term in g.non_terms.iter() {
        let prods = g.productions
            .iter()
            .filter(|prod| prod.replaced_symbol.value == *non_term)
            .cloned()
            .collect::<Vec<_>>();
        h.insert(non_term.to_string(), prods);
    }

    h
}
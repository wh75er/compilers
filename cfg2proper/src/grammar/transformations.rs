use crate::grammar::{ Grammar,
                      Symbol,
                      Production,
                      SymbolsKind,
                      EPSILON_SYMBOL,
                      NEW_START };
use std::collections::HashSet;
use itertools::Itertools;

pub fn get_epsilon_nonterms(g: &Grammar) -> HashSet<String> {
    let mut old_set: HashSet<String> = HashSet::new();

    loop {
        let mut new_set: HashSet<String> = HashSet::new();

        let mut union: HashSet<String> = HashSet::new();
        union.extend(old_set.iter().cloned());
        union.extend([EPSILON_SYMBOL.to_string()].iter().cloned());

        g.productions.iter().cloned().for_each(|prod| {
            let left = prod.replaced_symbol;
            let right = prod.expression
                .into_iter()
                .map(|symbol| symbol.value)
                .collect::<HashSet<String>>();

            if right.is_subset(&union) {
                new_set.insert(left.value);
            }
        });

        new_set.extend(old_set.iter().cloned());

        if new_set == old_set {
            break;
        }

        old_set = new_set;
    }

    old_set
}

pub fn remove_useless_symbols(g: &Grammar) -> Grammar {
    let n_e = get_epsilon_nonterms(g);
    let intersection:HashSet<String> = g.non_terms
        .intersection(&n_e)
        .cloned()
        .collect();
    let p1: Vec<Production> = g.productions.iter().cloned().filter(|prod| {
        let prod_right_set: HashSet<String> = prod.expression
            .iter()
            .cloned()
            .map(|v| v.value)
            .collect();

        prod_right_set.is_subset(
            &n_e.intersection(&g.terms).cloned().collect::<HashSet<String>>()
        )
    }).collect();

    Grammar::new(&intersection, &g.terms, &p1, g.start.clone())
}

pub fn remove_unreachable(g: &Grammar) -> Grammar {
    let mut old_set: HashSet<String> = HashSet::new();
    let mut new_set: HashSet<String> = [g.start.clone()].iter().cloned().collect();

    while old_set != new_set {
        old_set = new_set.clone();
        old_set.iter().cloned().for_each(|v| {
            g.productions.iter().for_each(|prod: &Production| {
                if prod.replaced_symbol.value == v {
                    new_set.extend(
                        prod.expression
                            .iter()
                            .cloned()
                            .map(|symbol| symbol.value)
                            .collect::<HashSet<String>>()
                    )
                }
            })
        });
    }

    let new_productions: Vec<Production> = g.productions.iter().filter(|prod| {
        let mut production_symbols: HashSet<String> = HashSet::new();
        production_symbols.insert(prod.replaced_symbol.value.clone());
        production_symbols.extend(
            prod.expression
                .iter()
                .cloned()
                .map(|v| v.value)
                .collect::<HashSet<String>>()
        );

        production_symbols.is_subset(&new_set)
    }).cloned().collect();

    Grammar::new(&new_set, &g.terms, &new_productions, g.start.clone())
}

pub fn to_e_free(g: &Grammar) -> Grammar {
    let n_e = get_epsilon_nonterms(g);

    let mut new_start: String = g.start.clone();

    // Removing epsilon productions
    let productions_without_epsilon = remove_epsilon_productions(&g.productions);

    let mut new_productions: Vec<Production> = vec!();

    // Building new productions with compensated deleted non-terminals
    productions_without_epsilon.iter().for_each(|prod| {
        compensate_epsilon_deletion(&mut new_productions, &prod, &n_e);
    });

    let mut new_non_terms: HashSet<String> = g.non_terms.clone();

    // S' -> S | e
    if n_e.contains(&g.start) {
        new_start = NEW_START.to_string();
        new_productions.push(Production::new(&vec!(
            (SymbolsKind::NONTERM, new_start.clone()), (SymbolsKind::NONTERM, g.start.clone())
        )));
        new_productions.push(Production::new(&vec!(
            (SymbolsKind::NONTERM, new_start.clone()), (SymbolsKind::EPSILON, EPSILON_SYMBOL.to_string())
        )));

        new_non_terms.insert(new_start.clone());
    }

    Grammar::new(&new_non_terms, &g.terms, &new_productions, new_start.into())
}

fn compensate_epsilon_deletion(new_productions: &mut Vec<Production>, prod: &Production, n_e: &HashSet<String>) {
    let nullable_idxs: Vec<usize> = prod.expression
        .iter()
        .cloned()
        .map(|symbol| symbol.value)
        .enumerate()
        .filter(|numeration| n_e.contains(&numeration.1))
        .map(|numeration| numeration.0)
        .collect();

    for i in 1..nullable_idxs.len() {
        let mut new_prod: Vec<Symbol> = vec!(prod.replaced_symbol.clone());
        new_prod.extend_from_slice(&prod.expression);
        for comb in nullable_idxs.iter().cloned().combinations(i) {
            comb.into_iter().for_each(|idx| {
                (new_prod.as_mut_slice())[1..][idx].value = "".to_string();
            })
        }
        let new_prod: Vec<(SymbolsKind, String)> = new_prod
            .into_iter()
            .filter(|symbol| symbol.value != "".to_string())
            .map(|symbol| (symbol.kind, symbol.value))
            .collect();
        new_productions.push(Production::new(&new_prod))
    }
}

fn remove_epsilon_productions(prods: &Vec<Production>) -> Vec<Production> {
    prods.iter().cloned().filter(|prod| {
        let expression_symbols: Vec<String> = prod.expression.iter().cloned().map(|symbol| symbol.value).collect();

        expression_symbols.len() < 2 && expression_symbols.contains(&EPSILON_SYMBOL.to_string())
    }).collect()
}

pub fn remove_unit_productions(g: &Grammar) -> Grammar {
    let unit_chains = detect_unit_productions(g);

    let mut new_productions: Vec<Production> = vec!();

    g.productions.iter().for_each(|prod| {
        if !is_unit_production(prod) {
            extend_productions(&mut new_productions, g, &prod, &unit_chains)
        }
    });

    Grammar::new(&g.non_terms, &g.terms, &new_productions, g.start.to_string())
}

fn extend_productions(new_prods: &mut Vec<Production>, g: &Grammar, prod: &Production, unit_chains: &Vec<HashSet<String>>) {
    //  iterates over each non-term's unit chain to find out
    // if production is an end of the chain
    for (idx, chain) in unit_chains.into_iter().enumerate() {
        if chain.contains(&prod.replaced_symbol.value) {
            new_prods.push(Production {
                replaced_symbol: Symbol {
                    kind: SymbolsKind::NONTERM,
                    value: g.non_terms.iter().cloned().collect::<Vec<String>>()[idx].clone()
                },
                expression: prod.expression.clone()
            });

            return;
        }
    }

    new_prods.push(prod.clone());
}

fn detect_unit_productions(g: &Grammar) -> Vec<HashSet<String>> {
    // Contains unit non-terminals and unit production index
    let mut unit_chains: Vec<HashSet<String>> = vec!();

    // Build non-terms chain for each non-term in G
    for non_term in g.non_terms.iter() {
        let mut old_set: HashSet<String> = HashSet::new();
        old_set.insert(non_term.to_string());

        // Build non-term chain of unit production
        loop {
            let mut new_set: HashSet<String> = HashSet::new();

            for prod in g.productions.iter() {
                if  is_unit_production(&prod) &&
                    old_set.contains(&prod.replaced_symbol.value)
                {
                    new_set.insert(prod.expression[0].value.clone());
                }
            }

            new_set.extend(old_set.iter().cloned());

            if new_set == old_set {
                break;
            }
        }

        unit_chains.push(old_set);
    }

    unit_chains
}

fn is_unit_production(prod: &Production) -> bool {
    prod.expression.len() == 1 &&
    prod.expression[0].kind == SymbolsKind::NONTERM
}

use crate::grammar::{ Grammar,
                      Symbol,
                      Production,
                      SymbolsKind,
                      EPSILON_SYMBOL,
                      NEW_START };
use std::collections::HashSet;
use itertools::Itertools::{ combinations };

pub fn get_epsilon_nonterms(g: &Grammar) -> HashSet<String> {
    let mut old_set: HashSet<String> = HashSet::new();

    loop {
        let mut new_set: HashSet<String> = HashSet::new();

        let mut union: HashSet<String> = HashSet::new();
        union.extend(&old_set)
            .extend(EPSILON_SYMBOL);

        g.productions.for_each(|prod: Production| {
            let left = prod.replaced_symbol;
            let right = prod.expression.into_iter().collect::<HashSet<String>>();

            if right.is_subset(&union) {
                new_set.insert(left.value);
            }
        });

        new_set.extend(&old_set);

        if new_set == old_set {
            break;
        }

        old_set = new_set;
    }

    old_set
}

pub fn remove_useless_symbols(g: &Grammar) -> Grammar {
    let n_e = get_epsilon_nonterms(g);
    let intersection:HashSet<String> = g.non_terms.intersection(&n_e).collect();
    let p1: Vec<Production> = g.productions.iter().filter(|prod| {
        let prod_right_set: HashSet<String> = prod.expression
            .map(|v| v.value)
            .into_iter()
            .collect();

        prod_right_set.is_subset(
            n_e.intersection(&g.terms).collect()
        )
    }).collect();

    Grammar::new(&intersection, &g.terms, &p1, &g.start)
}

pub fn remove_unreachable(g: &Grammar) -> Grammar{
    let mut old_set: HashSet<String> = HashSet::new();
    let mut new_set: HashSet<String> = [g.start.to_string()].iter().collect();

    while old_set != new_set {
        old_set = new_set.clone();
        old_set.iter().for_each(|v| {
            g.productions.iter().for_each(|prod: Production| {
                if prod.replaced_symbol == v {
                    new_set.extend(
                        prod.expression
                            .iter()
                            .collect::<HashSet<String>>()
                    )
                }
            })
        });
    }

    let new_productions: Vec<Production> = g.productions.iter().filter(|prod: Production| {
        let mut production_symbols: HashSet<String> = HashSet::new();
        production_symbols.insert(prod.replaced_symbol.value);
        production_symbols.extend(
            prod.expression
                .iter()
                .map(|v| v.value)
                .collect::<HashSet<String>>()
        );

        production_symbols.is_subset(&new_set)
    }).collect();

    Grammar::new(&new_set, &g.terms, &new_productions, &g.start)
}

pub fn to_e_free(g: &Grammar) -> Grammar {
    let n_e = get_epsilon_nonterms(g);

    let mut new_start = &g.start;

    // Removing epsilon productions
    let productions_without_epsilon = remove_epsilon_productions(&g.productions);

    let mut new_productions: Vec<Production> = vec!();

    // Building new productions with compensated deleted non-terminals
    productions_without_epsilon.iter().for_each(|prod: Production| {
        compensate_epsilon_deletion(&mut new_productions, &prod);
    });

    // S' -> S | e
    if n_e.contains(&g.start) {
        new_start = NEW_START.to_str();
        new_productions.push(Production::new(&vec!(
            (SymbolsKind::NONTERM, new_start.to_string()), (SymbolsKind::NONTERM, g.start.clone())
        )));
        new_productions.push(Production::new(&vec!(
            (SymbolsKind::NONTERM, new_start.to_string()), (SymbolsKind::EPSILON, EPSILON_SYMBOL.to_string())
        )));
    }

    Grammar
}

fn compensate_epsilon_deletion(new_productions: &mut Vec<Production>, prod: &Production) {
    let nullable_idxs: Vec<usize> = prod.expression
        .into_iter()
        .map(|symbol| symbol.value)
        .enumerate()
        .filter(|numeration| n_e.contains(&numeration.1))
        .map(|numeration| numeration.0)
        .collect();

    for i in 1..nullable_idxs.len() {
        let mut new_prod: Vec<Symbol> = vec!(prod.replaced_symbol);
        new_prod.extend_from_slice(&prod.expression);
        for comb in nullable_idxs.into_iter().combinations(i) {
            comb.into_iter().for_each(|idx| {
                &new_prod[1..][idx].value = &String::from("");
            })
        }
        let new_prod: Vec<(SymbolsKind, String)> = new_prod
            .into_iter()
            .filter(|symbol| symbol.value != String::from(""))
            .map(|symbol| (symbol.kind, symbol.value))
            .collect();
        new_productions.push(Production::new(&new_prod))
    }
}

fn remove_epsilon_productions(prods: &Vec<Production>) -> Vec<Production> {
    prods.iter().filter(|prod| {
        let expression_symbols: Vec<String> = prod.expression.iter().map(|symbol| symbol.value);

        expression_symbols.len() < 2 && expression_symbols.contains(EPSILON_SYMBOL.to_str())
    }).collect()
}

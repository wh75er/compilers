use crate::grammar::{ Grammar, Production, EPSILON_SYMBOL };
use std::collections::HashSet;

pub fn get_productive(g: &Grammar) -> HashSet<String> {
    let mut old_set: HashSet<String> = HashSet::new();

    loop {
        let mut new_set: HashSet<String> = HashSet::new();

        let mut union: HashSet<String> = HashSet::new();
        union.extend(&old_set)
            .extend(&g.terms)
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
    let n_e = get_productive(g);
    let intersection:HashSet<String> = g.non_terms.intersection(&n_e).collect();
    let p1: Vec<Production> = g.productions.clone().drain_filter(|prod| {
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

    let new_productions: Vec<Production> = g.productions.clone().drain_filter(|prod: Production| {
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
    let n_e = get_productive(g);

}

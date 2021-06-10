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

pub fn remove_useless_symbols(g: &mut Grammar) {
}

pub fn remove_unreachable(g: &mut Grammar) {
}

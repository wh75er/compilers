use super::Dfa;

use std::collections::{HashMap, HashSet, VecDeque};

pub fn minimize(a: &Dfa) -> Dfa {
    let rev_trans = get_reverse_trans(a);

    #[cfg(debug_assertions)]
    println!("Reverse transitions : {:#?}", rev_trans);

    let reachable = find_reachable(a);

    #[cfg(debug_assertions)]
    println!("Reachable : {:#?}", reachable);

    let marked = build_table(a, &rev_trans);

    #[cfg(debug_assertions)]
    println!("Marked : {:#?}", marked);

    let components = calc_components(a, &reachable, &marked);

    #[cfg(debug_assertions)]
    println!("Components : {:?}", components);

    build_dfa(a, &components)
}

fn build_dfa(a: &Dfa, components: &Vec<i32>) -> Dfa {
    let mut new_dfa = Dfa {
        alphabet: a.alphabet.clone(),
        states: vec![],
        trans: vec![],
        is_terminal: HashSet::new(),
    };

    let mut prev_state = -1;

    components.iter().enumerate().for_each(|(i, v)| {
        if *v != -1 {
            if *v == prev_state {
                new_dfa.states[*v as usize].push(i);
            } else {
                new_dfa.states.push(vec![*v as usize]);

                let mut new_state_trans: Vec<usize> = vec![];
                a.trans[i].iter().for_each(|old_state| {
                    new_state_trans.push(components[*old_state] as usize);
                });

                new_dfa.trans.push(new_state_trans);
                prev_state = *v;
            }
        }
    });

    a.is_terminal.iter().for_each(|v| {
        new_dfa.is_terminal.insert(components[*v] as usize);
    });

    new_dfa
}

fn calc_components(a: &Dfa, reachable: &Vec<bool>, marked: &Vec<Vec<bool>>) -> Vec<i32> {
    let mut components: Vec<i32> = vec![-1; a.states.len() + 1];

    components
        .iter_mut()
        .enumerate()
        .filter(|(i, _)| !marked[0][*i])
        .for_each(|(_, v)| {
            *v = 0;
        });

    let mut components_count: i32 = 0;
    for i in 1..a.states.len() {
        if !reachable[i] {
            continue;
        }

        if components[i] == -1 {
            components_count += 1;
            components[i] = components_count;
            for j in (i + 1)..a.states.len() {
                if !marked[i][j] {
                    components[j] = components_count;
                }
            }
        }
    }

    components
}

fn get_reverse_trans(a: &Dfa) -> HashMap<usize, Vec<Vec<usize>>> {
    let mut rev_trans = HashMap::<usize, Vec<Vec<usize>>>::new();

    for (state, to_state) in a.trans.iter().enumerate() {
        for (i, _) in a.alphabet.iter().enumerate() {
            if let Some(v) = rev_trans.get_mut(&(to_state[i] + 1)) {
                v[i].push(state + 1);
            } else {
                let mut new_vec = vec![vec!(); a.alphabet.len()];
                new_vec[i].push(state + 1);
                rev_trans.insert(to_state[i] + 1, new_vec);
            }
        }
    }

    add_additional_trans(a, &mut rev_trans);

    rev_trans
}

fn add_additional_trans(a: &Dfa, rev_trans: &mut HashMap<usize, Vec<Vec<usize>>>) {
    rev_trans.insert(0, vec![vec!(); a.alphabet.len()]);

    for i in 0..a.states.len() + 1 {
        for (j, _) in a.alphabet.iter().enumerate() {
            let v = rev_trans.get_mut(&0).unwrap();
            v[j].push(i);
        }
    }
}

fn find_reachable(a: &Dfa) -> Vec<bool> {
    let mut reachable: Vec<bool> = vec![false; a.states.len() + 1];
    reachable[0] = true;

    let mut stack: Vec<usize> = vec![];
    let mut visited = HashSet::<usize>::new();

    stack.push(0);

    while let Some(v) = stack.pop() {
        reachable[v + 1] = true;
        visited.insert(v);

        for state in a.trans[v].iter() {
            if !visited.contains(&state) {
                stack.push(*state);
            }
        }
    }

    reachable
}

fn build_table(a: &Dfa, rev_trans: &HashMap<usize, Vec<Vec<usize>>>) -> Vec<Vec<bool>> {
    let mut marked = vec![vec![false; a.states.len() + 1]; a.states.len() + 1];
    let mut queue = VecDeque::<(usize, usize)>::new();

    for i in 0..marked.len() {
        for j in 0..marked.len() {
            if !marked[i][j]
                && a.is_terminal.iter().map(|v| v + 1).any(|v| v == i)
                    != a.is_terminal.iter().map(|v| v + 1).any(|v| v == j)
            {
                marked[i][j] = true;
                marked[j][i] = true;
                queue.push_back((i, j));
            }
        }
    }

    while let Some((u, v)) = queue.pop_front() {
        for (c, _) in a.alphabet.iter().enumerate() {
            let r_vec = rev_trans.get(&u).map(|v| v[c].clone()).unwrap_or_default();
            let s_vec = rev_trans.get(&v).map(|v| v[c].clone()).unwrap_or_default();

            if r_vec.is_empty() || s_vec.is_empty() {
                continue;
            }

            for r in r_vec.iter() {
                for s in s_vec.iter() {
                    if !marked[*r][*s] {
                        marked[*r][*s] = true;
                        marked[*s][*r] = true;
                        queue.push_back((*r, *s));
                    }
                }
            }
        }
    }

    marked
}

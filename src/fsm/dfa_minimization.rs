use super::Dfa;

use std::collections::{HashMap, HashSet, VecDeque};

pub fn minimize(a: &Dfa) -> Dfa {
    let rev_trans = get_reverse_trans(a);

    let reachable = find_reachable(a);

    let marked_states = build_table(a, &rev_trans);

    Dfa{trans: vec!(), alphabet: vec!(), states: vec!(), is_terminal: HashSet::new()}
}

fn get_reverse_trans(a: &Dfa) -> HashMap<usize, Vec<usize>> {
    let mut rev_trans = HashMap::<usize, Vec<usize>>::new();

    for (state, to_state) in a.trans.iter().enumerate() {
        for (i, _) in a.alphabet.iter().enumerate() {
            if let Some(v) = rev_trans.get_mut(&to_state[i]) {
                v[i] = state;
            } else {
                rev_trans.insert(to_state[i], vec![0, a.alphabet.len()]);
            }
        }
    }

    rev_trans
}

fn find_reachable(a: &Dfa) -> Vec<bool> {
    let mut reachable: Vec<bool> = vec![false; a.states.len() + 1];
    reachable[0] = true;

    let mut stack: Vec<usize> = vec!();
    let mut visited = HashSet::<usize>::new();

    stack.push(0);

    while !stack.is_empty() {
        stack.pop().map(|v| {
            reachable[v+1] = true;
            visited.insert(v);

            for state in a.trans[v].iter() {
                if !visited.contains(&state) {
                    stack.push(v);
                }
            }
        });

    }

    reachable
}

fn build_table(a: &Dfa, rev_trans: &HashMap<usize, Vec<usize>>) -> Vec<Vec<bool>> {
    let mut marked = vec![vec![false; a.states.len() + 1]; a.states.len() + 1];
    let mut queue = VecDeque::<(usize, usize)>::new();

    for (i, row) in marked.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            if i != 0 && j != 0 && a.is_terminal.contains(&(i - 1)) != a.is_terminal.contains(&(j - 1)) {
                *value = true;
                queue.push_back((i, j));
            }
        }
    }

    while !queue.is_empty() {
        let (u, v) = queue.pop_front().unwrap();
        for (c, _) in a.alphabet.iter().enumerate() {
            let r = rev_trans.get(&u).map(|v| v[c]);
            let s = rev_trans.get(&v).map(|v| v[c]);
            if r.is_some() && s.is_some() && !marked[r.unwrap()][s.unwrap()] {
                marked[r.unwrap()][s.unwrap()] = true;
                queue.push_back((r.unwrap(), s.unwrap()));
            }
        }
    }

    marked
}

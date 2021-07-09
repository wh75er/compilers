mod syntax_tree;

fn main() {
    let tokenized_expr: Vec<_> = vec![
        "begin", "a", "=", "5", "+", "5", ">", "(", "3", "+", "8", ")", "*", "8", "^", "5", "^", "'", ";",
        "ab", "=", "5", "+", "4", ">", "3", ";",
        "abc", "=", "8", "<", "4", "end"
    ];

    let result = syntax_tree::parser::parse(tokenized_expr).unwrap();

    (*result).render_to("syntax-tree.dot");
}

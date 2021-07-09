mod syntax_tree;

fn main() {
    let tokenized_expr: Vec<_> = vec![
        "5", "+", "5", ">", "(", "3", "+", "8", ")", "*", "8", "^", "5", "^", "'"
    ];

    let result = syntax_tree::parser::parse(tokenized_expr).unwrap();

    println!("{:?}", result);

    (*result).render_to("syntax-tree.dot");
}

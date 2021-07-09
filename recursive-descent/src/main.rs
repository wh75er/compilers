mod syntax_tree;

fn main() {
    let tokenized_expr: Vec<_> = vec![
        "5", "+", "5", ">", "8", "*", "8", "^", "5"
    ];

    let result = syntax_tree::parser::parse(tokenized_expr);

    println!("{:?}", result);
}

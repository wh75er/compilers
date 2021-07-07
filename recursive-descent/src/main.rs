mod syntax_tree;

fn main() {
    let tokenized_expr: Vec<_> = vec![
        "5", "+", "5", ">", "-", "3"
    ];

    let result = syntax_tree::parser::parse(tokenized_expr);
}

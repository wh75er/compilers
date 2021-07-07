# Basic expression parser(recursive-descent)

This program is parsing expressions like *5 + 5 < -2*. Parser is a recursive-descent parser with the following grammar:

```
expr ::= <math-expr> <equality-sign> <math-expr>
math-expr ::= <term> | <add-sign> <term> | <term> <math-expr'> | <add-sign> <term> <math-expr'>
math-expr' ::= <add-sign> <term> | <add-sign> <term> <math-expr'>
term ::= <factor> | <factor> <term'>
term' ::= <multiplier-sign> <factor> | <multiplier-sign> <factor> <term'>
factor ::= <primary-expr> | <primary-expr> <factor'>
factor' ::= '^' <primary-expr> | '^' <primary-expr> <factor'>
primary-expr ::= <number> | <identifier> | '(' <math-expr> ')'
add-sign ::= '+' | '-'
multiplier-sign ::= '*' | '/' | '%'
equality-sign ::= '<' | '<=' | '=' | '>=' | '>' | '<>'
number ::= 123456789
identifier ::= '
```

Input data should be tokenized manually(there's no tokenization, only parsing)

USAGE:

```
    cargo run
```

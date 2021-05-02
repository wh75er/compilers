# Basic regex parser

This program is parsing basic regex and transforming it into Deterministic Finite Automaton. Parser is a recursive-descent parser with the following grammar:

```
regex ::= <concat> '|' <regex> | <concat>
concat ::= <factor> '.' <concat> | <factor>
factor ::= <base> '*' | <base>
base ::= <char> | '\' <char> | '(' regex ')'
```

USAGE:

```
    cargo run "<regex>"
```

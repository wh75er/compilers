#!/bin/bash

dfa=dfa
min_dfa=min_dfa

if [[ -e $dfa.dot ]] && [[ -e $min_dfa.dot ]]; then
    dot -Tpng $dfa.dot -o $dfa.png
    dot -Tpng $min_dfa.dot -o $min_dfa.png
else
    echo "One of the $dfa and $min_dfa files not exist."
fi

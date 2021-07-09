#!/bin/bash

filename=syntax-tree

if [[ -e $filename.dot ]]; then
    dot -Tpng $filename.dot -o $filename.png
else
    echo "file $filename doesn't exist."
fi

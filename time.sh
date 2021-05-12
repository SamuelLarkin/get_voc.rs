#!/bin/bash


readonly corpus=${1:-corpus.small}

~/.cargo/bin/hyperfine \
    --prepare="cat $corpus > /dev/null" \
    "target/release/get_voc 1 $corpus &> /dev/null" \
    "target/release/get_voc 2 $corpus &> /dev/null" \
    "target/release/get_voc 3 $corpus &> /dev/null" \
    "target/release/get_voc 4 $corpus &> /dev/null" \
    "target/release/get_voc 5 $corpus &> /dev/null"

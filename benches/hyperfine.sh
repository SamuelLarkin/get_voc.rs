#!/bin/bash

# corpus.small => /space/project/portage/models/WMT2020/hsb-de/corpora/preprocessing.multilingual/bpe.02k/student.bpe-dropout/train.hsb-de.de


readonly corpus=${1:-corpus.small}

which get_voc
cargo build --release

declare -ar tests=(
   "get_voc -s $corpus &> /dev/null"
   "./get_voc.py < $corpus &> /dev/null"
   "cargo run --release wc1 $corpus &> /dev/null"
   "cargo run --release wc1a $corpus &> /dev/null"
   "cargo run --release wc1b $corpus &> /dev/null"
   "cargo run --release wc2 $corpus &> /dev/null"
   "cargo run --release wc3 $corpus &> /dev/null"
   "cargo run --release wc-f1 $corpus &> /dev/null"
   "cargo run --release wc-f2 $corpus &> /dev/null"
   "cargo run --release wc-f3 $corpus &> /dev/null"
   "cargo run --release wc-f4 $corpus &> /dev/null"
   "cargo run --release wc-f5 $corpus &> /dev/null"
   "cargo run --release wc-f6 $corpus &> /dev/null"
)


function run {
   cat src/main.rs

   hyperfine \
      --shell bash \
      --style full \
      --export-json hyperfine.json \
      --prepare="cat $corpus > /dev/null" \
      "${tests[@]}"
}


run \
| tee \
> hyperfine.results

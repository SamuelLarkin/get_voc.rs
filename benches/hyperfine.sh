#!/bin/bash

# corpus.small => /space/project/portage/models/WMT2020/hsb-de/corpora/preprocessing.multilingual/bpe.02k/student.bpe-dropout/train.hsb-de.de


readonly corpus=${1:-corpus.small}

which get_voc

#readonly get_voc_rs="cargo run --release -- -s"
readonly get_voc_rs="./target/release/get_voc_rs -s"
declare -ar commands=(
   "$get_voc_rs wcff $corpus &> /dev/null"
   "$get_voc_rs wcwf $corpus &> /dev/null"
   "$get_voc_rs wcbwf $corpus &> /dev/null"
   "$get_voc_rs wcfmf $corpus &> /dev/null"
   "$get_voc_rs wcrf $corpus &> /dev/null"
   "$get_voc_rs wc-f1 $corpus &> /dev/null"
   "$get_voc_rs wc-f2 $corpus &> /dev/null"
   "$get_voc_rs wc-f3 $corpus &> /dev/null"
   "$get_voc_rs wc-f4 $corpus &> /dev/null"
   "$get_voc_rs wc-f5 $corpus &> /dev/null"
   "$get_voc_rs wc-f6 $corpus &> /dev/null"
   #"get_voc -s $corpus &> /dev/null"
   #"./get_voc.py < $corpus &> /dev/null"
)


function run {
   cargo build --release

   head -n 123123 src/main.rs src/lib.rs

   # This will be used to validate the output of each command.
   for cmd in wcff wcwf wcbwf wcfmf wcrf wc-f1 wc-f2 wc-f3 wc-f4 wc-f5 wc-f6; do
      echo $cmd >&2
      $get_voc_rs $cmd $corpus > wc.get_voc_rs.$cmd &
   done
   get_voc -s $corpus > wc.get_voc &
   ./get_voc.py < $corpus > wc.get_voc.py &
   wait

   hyperfine \
      --setup="cargo build --release" \
      --shell bash \
      --export-json hyperfine.json \
      --prepare="cat $corpus > /dev/null" \
      --style full \
      "${commands[@]}"
}


run \
| tee \
> hyperfine.results

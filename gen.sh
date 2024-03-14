#!/usr/bin/env bash

for i in $(seq 2 1000); do
  bnfgen --start "program" "$1" --max-nonproductive-reductions 10 --separator='' > test-files/term/$i.term
done

#!/usr/bin/env bash

for i in $(seq 1 25); do
  # bnfgen --start "program" "$1" --max-nonproductive-reductions 10 --separator='' > test-files/term/$i.term
  # bnfgen --start "program" "$1" --max-nonproductive-reductions 10 --separator='' > test-files/factor/$i.factor
  # bnfgen --start "program" "$1" --max-nonproductive-reductions 10 --separator='' > test-files/grouping/$i.grouping
  # bnfgen --start "program" "$1" --max-nonproductive-reductions 10 --separator='' > test-files/unary/$i.unary
  bnfgen --start "program" "$1" --max-nonproductive-reductions 10 --separator='' > test-files/relational/$i.relational
done

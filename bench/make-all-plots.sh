#! /bin/bash

set -eu

cd results

IMPLEMENTATIONS="csharp chi gin gorilla actix hyper"

for impl in $IMPLEMENTATIONS
do
  ../make-plot.R $impl true
  ../make-plot.R $impl false
done

cd ..

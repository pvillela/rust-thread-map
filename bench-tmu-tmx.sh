#!/bin/bash

export RUSTFLAGS="-Awarnings"

# $1: number of repetitions 

echo "----- Started: `date +"%Y-%m-%d at %H:%M:%S"` -----"
echo

for ((i=1; i<=$1; i++)); do
    echo "*** i=$i ***" | tee /dev/stderr
    cargo bench --bench tmu_tmx_bench --target-dir target/bench-target
done

echo ""
echo "Finished at: `date +"%H:%M:%S"`"


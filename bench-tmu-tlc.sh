#!/bin/bash

export RUSTFLAGS="-Awarnings"

# $1: number of repetitions 

echo "----- ThreadMap to ThreadLocal comparison -- Started: `date +"%Y-%m-%d at %H:%M:%S"` -----"
echo

cargo bench --bench tmu_tlc_bench --target-dir target/bench-target

echo
echo "Finished at: `date +"%H:%M:%S"`"


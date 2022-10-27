#!/bin/bash

for filename in instruction_tests/*.bin; do
    # Baseline
    cargo run $filename false false false
done
# Notes
This document will be used for keeping track of helpful discoveries throughout the project.

## Rust
### Right shifting in Rust
Using >> in an *i32* will result in an arithmetic right shift, and using >> in an *u32* will result in a logical right shift.
### Usizes in Rust
**Be aware** of that Rust will automatically extend to either 32 bits or 64 bits dependent on the current number of bits when working with *usizes*.

## RISC-V
### Upper immediates and most significant bits
When working with the most significant bits of immidiates, remember to include the leading 1s.
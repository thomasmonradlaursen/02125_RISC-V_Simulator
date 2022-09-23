# Notes
This document will be used for keeping track of helpful discoveries throughout the project.

## Rust
### Right shifting in Rust
Using >> in an *i32* will result in an arithmetic right shift, and using >> in an *u32* will result in a logical right shift.
### Usizes in Rust
**Be aware** of that Rust will automatically extend to either 32 bits or 64 bits dependent on the current number of bits when working with *usizes*.
### Stack allocation and unit tests
**Be aware** that the memory and register arrays must be allocted only once on the stack. This can be achieved by mutable borrowing to this single reference. Otherwise, Rust will throw an error stating that *"thread '<[TEST_NAME]>' has overflowed its stack"*. I suppose that this could be true for any thread trying to access more memory on the stack than it is allowed. Converting to heap should not help according to [Stackoverflow](https://stackoverflow.com/questions/28914042/thread-main-has-overflowed-its-stack-in-rust).

## RISC-V
### Upper immediates and most significant bits
When working with the most significant bits of immidiates, remember to include the leading 1s.
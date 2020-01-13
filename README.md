# loop_blocking
Experiments in achieving C performance in Rust when manipulating arrays.

Specifially the idea here is to write, in Rust, the equavalent of the C code shown in this article by Intel
about cache friendly processing of arrays: https://software.intel.com/en-us/articles/loop-optimizations-where-blocks-are-required

## Build Rust

   $ RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo run  --release


## Build C

By way of a performance comparison the C version is included

    gcc -Wall -O3 -o loop_blocking loop_blocking.c










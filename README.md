# `is_even` 4B If Statements

`is-even-4B` is a Rust library inspired by the joke concept presented in the ["4 Billion If Statements"](https://andreasjhkarlsson.github.io//jekyll/update/2023/12/27/4-billion-if-statements.html) blog post. This library, however, takes advantage of Rust's macro system to generate the extensive series of conditional statements at compile-time. Unlike the original Python implementation discussed in the blog, which required users to generate massive C files, `is-even-4B` in Rust handles this process automatically during compilation. This project is mainly for entertainment purposes, playing with the capabilities of Rust macros and benchmarking this unconventional approach.

## Usage

Run the benchmarks with

```shell
cargo bench
```

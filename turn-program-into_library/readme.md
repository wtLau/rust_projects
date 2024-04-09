# The Fern Simulation

This crate is an example to show how to turn a Rust program into a Rust Library.

It's easy 3 steps:

1. Rename the file `src/main.rs` to `src/lib.rs`
2. Add the `pub` keyword
3. Move `main()` to `src/bin/efern.rs`

Ofc, you can always make the program in its own isolated project, by listing the folder name as dependency!


## To Run

`cargo build`

## Documentation

`cargo doc --no-deps --open`

`--no-deps` == only generate doucment for the crate itself, not all the dependency
`--open` == open the doc in browser afterward

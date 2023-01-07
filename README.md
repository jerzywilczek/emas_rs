# emas
An implementation of Evolutionary Multi-Agent Systems function optimization method.

[![Crates.io](https://img.shields.io/crates/v/emas_rs.svg)](https://crates.io/crates/emas_rs)
[![Build Status](https://github.com/jerzywilczek/emas_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/jerzywilczek/emas_rs/actions/workflows/rust.yml?query=branch%3Amaster)

## Usage
```rust
let solution = SystemBuilder::new(conf_functions::predefined::rastrigin::<10>())
    .build(thread_rng())
    .run();

println!(
    "Fitness: {}",
    conf_functions::fitness::rastrigin_fitness::rastrigin(&solution)
);
```
More elaborate configuration examples are located in the `examples` folder.

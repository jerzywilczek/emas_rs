# emas
An implementation of Evolutionary Multi-Agent Systems function optimization method.

## Usage
```rust
let solution = SystemBuilder::new(conf_functions::predefined::rastrigin::<10>())
    .build(thread_rng())
    .run();

println!("Fitness: {}", conf_functions::fitness::rastrigin_fitness::rastrigin(&solution));
```
More elaborate configuration examples are located in the `examples` folder.

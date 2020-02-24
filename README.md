# permu-rs
A collection of utilities for permutations. It contains useful tools to create, manage and experiment with permutations.

![Build status](https://travis-ci.org/mikelma/permu-rs.svg?branch=master)
![](https://github.com/mikelma/permu-rs/workflows/dev-build-test/badge.svg)

## Documentation
You can find the documentation with examples [here](https://docs.rs/permu-rs).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
permu-rs = "0.2.0"
```

Here's a simple example in order to illustrate how to transform populations from one representation space to another and how to learn and sample distributions:
```rust
use permu_rs::permutation::PermuPopulation;
use permu_rs::inversion::InversionPopulation;
use permu_rs::Population;

fn main() {
    let length = 5;     // Length of permutations
    let pop_size = 5;   // Number of individuals in the population

    // Create an identity permutation population
    let identity = PermuPopulation::<u8>::identity(pop_size, length);
    println!("Identity permutation population:\n{}", identity);

    // Initialize an inversion population to hold the inversion vector
    // representation of the population of permutations
    let mut invs = InversionPopulation::<u8>::zeros(pop_size, length-1);

    // Convert the permutation population into its inversion representation
    identity.to_inversion(&mut invs).unwrap();
    println!("Inversion population from permutations:\n{}", invs);

    // Learn a distritibution over the inversion vector population
    let mut distr = invs.learn();
    println!("Distribution of the inversion population:\n{}", distr);

    // Sample the learned distribution creating a new inversion population
    let mut samples = InversionPopulation::<u8>::zeros(pop_size, length-1);
    InversionPopulation::sample(&mut distr, &mut samples).unwrap();
    // Note that the distribution has changed. The distribution was
    // soften inside the sampling procedure.
    println!("Soften distribution of the inversion population:\n{}", distr);
    println!("Sampled solutions from the distribution:\n{}", samples);

    // Create a permutation population to hold the new permutation population
    let mut recovered = PermuPopulation::<u8>::identity(pop_size, length);

    // Convert the sampled inversion vectors to their permutation representation
    samples.to_permus(&mut recovered).unwrap();
    println!("Permutation representation of samples:\n{}", recovered);
}
```

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.


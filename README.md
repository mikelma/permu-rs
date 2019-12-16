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

Here's a simple example that tries to recover the original distribution from a sampled poplation:
```rust
use permu_rs::permutation::PermuPopulation;
use permu_rs::Population;

fn main() {

    let length = 5; // Length of permutations
    let n_samples = 100;

    // Create a population of identity permutations (elements are of 8 bits, so max length is 255)
    let mut identity = PermuPopulation::<u8>::identity(n_samples, length);

    let mut distr = identity.learn(); // Calculate the distribution

    println!("Original distribution:\n{}", distr);

    // Init samples population
    let mut samples = PermuPopulation::<u8>::zeros(n_samples, length);

    // Sample a new population from the original distribution
    match Population::sample(&mut distr, &mut samples) {
        Ok(_) => (),
        Err(e) => panic!("Fatal: {}", e),
    }

    // The distribution is soften before sampling (+1 to every value of 
    // distribution matrix). So, the original distribution is soften now.
    let soften_distr = distr;
    println!("Soften original distribution:\n{}", soften_distr);

    // Samples follow the soften distribution of the original distribution.
    // In consequence, the recovered distribution is compared with the soften 
    // original distribution.
    println!("Recovered distribution:\n{}",samples.learn());

}
```

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.


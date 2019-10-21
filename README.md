# permu-rs
A collection of utilities for permutations. It contains useful tools to create, manage and experiment with permutations.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
permu-rs = "0.1.2"
```
Here's a simple example that tries to recover the original distribution from a sampled poplation:
```rust
    let length = 5;         // Length of permutations
    let n_samples = 100;
    
    // Create a population of identity permutations (elements are of 8 bits, so max length is 255)
    let mut pop : Vec<permu::Permutation<u8>> = vec![];
    (0..n_samples).for_each(|_| {
        pop.push(permu::Permutation::identity(length));
    });
    let pop = permu::PermuPopulation::from__vec(pop);
    
    let mut distr = pop.learn(); // Calculate distribution
        
    println!("Original distribution: ");
    distr.distribution.iter()
        .for_each(|p| println!("{:?}", p));
    
    // Init samples population
    let mut samples = permu::PermuPopulation::<u8>::zeros(n_samples, length);

    // Sample a new population from the original distribution
    match Population::sample(&mut distr, &mut samples) {
        Ok(_) => (),
        Err(e) => panic!("Fatal: {}", e),
    }
    println!("Recovered distribution: ");
    // The recovered distribution will never match exactly the 
    // original one, as the original distribution is soften 
    // (+1 to every value of distribution matrix) before sampling.
    samples.learn().distribution.iter()
        .for_each(|p| println!("{:?}", p));
```

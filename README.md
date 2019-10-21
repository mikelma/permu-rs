# permu-rs
A collection of utilities for permutations. It contains useful tools to create, manage and experiment with permutations.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
permu-rs = "0.1.2"
```

```Getting started
    let length = 5;
    let n_samples = 100;
    
    // Create a population of identity permutations
    let mut pop : Vec<permu::Permutation<u8>> = vec![];
    (0..n_samples).for_each(|_| {
        pop.push(permu::Permutation::identity(length));
    });
    let pop = permu::PermuPopulation::from_vec(pop);
    
    println!("Initial distribution: ");
    let mut distr = pop.learn();
        
    distr.distribution.iter()
        .for_each(|p| println!("{:?}", p));
    
    // Init samples population
    let mut samples = permu::PermuPopulation::<u8>::zeros(n_samples, length);

    // Sample population
    match Population::sample(&mut distr, &mut samples) {
        Ok(_) => (),
        Err(e) => panic!("Fatal: {}", e),
    }
    println!("Recovered distribution: ");
    samples.learn().distribution.iter()
        .for_each(|p| println!("{:?}", p));
```

extern crate permu_rs;
use permu_rs::permutation::{PermuPopulation};
use permu_rs::{Distribution, Population};

#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

fn start_up(size: usize) -> (Distribution,PermuPopulation<u8>) {
    let n_samples = 1;
    let pop = PermuPopulation::<u8>::random(5,size);
    let mut distr = pop.learn();
    (distr,
     PermuPopulation::<u8>::zeros(n_samples,size))
}

fn sample(data: (&mut Distribution, &mut PermuPopulation<u8>)) {
    let (mut distr, mut samples) = data; 
    Population::sample(distr, samples);
}

fn criterion_benchmark(c: &mut Criterion) {
    // SIZE : 50
    let (mut distr, mut zeros) = start_up(50);
    c.bench_function("sampling, size: 50", move |b| b.iter(|| sample(black_box((&mut distr, &mut zeros)))));

    // SIZE : 100
    let (mut distr, mut zeros) = start_up(100);
    c.bench_function("sampling, size: 100", move |b| b.iter(|| sample(black_box((&mut distr, &mut zeros)))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


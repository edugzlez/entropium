use criterion::{Criterion, black_box, criterion_group, criterion_main};
use entropium::{entropy, mutual_information};

fn bench_entropy(c: &mut Criterion) {
    let small: Vec<u32> = (0..1_024).map(|i| i % 251).collect();
    let large: Vec<u32> = (0..65_536).map(|i| i % 251).collect();

    c.bench_function("entropy/sequential/1024", |b| {
        b.iter(|| entropy(black_box(&small)))
    });
    c.bench_function("entropy/sequential/65536", |b| {
        b.iter(|| entropy(black_box(&large)))
    });
}

fn bench_mutual_information(c: &mut Criterion) {
    let x_small: Vec<u32> = (0..1_024).map(|i| i % 251).collect();
    let y_small: Vec<u32> = (0..1_024).map(|i| (i * 3) % 251).collect();
    let x_large: Vec<u32> = (0..65_536).map(|i| i % 251).collect();
    let y_large: Vec<u32> = (0..65_536).map(|i| (i * 3) % 251).collect();

    c.bench_function("mutual-information/sequential/1024", |b| {
        b.iter(|| mutual_information(black_box(&x_small), black_box(&y_small)))
    });
    c.bench_function("mutual-information/sequential/65536", |b| {
        b.iter(|| mutual_information(black_box(&x_large), black_box(&y_large)))
    });
}

criterion_group!(benches, bench_entropy, bench_mutual_information);
criterion_main!(benches);

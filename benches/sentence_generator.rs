use criterion::{criterion_group, criterion_main, Criterion};
use rust_random_text_generation::SentenceGenerator;
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let alice = fs::read_to_string("./books/alice-in-wonderland.txt").unwrap();
    c.bench_function("make sentence generator", move |b| {
        let alice_part = &alice[0..10000]; // It is ~150000 chars

        b.iter(|| {
            let mut sg = SentenceGenerator::new(5);
            sg.add_text(alice_part);
            sg
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const COOKIE: &str = include_str!("cookies.txt");

fn parse(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        b.iter(|| black_box(nescookie::parse(COOKIE).unwrap()))
    });
}

criterion_group!(benches, parse);
criterion_main!(benches);
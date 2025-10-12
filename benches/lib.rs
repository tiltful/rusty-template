use criterion::{Criterion, criterion_group, criterion_main};
use rusty_template::add;
use std::hint::black_box;

fn bench_add(c: &mut Criterion) {
	c.bench_function("add", |b| b.iter(|| add(black_box(2), black_box(2))));
}

criterion_group!(benches, bench_add);
criterion_main!(benches);

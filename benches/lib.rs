{% if crate_type == "lib" %}
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use {{crate_name}}::greeting;

fn bench_greeting(criterion: &mut Criterion) {
    criterion.bench_function("greeting", |bencher| {
        bencher.iter(|| black_box(greeting()))
    });
}

criterion_group!(benches, bench_greeting);
criterion_main!(benches);
{% endif %}

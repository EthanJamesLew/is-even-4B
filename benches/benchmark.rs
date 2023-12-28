use criterion::{black_box, criterion_group, criterion_main, Criterion};
use is_even_macro::is_even;

pub fn criterion_benchmark(c: &mut Criterion) {
    let check_even_macro = is_even!(1_000_000);

    c.bench_function("is_even_macro", |b| {
        b.iter(|| {
            for i in 0..1_000_000 {
                black_box(check_even_macro(i));
            }
        })
    });

    c.bench_function("is_even_baseline", |b| {
        b.iter(|| {
            for i in 0..1_000_000 {
                black_box(i % 2 == 0);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


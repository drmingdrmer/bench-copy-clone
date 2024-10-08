use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct CopyType(u64);

#[allow(dead_code)]
#[derive(Clone)]
struct CloneType(u64);

fn benchmark_copy(c: &mut Criterion) {
    let value = CopyType(42);
    c.bench_function("copy", |b| b.iter(|| {
        let _copied = black_box(value);
    }));
}

fn benchmark_clone(c: &mut Criterion) {
    let value = CloneType(42);
    c.bench_function("clone", |b| b.iter(|| {
        let _cloned = black_box(value.clone());
    }));
}

criterion_group!(benches, benchmark_copy, benchmark_clone);
criterion_main!(benches);

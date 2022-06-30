use boole_lib::{
    adder, adder2, adder_ref, multiplier, multiplier2, multiplier_easy, multiplier_ref,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_adder(c: &mut Criterion) {
    c.bench_function("adder", |b| b.iter(|| adder(black_box(20), black_box(123))));
    c.bench_function("adder2", |b| {
        b.iter(|| adder2(black_box(20), black_box(123)))
    });
    c.bench_function("adder_ref", |b| {
        b.iter(|| adder_ref(black_box(20), black_box(123)))
    });
}

pub fn bench_multiplier(c: &mut Criterion) {
    c.bench_function("multiplier", |b| {
        b.iter(|| multiplier(black_box(20), black_box(123)))
    });
    c.bench_function("multiplier2", |b| {
        b.iter(|| multiplier2(black_box(20), black_box(123)))
    });
    c.bench_function("multiplier_easy", |b| {
        b.iter(|| multiplier_easy(black_box(20), black_box(123)))
    });
    c.bench_function("multiplier_ref", |b| {
        b.iter(|| multiplier_ref(black_box(20), black_box(123)))
    });
}

criterion_group!(benches, bench_adder, bench_multiplier);
criterion_main!(benches);

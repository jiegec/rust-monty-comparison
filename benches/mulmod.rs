use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_bigint::BigUint;
use rust_monty_comparison::*;

fn criterion_benchmark(c: &mut Criterion) {
    // input numbers
    let mut a = BigUint::from_slice(&rust_monty_comparison::N);
    a = a - BigUint::from(1u32);
    let mut b = BigUint::from_slice(&rust_monty_comparison::N);
    b = b - BigUint::from(2u32);
    let m = BigUint::from_slice(&rust_monty_comparison::N);

    c.bench_function("num-bigint", |bench| {
        bench.iter(|| {
            let res = big_uint_mulmod(black_box(&a), black_box(&b), black_box(&m));
            res
        })
    });
    let a_monty = MontyBigNum::from_u32_slice(&a.to_u32_digits());
    let b_monty = MontyBigNum::from_u32_slice(&b.to_u32_digits());
    let r2_mod = MontyBigNum::from_u32_slice(&R2_MOD);
    let one = MontyBigNum::one();
    c.bench_function("4x monty", |bench| {
        bench.iter(|| {
            let a = black_box(a_monty) * r2_mod;
            let b = black_box(b_monty) * r2_mod;
            let res = a * b;
            let res2 = res * one;
            res2
        })
    });

    let a_monty2 = a_monty * r2_mod;
    let b_monty2 = b_monty * r2_mod;
    c.bench_function("2x monty", |bench| {
        bench.iter(|| {
            let res = black_box(a_monty2) * black_box(b_monty2);
            let res2 = res * one;
            res2
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

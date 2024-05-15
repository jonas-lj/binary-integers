use binary_gcd::binary_gcd::extended_euclidean_algorithm;
use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigInt;
use rand::{thread_rng, RngCore};

fn binary_gcd(c: &mut Criterion) {
    let mut rnd = thread_rng();

    let mut a_bytes = vec![0u8; 4096];
    rnd.fill_bytes(&mut a_bytes);
    let a = BigInt::from_bytes_be(num_bigint::Sign::Plus, &a_bytes);

    let mut b_bytes = vec![0u8; 4096];
    rnd.fill_bytes(&mut b_bytes);
    let b = BigInt::from_bytes_be(num_bigint::Sign::Plus, &b_bytes);

    let a_prime = binary_gcd::integers::Integer::from(&a);
    let b_prime = binary_gcd::integers::Integer::from(&b);

    c.bench_function("binary integers", |b| {
        b.iter(|| extended_euclidean_algorithm::<binary_gcd::integers::Integer>(&a_prime, &b_prime))
    });

    c.bench_function("num-bigint binary", |bench| {
        bench.iter(|| extended_euclidean_algorithm::<BigInt>(&a, &b))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = binary_gcd,
}

criterion_main!(benches);

#![feature(random)]

use std::{hint::black_box, random::random};

use hashstr::sha3::Sha3_512Str;
use sha3::{Digest, Sha3_512};

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

fn random_bytes_array<const N: usize>() -> [u8; N] {
    let mut array = [0u8; N];
    for i in 0..N {
        array[i] = random(..);
    }
    array
}

fn digest_bytes(bytes: &[u8]) -> [u8; 64] {
    let data: [u8; 64] = Sha3_512::digest(bytes).into();
    data
}

fn encode_digest_hex(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_digest_hex");
    for _ in 1..10 {
        let bytes: [u8; 5] = random_bytes_array();
        let data = digest_bytes(&bytes);

        group.throughput(Throughput::Bytes(data.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{bytes:?}")),
            &data,
            |bench, &data| {
                bench.iter(|| black_box(Sha3_512Str::encode(black_box(&data))).unwrap());
            },
        );
    }
    group.finish();
}

criterion_group!(benches, encode_digest_hex);
criterion_main!(benches);

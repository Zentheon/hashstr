#![feature(random)]

use std::{hint::black_box, random::random};

use hashstr::sha3::Sha3_512Str;
use sha3::{Digest, Sha3_512};

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

fn digest_byte(byte: u8) -> [u8; 64] {
    let data: [u8; 64] = Sha3_512::digest([byte]).into();
    data
}

fn encode_hex(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_hex");
    for i in 1..10 {
        let byte: u8 = random(..);
        let data = digest_byte(10);

        group.throughput(Throughput::Bytes(data.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(byte), &data, |bench, &data| {
            bench.iter(|| black_box(Sha3_512Str::encode(black_box(&data))).unwrap());
        });
    }
    group.finish();
}

criterion_group!(benches, encode_hex);
criterion_main!(benches);

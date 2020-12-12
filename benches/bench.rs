use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("crcx_crc32", |b| {
        b.iter(|| crcx::crc32(&black_box(b"123456789")[..]))
    });

    c.bench_function("crc_crc32", |b| {
        b.iter(|| crc::crc32::checksum_ieee(&black_box(b"123456789")[..]))
    });

    c.bench_function("crc_any_crc32", |b| {
        b.iter(|| {
            let mut c = crc_any::CRCu32::crc32();
            c.digest(&black_box(b"123456789")[..])
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

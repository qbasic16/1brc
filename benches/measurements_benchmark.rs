use criterion::{black_box, criterion_group, criterion_main, Criterion};
use onebrc::process_measurements;

pub fn criterion_benchmark(c: &mut Criterion) {
    let path = "measurements_100k.txt".to_owned();
    c.bench_function("process_measurements 100k", |b| {
        b.iter(|| process_measurements(black_box(path.clone()), true))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

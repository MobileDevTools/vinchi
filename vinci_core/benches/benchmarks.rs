use criterion::{Criterion, criterion_group, criterion_main};
use criterion::measurement::WallTime;
use std::time::Duration;
use vinci_core::async_walk_and_find_all_files;

fn bench() {
    async_std::task::block_on(async {
        let result = async_walk_and_find_all_files("/").await;
     println!("{:?}", result);
      //  assert_eq!("", result.unwrap().first().unwrap())
    });
//    let count = walkdir::WalkDir::new("/")
//        .into_iter().count();
//      println!("{:?}", count);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("async_walk_and_find_all_files", |b| b.iter(|| bench()));
}

fn alternate_measurement() -> Criterion<WallTime> {
    Criterion::default().sample_size(10).nresamples(10).measurement_time(Duration::new(180, 00))
}

criterion_group! {
    name = benches;
    config = alternate_measurement();
    targets = criterion_benchmark
}

criterion_main!(benches);
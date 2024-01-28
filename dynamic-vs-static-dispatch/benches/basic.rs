use criterion::{criterion_group, criterion_main, Criterion, Bencher};
use dynamic_vs_static_dispatch::command::benchmark::{Benchmark, Solution};
use dynamic_vs_static_dispatch::command::dynamic_dispatch::DynamicSolution;

pub fn criterion_benchmark(c: &mut Criterion) {
    for size in [1, 10, 100, 1000, 10000, 100000, 1000000] {
        let bench = Benchmark::generate_random(size);
        let dynamic_solution = DynamicSolution::new(bench.command_list.clone());

        c.bench_function(&format!("Dynamic Solution w/ {} commands", size), |b: &mut Bencher| {
            b.iter(|| {
                let _output = dynamic_solution.run(bench.input);
                //assert_eq!(output, bench.output);
            });
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

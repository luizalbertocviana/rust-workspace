pub use std::{fs::File, io::Write};

use crate::traits::{Benchmark, BenchmarkInfo, Header, ReadBenchmark, ReadingInfo};

fn multiple_benchmark<B, IB>(benchs: IB) -> impl Iterator<Item = BenchmarkInfo<B>>
where
    B: Benchmark,
    IB: Iterator<Item = B>,
{
    benchs.map(|bench| bench.benchmark())
}

pub fn write_multiple_benchmark<B, IB>(benchs: IB, filename: &String)
where
    B: Benchmark,
    IB: Iterator<Item = B>,
{
    let mut file =
        File::create(filename).expect(&format!("error while creating file {}", filename));

    let augmented_header = format!("Time {}\n", B::SolvingInfo::header());

    file.write_all(augmented_header.as_bytes()).expect(&format!(
        "error while writing the header of file {}",
        filename
    ));

    for (duration, info) in multiple_benchmark(benchs) {
        let bench_str = format!("{} {}\n", duration.as_secs_f64(), info);

        file.write_all(bench_str.as_bytes()).expect(&format!(
            "error while writing benchmark line in file {}",
            filename
        ));
    }
}

pub fn perform_multiple_benchmarks<R, IR>(reading_infos: IR, filename: &String)
where
    R: ReadBenchmark,
    IR: Iterator<Item = ReadingInfo<R>>,
{
    let benchs = reading_infos.map(|reading_info| R::read_benchmark(reading_info));

    write_multiple_benchmark(benchs, filename)
}

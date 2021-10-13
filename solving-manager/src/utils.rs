use std::{fs::File, io::Write, rc::Rc};

use crate::{Benchmark, BenchmarkInfo, Header};

fn multiple_benchmark<B, IB>(benchs: IB) -> impl Iterator<Item = BenchmarkInfo<B>>
where
    B: Benchmark,
    IB: Iterator<Item = Rc<B>>,
{
    benchs.map(|bench| bench.benchmark())
}

pub fn write_multiple_benchmark<B, IB>(benchs: IB, filename: &String)
where
    B: Benchmark,
    IB: Iterator<Item = Rc<B>>,
{
    let mut file =
        File::create(filename).expect(&format!("error while creating file {}", filename));

    let augmented_header = format!("Time {}", B::SolvingInfo::header());

    file.write_all(augmented_header.as_bytes()).expect(&format!(
        "error while writing the header of file {}",
        filename
    ));

    for (duration, info) in multiple_benchmark(benchs) {
        let bench_str = format!("{} {}", duration.as_secs_f64(), info);

        file.write_all(bench_str.as_bytes()).expect(&format!(
            "error while writing benchmark line in file {}",
            filename
        ));
    }
}

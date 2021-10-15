use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub trait Header {
    fn header() -> String;
}

pub trait Solve {
    type SolvingInfo: Display + Header;

    fn solve(&self) -> Self::SolvingInfo;
}

pub type SolvingInfo<S> = <S as Solve>::SolvingInfo;

pub type BenchmarkInfo<S> = (Duration, SolvingInfo<S>);

pub trait Benchmark: Solve {
    fn benchmark(&self) -> BenchmarkInfo<Self> {
        let starting_instant = Instant::now();

        let info = self.solve();

        let ending_instant = Instant::now();

        (ending_instant.duration_since(starting_instant), info)
    }
}

pub type ReadingInfo<R> = <R as ReadBenchmark>::ReadingInfo;

pub trait ReadBenchmark {
    type Benchmark: Benchmark;
    type ReadingInfo;

    fn read_benchmark(reading_info: Self::ReadingInfo) -> Self::Benchmark;
}

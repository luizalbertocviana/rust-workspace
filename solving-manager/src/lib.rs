pub mod utils;

use std::{fmt::Display, time::{Duration, Instant}};

pub trait Header {
    fn header() -> String;
}

pub trait Solve {
    type SolvingInfo: Display + Header;

    fn solve(&self) -> Self::SolvingInfo;
}

type SolvingInfo<S> = <S as Solve>::SolvingInfo;

type BenchmarkInfo<S> = (Duration, SolvingInfo<S>);

pub trait Benchmark: Solve {
    fn benchmark(&self) -> BenchmarkInfo<Self> {
        let starting_instant = Instant::now();

        let info = self.solve();

        let ending_instant = Instant::now();

        (ending_instant.duration_since(starting_instant), info)
    }
}

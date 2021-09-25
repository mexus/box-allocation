use std::time::Instant;

use box_large_array::*;

use argh::FromArgs;
use rand::{distributions::Standard, Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

/// Test application to compare "naïve" box allocation (with a stack copy) and a
/// maybe-uninit approach.
///
/// When using "naïve", run `ulimit -s 1048600` to increase stack size to 1GB
/// (plus some extra bytes).
#[derive(FromArgs)]
struct Option {
    /// whether or not use naïve algorithm.
    #[argh(switch, long = "naive")]
    naive: bool,

    /// whether or not use "not-that-naïve" algorithm. Overrides the "naive"
    /// flag.
    #[argh(switch, long = "not-that-naive")]
    not_that_naive: bool,
}

fn main() {
    const N: usize = 1024 * 1024 * 1024;

    let Option { naive, not_that_naive } = argh::from_env();
    let data = Xoshiro256PlusPlus::seed_from_u64(10).sample_iter(Standard);

    let begin = Instant::now();
    let boxed = if not_that_naive {
        not_that_naïve::<_, N>(data)
    } else if naive {
        naïve::<_, N>(data)
    } else {
        maybe_uninit::<_, N>(data)
    };
    let sum = boxed.iter().copied().map(u64::from).sum::<u64>();
    let elapsed = begin.elapsed();
    println!("sum = {}", sum);
    println!("Calculated in {:?}", elapsed);
}

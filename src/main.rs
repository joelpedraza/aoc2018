#![cfg_attr(feature = "unstable", feature(test))]

extern crate bit_set;

mod bitvec;
mod trie;
mod math;

mod problem01;
mod problem02;
mod problem03;
mod problem04;
mod problem05;
mod problem06;

fn main() {
    do_problem(problem01::part1);
    do_problem(problem01::part2);

    do_problem(problem02::part1);
    do_problem(problem02::part2);

    do_problem(problem03::part1);
    do_problem(problem03::part2);

    do_problem(problem04::part1);
    do_problem(problem04::part2);

    do_problem(problem05::part1);
    do_problem(problem05::part2);

    do_problem(problem06::part1);
    do_problem(problem06::part2);
}

fn time_it(f: fn() -> ()) {
    use std::time::Instant;

    let now = Instant::now();

    f();

    let elapsed = now.elapsed();

    let nanos = elapsed.as_secs() * 1_000_000_000u64 + elapsed.subsec_nanos() as u64;
    let millis = nanos as f64 / 1_000f64;

    println!("{: >20.04}ms", millis);

}

fn do_problem(f: fn() -> ()) {
    time_it(f)
}
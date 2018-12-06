const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    println!("Problem 5, Part 1: {}", "???")
}

pub fn part2() {
    println!("Problem 5, Part 2: {}", "???")
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample.txt");

    #[test]
    fn part_1_sample_is_correct() {
        assert_eq!(10, length_of_polymer_after_reaction(SAMPLE_INPUT))
    }

    #[test]
    fn part_2_sample_is_correct() {
        assert_eq!(4, length_of_best_polymer(SAMPLE_INPUT))
    }
}


#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use self::test::Bencher;

    #[bench]
    fn react_bench(b: &mut Bencher) {
        b.iter(|| {});
    }

}
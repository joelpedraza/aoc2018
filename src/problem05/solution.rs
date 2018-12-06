use ::math::SubAbs;
use super::input::POLYMER;

pub fn part1() {
    println!("Problem 5, Part 1: {}", length_of_polymer_after_reaction(POLYMER))
}

pub fn part2() {
    println!("Problem 5, Part 2: {}", length_of_best_polymer(POLYMER))
}

fn react(polymer: impl Iterator<Item = u8>) -> String {
    let mut stack: Vec<u8> = Vec::new();
    for c in polymer {
        match stack.last() {
            None => stack.push(c),
            Some(&d) => if has_reaction(c, d) {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
    }
    String::from_utf8(stack).expect("output polymer was not valid utf8")
}

fn length_of_polymer_after_reaction(polymer: &str) -> usize {
    react(polymer.bytes()).len()
}

const ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwzyz";
const ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUBWZYZ";

fn length_of_best_polymer(polymer: &str) -> usize {
    let initial = react(polymer.bytes());

    ALPHA_LOWER.bytes()
        .zip(ALPHA_UPPER.bytes())
        .map(|(lower, upper)| {
            let cand = initial.bytes()
                .filter(|&c| { c != lower && c != upper });
            react(cand).len()
        })
        .min()
        .unwrap()
}

// Helpers
// =======

const REACTION_DIFFERENCE: u8 = 'a' as u8 - 'A' as u8;

fn has_reaction(left: u8, right: u8) -> bool {
    left.sub_abs(right) == REACTION_DIFFERENCE
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::input::POLYMER;

    const SAMPLE_INPUT: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn part_1_sample_is_correct() {
        assert_eq!(10, length_of_polymer_after_reaction(SAMPLE_INPUT))
    }

    #[test]
    fn part_2_sample_is_correct() {
        assert_eq!(4, length_of_best_polymer(SAMPLE_INPUT))
    }

    #[test]
    fn part_1_is_correct() {
        assert_eq!(11242, length_of_polymer_after_reaction(POLYMER))
    }

    #[test]
    fn part_2_is_correct() {
        assert_eq!(5492, length_of_best_polymer(POLYMER))
    }
}


#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use super::super::input::POLYMER;
    use self::test::Bencher;

    #[bench]
    fn react_bench(b: &mut Bencher) {
        b.iter(|| react(POLYMER.bytes()));
    }

}
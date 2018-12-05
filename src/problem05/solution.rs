use super::input::POLYMER;

pub fn part1() {
    println!("Problem 5, Part 1: {}", length_of_polymer_after_reaction(POLYMER))
}

pub fn part2() {
    println!("Problem 5, Part 2: {}", length_of_best_polymer(POLYMER))
}

fn react(polymer: String) -> String {
    let mut polymer = polymer.into_bytes();
    let mut left = 0;

    while left + 1 < polymer.len() {
        if polymer.has_reaction(left, left+1) {
            polymer.remove(left);
            polymer.remove(left);
            left = left.saturating_sub(1);
        } else {
            left += 1;
        }
    }

    String::from_utf8(polymer).expect("output polymer was not valid utf8")
}

fn length_of_polymer_after_reaction(polymer: &str) -> usize {
    react(polymer.to_owned()).len()
}

const ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwzyz";
const ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUBWZYZ";

fn length_of_best_polymer(polymer: &str) -> usize {
    let initial = react(polymer.to_owned());

    ALPHA_LOWER.chars()
        .zip(ALPHA_UPPER.chars())
        .map(|(lower, upper)| {
            let mut cand = initial.clone();
            cand.retain(|c| { c != lower && c != upper });
            react(cand).len()
        })
        .min()
        .unwrap()
}

// Helpers
// =======

fn diff_abs(a: u8, b: u8) -> u8 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

const REACTION_DIFFERENCE: u8 = 'a' as u8 - 'A' as u8;

trait  HasReaction {
    fn has_reaction(&self, left: usize, right: usize) -> bool;
}

impl HasReaction for [u8] {
    fn has_reaction(&self, left: usize, right: usize) -> bool {
        diff_abs(self[left] as u8, self[right] as u8) == REACTION_DIFFERENCE
    }
}

impl HasReaction for str {
    fn has_reaction(&self, left: usize, right: usize) -> bool {
        self.as_bytes().has_reaction(left, right)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn part_1_is_correct() {
        assert_eq!(10, length_of_polymer_after_reaction(SAMPLE_INPUT))
    }

    #[test]
    fn part_2_is_correct() {
        assert_eq!(4, length_of_best_polymer(SAMPLE_INPUT))
    }
}
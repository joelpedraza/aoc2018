pub fn part1() {
    println!("Problem 1, Part 1: {}", solve_part_1(INPUT))
}

pub fn part2() {
    println!("Problem 1, Part 2: {}", solve_part_2(INPUT))
}

pub fn solve_part_1(input: &str) -> isize {
    input.lines()
        .map(|line| {
            line.parse::<isize>().unwrap()
        })
        .sum()
}

pub fn solve_part_2(input: &str) -> isize {
    use std::collections::BTreeSet;
    use std::iter;
    let mut set = BTreeSet::new();

    // 0 is always the initial frequency, so chain the partial sums after 0
    iter::once(0isize).chain(
        input.lines()
            .cycle()
            .map(|line| {
                line.parse::<isize>().unwrap()
            })
            .scan(0isize, |acc, n| {
                *acc += n;
                Some(*acc)
            })
    )
        .find(|freq| {
            !set.insert(*freq)
        })
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_is_correct() {
        test_sample(SAMPLE1, 3, solve_part_1);
        test_sample(SAMPLE2, 3, solve_part_1);
        test_sample(SAMPLE3, 0, solve_part_1);
        test_sample(SAMPLE4, -6, solve_part_1);
    }

    #[test]
    fn part_2_is_correct() {
        test_sample(SAMPLE1, 2, solve_part_2);
        test_sample(SAMPLE5, 0, solve_part_2);
        test_sample(SAMPLE6, 10, solve_part_2);
        test_sample(SAMPLE7, 5, solve_part_2);
        test_sample(SAMPLE8, 14, solve_part_2);
    }

    fn test_sample(sample: &str, expected: isize, f: fn(&str) -> isize) {
        let actual = f(sample);
        assert_eq!(actual, expected);
    }

    const SAMPLE1: &str = "\
+1\n\
-2\n\
+3\n\
+1\
";

    const SAMPLE2: &str = "\
+1\n\
+1\n\
+1\
";

    const SAMPLE3: &str = "\
+1\n\
+1\n\
-2\
";

    const SAMPLE4: &str = "\
-1\n\
-2\n\
-3\
";

    const SAMPLE5: &str = "\
+1\n\
-1\
";

    const SAMPLE6: &str = "\
+3\n\
+3\n\
+4\n\
-2\n\
-4\
";

    const SAMPLE7: &str = "\
-6\n\
+3\n\
+8\n\
+5\n\
-6\
";

    const SAMPLE8: &str = "\
+7\n\
+7\n\
-2\n\
-7\n\
-4\
";
}


// SAMPLE AND ACTUAL INPUTS
// ========================

const INPUT: &str = "\
-1\n\
-17\n\
-4\n\
-15\n\
-1\n\
+6\n\
-17\n\
+3\n\
-14\n\
-9\n\
-5\n\
+16\n\
+10\n\
+15\n\
+13\n\
+15\n\
+9\n\
+15\n\
+18\n\
-14\n\
+2\n\
+4\n\
-13\n\
-1\n\
+9\n\
-11\n\
+18\n\
+18\n\
-7\n\
+13\n\
-9\n\
+2\n\
-16\n\
+2\n\
+10\n\
-11\n\
+8\n\
-5\n\
-8\n\
-17\n\
+15\n\
+4\n\
+15\n\
-6\n\
+21\n\
+6\n\
-9\n\
+16\n\
+2\n\
+5\n\
-6\n\
+13\n\
-15\n\
+7\n\
+4\n\
-14\n\
-18\n\
+1\n\
+7\n\
-21\n\
-3\n\
+8\n\
+10\n\
+12\n\
-15\n\
-3\n\
+16\n\
+8\n\
+13\n\
+10\n\
+8\n\
+1\n\
+18\n\
+2\n\
-13\n\
-11\n\
+7\n\
+13\n\
+14\n\
-3\n\
-1\n\
-4\n\
+10\n\
+11\n\
+8\n\
+11\n\
-13\n\
+14\n\
+14\n\
-5\n\
-12\n\
-19\n\
-5\n\
+9\n\
+2\n\
-3\n\
+2\n\
-7\n\
+13\n\
+16\n\
+17\n\
+16\n\
+5\n\
+17\n\
-9\n\
-5\n\
+6\n\
+17\n\
-10\n\
+5\n\
-20\n\
-3\n\
-10\n\
-10\n\
+19\n\
+17\n\
+5\n\
-14\n\
+15\n\
-13\n\
+15\n\
+19\n\
+7\n\
-13\n\
-2\n\
-18\n\
+12\n\
+3\n\
-2\n\
+17\n\
-11\n\
-16\n\
-12\n\
-15\n\
+3\n\
+18\n\
-16\n\
-15\n\
-17\n\
-16\n\
-20\n\
+22\n\
-3\n\
+6\n\
+12\n\
+8\n\
-15\n\
+3\n\
+15\n\
+13\n\
-3\n\
-18\n\
-8\n\
+17\n\
-3\n\
+13\n\
-20\n\
+2\n\
+15\n\
-2\n\
-12\n\
+15\n\
+26\n\
+12\n\
+1\n\
+15\n\
+18\n\
+3\n\
-4\n\
+17\n\
+5\n\
+2\n\
-3\n\
+7\n\
+3\n\
+17\n\
+1\n\
-15\n\
+9\n\
+10\n\
-14\n\
+3\n\
-19\n\
-11\n\
-2\n\
+9\n\
-13\n\
-10\n\
-1\n\
-9\n\
+11\n\
-9\n\
-6\n\
-2\n\
+18\n\
+5\n\
+6\n\
-18\n\
+14\n\
-6\n\
+16\n\
-7\n\
-1\n\
+19\n\
+9\n\
+2\n\
+13\n\
+17\n\
+7\n\
-19\n\
+18\n\
+9\n\
-2\n\
+3\n\
+6\n\
+16\n\
-6\n\
+7\n\
-15\n\
+17\n\
-8\n\
-14\n\
-7\n\
+5\n\
-7\n\
+13\n\
+17\n\
+3\n\
-13\n\
+17\n\
-19\n\
+8\n\
+19\n\
+1\n\
+6\n\
-11\n\
+8\n\
+15\n\
-11\n\
+7\n\
-16\n\
-1\n\
-9\n\
+8\n\
-4\n\
-3\n\
-19\n\
+3\n\
+2\n\
-13\n\
-10\n\
-7\n\
-5\n\
-6\n\
+3\n\
+2\n\
+3\n\
+7\n\
-21\n\
-11\n\
+18\n\
+8\n\
+3\n\
-8\n\
-7\n\
+10\n\
-18\n\
-8\n\
+7\n\
-16\n\
-18\n\
+2\n\
-11\n\
+14\n\
+2\n\
+2\n\
+21\n\
-16\n\
+22\n\
-15\n\
+20\n\
+26\n\
-7\n\
+21\n\
+12\n\
+21\n\
-9\n\
-7\n\
-15\n\
-18\n\
+5\n\
-11\n\
-26\n\
+15\n\
+29\n\
-32\n\
-23\n\
+3\n\
-9\n\
+3\n\
-43\n\
-12\n\
-6\n\
-15\n\
-8\n\
-9\n\
-1\n\
-12\n\
-31\n\
-15\n\
+2\n\
-7\n\
+15\n\
+19\n\
-4\n\
+13\n\
+18\n\
-7\n\
-28\n\
-4\n\
+1\n\
-27\n\
-17\n\
+11\n\
-15\n\
-5\n\
+12\n\
+6\n\
+15\n\
+10\n\
-11\n\
-24\n\
+6\n\
-11\n\
+13\n\
+3\n\
+5\n\
-14\n\
-14\n\
-19\n\
-18\n\
+9\n\
-15\n\
+9\n\
+11\n\
+5\n\
+6\n\
+4\n\
+18\n\
-19\n\
+17\n\
-14\n\
+3\n\
+6\n\
-18\n\
+8\n\
+12\n\
-18\n\
-14\n\
-21\n\
+12\n\
-10\n\
-18\n\
+14\n\
+10\n\
+14\n\
+15\n\
-11\n\
+8\n\
+25\n\
+21\n\
-31\n\
-5\n\
-38\n\
-52\n\
+4\n\
-5\n\
+11\n\
+2\n\
-1\n\
-3\n\
-12\n\
+6\n\
-19\n\
-2\n\
+1\n\
-17\n\
+8\n\
-13\n\
-4\n\
-8\n\
+3\n\
-20\n\
+5\n\
-9\n\
+8\n\
-10\n\
+15\n\
+14\n\
+9\n\
-4\n\
-9\n\
-1\n\
+12\n\
-16\n\
-8\n\
+9\n\
+18\n\
+13\n\
+7\n\
-9\n\
+18\n\
-17\n\
-9\n\
+13\n\
+7\n\
+8\n\
+3\n\
-20\n\
+19\n\
-9\n\
-17\n\
-24\n\
+8\n\
+7\n\
+4\n\
+26\n\
-1\n\
-12\n\
+38\n\
-79\n\
-15\n\
+20\n\
-19\n\
+9\n\
-25\n\
+4\n\
+1\n\
+2\n\
+2\n\
-19\n\
+22\n\
-11\n\
+25\n\
-20\n\
+5\n\
-14\n\
-37\n\
+14\n\
+12\n\
+34\n\
+1\n\
+40\n\
+38\n\
-21\n\
-12\n\
+104\n\
+120\n\
+14\n\
+57\n\
-20\n\
-1\n\
-17\n\
+8\n\
+6\n\
-9\n\
-62\n\
+29\n\
+25\n\
-107\n\
+236\n\
+43\n\
+13\n\
+68519\n\
-16\n\
+8\n\
-2\n\
-17\n\
+2\n\
-18\n\
-1\n\
+2\n\
+8\n\
+10\n\
-9\n\
+12\n\
-7\n\
+15\n\
+14\n\
-7\n\
-12\n\
+13\n\
+19\n\
-16\n\
-17\n\
-19\n\
-8\n\
-18\n\
+4\n\
-1\n\
+3\n\
+4\n\
-19\n\
+18\n\
-16\n\
-10\n\
-14\n\
-6\n\
+16\n\
+17\n\
+1\n\
+14\n\
+14\n\
+3\n\
-11\n\
-19\n\
-21\n\
-17\n\
-1\n\
-10\n\
+5\n\
-9\n\
-15\n\
-8\n\
-5\n\
-4\n\
+3\n\
+2\n\
+1\n\
-7\n\
+14\n\
-19\n\
-3\n\
+7\n\
+18\n\
-4\n\
-19\n\
-12\n\
-18\n\
+2\n\
+15\n\
-19\n\
+10\n\
+16\n\
-14\n\
-14\n\
+19\n\
+14\n\
-4\n\
+1\n\
+7\n\
-19\n\
-3\n\
-16\n\
-16\n\
-17\n\
-5\n\
-9\n\
+3\n\
+16\n\
+17\n\
-2\n\
-18\n\
-18\n\
+7\n\
-10\n\
+15\n\
+13\n\
+15\n\
+5\n\
-6\n\
+9\n\
+9\n\
+3\n\
+7\n\
+6\n\
-7\n\
+10\n\
-1\n\
-5\n\
+16\n\
-5\n\
+20\n\
-9\n\
+20\n\
-13\n\
+3\n\
+13\n\
-9\n\
+11\n\
+18\n\
+10\n\
+19\n\
-16\n\
-18\n\
-2\n\
+6\n\
-15\n\
-9\n\
+19\n\
-4\n\
-7\n\
+13\n\
-9\n\
-11\n\
+3\n\
+6\n\
-11\n\
-11\n\
-10\n\
+5\n\
+23\n\
+5\n\
+18\n\
+4\n\
+9\n\
+15\n\
-2\n\
+18\n\
+7\n\
+7\n\
+16\n\
+7\n\
-9\n\
-19\n\
-15\n\
+16\n\
-2\n\
+16\n\
-5\n\
-8\n\
+11\n\
+7\n\
+19\n\
+18\n\
+18\n\
+16\n\
-12\n\
-6\n\
+8\n\
+18\n\
+16\n\
-17\n\
+16\n\
+9\n\
+9\n\
-15\n\
+3\n\
+7\n\
-16\n\
-10\n\
-11\n\
+17\n\
-3\n\
-10\n\
-10\n\
-2\n\
-5\n\
+12\n\
-15\n\
-19\n\
+12\n\
-14\n\
+19\n\
-14\n\
-12\n\
+13\n\
+2\n\
-9\n\
-14\n\
-11\n\
+3\n\
-6\n\
-6\n\
+23\n\
+13\n\
+6\n\
+5\n\
+6\n\
+5\n\
+2\n\
-10\n\
+15\n\
+13\n\
+9\n\
+11\n\
-4\n\
-9\n\
+5\n\
+20\n\
+1\n\
+18\n\
+4\n\
-10\n\
+1\n\
-12\n\
-6\n\
-5\n\
+2\n\
+1\n\
+11\n\
+1\n\
+4\n\
+11\n\
-6\n\
+12\n\
+12\n\
-17\n\
-15\n\
+17\n\
+6\n\
-2\n\
+16\n\
-18\n\
-9\n\
+8\n\
+4\n\
+19\n\
+7\n\
-2\n\
+12\n\
+6\n\
-10\n\
+2\n\
+7\n\
+7\n\
-3\n\
-10\n\
+17\n\
-12\n\
+11\n\
-12\n\
-1\n\
+5\n\
+19\n\
+8\n\
-16\n\
-7\n\
+12\n\
+7\n\
-11\n\
+19\n\
-16\n\
+17\n\
+3\n\
+17\n\
+7\n\
-8\n\
-7\n\
+12\n\
+19\n\
-5\n\
+4\n\
-12\n\
+10\n\
+20\n\
+10\n\
-14\n\
+3\n\
-13\n\
+12\n\
-14\n\
-22\n\
-6\n\
-6\n\
+11\n\
-19\n\
+21\n\
-6\n\
-12\n\
+21\n\
+6\n\
+3\n\
+18\n\
-17\n\
+15\n\
-6\n\
-16\n\
-14\n\
-17\n\
+8\n\
-5\n\
+3\n\
+3\n\
-10\n\
+12\n\
+25\n\
+3\n\
-9\n\
-20\n\
-18\n\
+6\n\
-2\n\
-2\n\
-11\n\
-6\n\
-19\n\
-17\n\
-12\n\
+18\n\
-17\n\
+6\n\
+8\n\
+1\n\
+10\n\
-1\n\
-14\n\
+9\n\
-16\n\
+19\n\
-18\n\
-3\n\
+7\n\
+19\n\
-14\n\
+13\n\
+4\n\
+16\n\
+19\n\
-11\n\
-20\n\
-14\n\
-18\n\
+7\n\
-23\n\
-37\n\
-7\n\
-7\n\
+23\n\
+18\n\
+12\n\
-17\n\
+13\n\
-5\n\
-14\n\
-19\n\
+20\n\
-13\n\
-20\n\
+4\n\
+3\n\
+10\n\
+27\n\
+3\n\
+25\n\
+52\n\
+1\n\
+3\n\
-1\n\
-19\n\
+2\n\
-25\n\
+32\n\
+36\n\
+17\n\
+10\n\
-16\n\
-14\n\
+28\n\
+30\n\
+13\n\
+10\n\
+11\n\
+10\n\
-19\n\
-7\n\
+21\n\
+16\n\
-15\n\
+2\n\
+16\n\
+36\n\
+13\n\
+10\n\
+17\n\
+6\n\
-8\n\
-5\n\
+20\n\
+7\n\
-15\n\
+5\n\
-9\n\
-14\n\
-7\n\
+17\n\
-6\n\
+16\n\
+10\n\
+16\n\
-18\n\
+12\n\
-8\n\
-16\n\
-14\n\
+20\n\
-38\n\
+25\n\
+30\n\
+60\n\
+2\n\
+13\n\
+4\n\
+19\n\
+2\n\
+5\n\
-22\n\
+18\n\
-19\n\
-13\n\
+30\n\
-6\n\
-45\n\
-2\n\
+3\n\
+7\n\
-34\n\
-16\n\
-130\n\
-24\n\
-18\n\
+4\n\
+7\n\
-10\n\
+12\n\
+8\n\
-5\n\
+12\n\
-11\n\
-3\n\
-4\n\
-1\n\
+21\n\
-15\n\
-13\n\
-17\n\
+1\n\
-31\n\
+79\n\
-1\n\
-42\n\
-41\n\
+20\n\
+52\n\
+16\n\
+54\n\
-324\n\
+68055\n\
-4\n\
+2\n\
+21\n\
+14\n\
-15\n\
+11\n\
-19\n\
+11\n\
+13\n\
-3\n\
-18\n\
+1\n\
-2\n\
+13\n\
-17\n\
+20\n\
+19\n\
+8\n\
-20\n\
-18\n\
+13\n\
-15\n\
+1\n\
-22\n\
-3\n\
-5\n\
-4\n\
+3\n\
-1\n\
-17\n\
-21\n\
+12\n\
-15\n\
+28\n\
-136507\
";
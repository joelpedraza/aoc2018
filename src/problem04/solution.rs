use super::input::INPUT;

pub fn part1() {
    println!("Problem 4, Part 1: {}", solve_1(INPUT))
}

pub fn part2() {
    println!("Problem 4, Part 2: {}", solve_2(INPUT))
}

fn solve_1(input: &str) -> usize {
    let events = sort_events(input);
    let sleepiest_guard = find_sleepiest_guard(&events);
    let most_slept_minute = find_most_frequently_slept_minute(&events, sleepiest_guard);
    sleepiest_guard as usize * most_slept_minute as usize
}

fn solve_2(input: &str) -> usize {
    let events = sort_events(input);

    use std::collections::BTreeMap;

    let mut guard_minutes: BTreeMap<u16, [u16; 60]> = BTreeMap::new();
    let mut current_guard_id = 0;
    let mut current_sleep_time = 0;

    let mut most_freq_slept_id = 0;
    let mut most_freq_slept_minute = 0;
    let mut most_freq_slept_freq = 0;

    for event in events {

        match event.as_bytes()[19] as char {
            'G'/*uard #NN begins shift*/ => {
                current_guard_id = parse_id(event);
            }

            'f'/*alls asleep*/ => {
                current_sleep_time = parse_minute(event);
            }

            'w'/*akes up*/ => {
                let wake_time = parse_minute(event);
                let minutes = guard_minutes.entry(current_guard_id).or_insert([0; 60]);
                for i in current_sleep_time..wake_time {
                    let i = i as usize;
                    minutes[i] += 1;

                    let cur_freq = minutes[i];
                    if cur_freq > most_freq_slept_freq {
                        most_freq_slept_freq = cur_freq;
                        most_freq_slept_minute = i;
                        most_freq_slept_id = current_guard_id;
                    }
                }
            }
            _ => panic!()
        }
    }

    most_freq_slept_minute * most_freq_slept_id as usize
}

fn find_sleepiest_guard(events: &[&str]) -> u16 {
    use std::collections::BTreeMap;

    let mut guard_times: BTreeMap<u16, u16> = BTreeMap::new();
    let mut current_guard_id = 0;
    let mut current_sleep_time = 0;

    let mut highest_sleep_id = 0;
    let mut highest_sleep_minutes = 0;

    for event in events {
        match event.as_bytes()[19] as char {
            'G'/*uard #NN begins shift*/ => {
                current_guard_id = parse_id(event);
            }

            'f'/*alls asleep*/ => {
                current_sleep_time = parse_minute(event);
            }

            'w'/*akes up*/ => {
                let elapsed_sleep = parse_minute(event) - current_sleep_time;
                let entry_value = guard_times.entry(current_guard_id).or_insert(0);
                let total_sleep = *entry_value + elapsed_sleep as u16;
                *entry_value = total_sleep;

                if total_sleep > highest_sleep_minutes {
                    highest_sleep_minutes = total_sleep;
                    highest_sleep_id = current_guard_id
                }
            }
            _ => panic!()
        }
    }

    highest_sleep_id
}

fn find_most_frequently_slept_minute(events: &[&str], id: u16) -> u8 {
    let mut current_guard_id = 0;
    let mut current_sleep_time = 0;

    let mut minutes: [u16; 60] = [0; 60];

    for event in events {
        match event.as_bytes()[19] as char {
            'G'/*uard #NN begins shift*/ => {
                current_guard_id = parse_id(event);
            }

            'f'/*alls asleep*/ => {
                if current_guard_id == id {
                    current_sleep_time = parse_minute(event);
                }
            }

            'w'/*akes up*/ => {
                if current_guard_id == id {
                    let wake_time = parse_minute(event);
                    for i in current_sleep_time..wake_time {
                        minutes[i as usize] += 1;
                    }
                }
            }
            _ => panic!()
        }
    }

    minutes.iter().enumerate().max_by_key(|(_, total)| { *total }).unwrap().0 as u8
}

fn parse_id(event: &str) -> u16 {
    event[26..].split_whitespace().next().unwrap().parse().unwrap()
}

fn parse_minute(event: &str) -> u8 {
    event[15..=16].parse().unwrap()
}

fn sort_events(input: &str) -> Vec<&str> {
    let mut vec: Vec<&str> = input.lines().collect();

    vec.sort();

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_is_correct() {
        assert_eq!(240, solve_1(SAMPLE_INPUT))
    }

    #[test]
    fn part_2_is_correct() {
        assert_eq!(4455, solve_2(SAMPLE_INPUT))
    }

    pub const SAMPLE_INPUT: &str = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up\
";
}
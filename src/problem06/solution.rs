use ::math::SubAbs;

const INPUT: &str = include_str!("input.txt");
const INPUT_THRESHOLD: usize = 10_000;

pub fn part1() {
    println!("Problem 6, Part 1: {}", solve_part_1(INPUT))
}

pub fn part2() {
    println!("Problem 6, Part 2: {}", solve_part_2(INPUT, INPUT_THRESHOLD))
}

fn solve_part_1(input: &str) -> usize {
    part_1_bruteforce(&parse_points(input))
}

fn part_1_bruteforce(points: &[Point]) -> usize {
    let bb = bounding_box(points.iter()).expect("failed to create bounding box");

    let w = bb.width();
    let h = bb.height();

    let mut closests: Vec<u16> = vec![0; points.len()];

    for y in 0..h {
        for x in 0..w {
            let cell = Point { x: x + bb.left, y: y + bb.top };

            let mut nearest_idx = 0;
            let mut min_dist = u16::max_value();
            let mut dist_count = u16::max_value();

            for (i, point) in points.iter().enumerate() {
                let dist = cell.manhattan_distance(&point);

                if dist < min_dist {
                    nearest_idx = i;
                    min_dist = dist;
                    dist_count = 0;
                } else if dist == min_dist {
                    dist_count += 1;
                }
            }

            if dist_count == 0 && bb.contains(&points[nearest_idx]) {
                closests[nearest_idx] += 1;
            }
        }
    }

    *closests.iter().max().unwrap_or(&0) as usize
}

fn solve_part_2(input: &str, threshold: usize) -> usize {
    part_2_bruteforce(&parse_points(input), threshold)
}

fn part_2_bruteforce(points: &[Point], threshold: usize) -> usize {
    let threshold = threshold as u16;

    let bb = bounding_box(points.iter()).expect("failed to create bounding box");

    let w = bb.width();
    let h = bb.height();

    let mut safe_count = 0;

    for y in 0..h {
        'cells: for x in 0..w {
            let cell = Point { x: x + bb.left, y: y + bb.top };

            let mut total_distance = 0;

            for point in points.iter() {
                let dist = cell.manhattan_distance(&point);

                total_distance += dist;

                // if the current cell is not within the threshold continue to the next one
                if total_distance >= threshold {
                    continue 'cells;
                }
            }

            safe_count += 1;
        }
    }

    safe_count
}

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }

    fn to_rect(&self) -> Rect {
        Rect {
            left: self.x,
            top: self.y,
            right: self.x,
            bottom: self.y,
        }
    }
}

#[derive(Debug)]
struct Rect {
    left: u16,
    top: u16,
    right: u16,
    bottom: u16,
}

impl Rect {
    fn new() -> Rect {
        Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn width(&self) -> u16 {
        self.right - self.left
    }

    fn height(&self) -> u16 {
        self.bottom - self.top
    }

    fn from_point(point: &Point) -> Rect {
        point.to_rect()
    }

    fn encapsulate(&mut self, point: &Point) {
        if point.x < self.left {
            self.left = point.x
        }

        if point.y < self.top {
            self.top = point.y
        }

        if point.x > self.right {
            self.right = point.x
        }

        if point.y > self.bottom {
            self.bottom = point.y
        }
    }

    fn contains(&self, point: &Point) -> bool {
        if point.x <= self.left {
            return false;
        }

        if point.y <= self.top {
            return false;
        }

        if point.x >= self.right {
            return false;
        }

        if point.y >= self.bottom {
            return false;
        }

        true
    }
}

trait ManhattanDistance {
    type Output;

    fn manhattan_distance(&self, other: &Self) -> Self::Output;
}

impl ManhattanDistance for Point {
    type Output = u16;

    fn manhattan_distance(&self, other: &Self) -> Self::Output {
        self.x.sub_abs(other.x) + self.y.sub_abs(other.y)
    }
}

fn bounding_box<'a>(mut points: impl Iterator<Item=&'a Point>) -> Option<Rect> {
    points.next()
        .map(Point::to_rect)
        .map(|mut bb| {
            for p in points {
                bb.encapsulate(p);
            }
            bb
        })
}

fn parse_point(s: &str) -> Point {
    let mut iter = s.split(", ");
    let x = iter.next().expect("no x value").parse().expect("failed to parse x");
    let y = iter.next().expect("no y value").parse().expect("failed to parse y");
    Point { x, y }
}

fn parse_points(s: &str) -> Vec<Point> {
    s.lines()
        .map(parse_point)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample.txt");
    const SAMPLE_THRESHOLD: usize = 32;

    #[test]
    fn part_1_sample_is_correct() {
        assert_eq!(17, solve_part_1(SAMPLE_INPUT))
    }

    #[test]
    fn part_2_sample_is_correct() {
        assert_eq!(16, solve_part_2(SAMPLE_INPUT, SAMPLE_THRESHOLD))
    }

    #[test]
    fn part_1_is_correct() {
        assert_eq!(5358, solve_part_1(INPUT))
    }

    #[test]
    fn part_2_is_correct() {
        assert_eq!(37093, solve_part_2(INPUT, INPUT_THRESHOLD))
    }
}


#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use self::test::Bencher;

    #[bench]
    fn part_1_bf(b: &mut Bencher) {
        let points = parse_points(INPUT);
        b.iter(|| { part_1_bruteforce(&points) });
    }

    #[bench]
    fn part_2_bf(b: &mut Bencher) {
        let points = parse_points(INPUT);
        b.iter(|| { part_2_bruteforce(&points, INPUT_THRESHOLD) });
    }

}

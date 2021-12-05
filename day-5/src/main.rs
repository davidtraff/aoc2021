use std::collections::HashMap;

fn main() {
    let lines = include_str!("../input")
        .split("\r\n")
        .map(|line| Line::new(line))
        .collect();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<Line>) {
    let points = lines
        .into_iter()
        .filter(|line| line.is_straight())
        .flat_map(|line| line.into_iter().collect::<Vec<_>>())
        .collect();

    let duplicates = count_duplicates(&points);

    println!("Part 1: {}", duplicates);
}

fn part_two(lines: &Vec<Line>) {
    let points = lines
        .into_iter()
        .flat_map(|line| line.into_iter().collect::<Vec<_>>())
        .collect();

    let duplicates = count_duplicates(&points);

    println!("Part 2: {}", duplicates);
}

fn count_duplicates(points: &Vec<Point>) -> usize {
    let mut map = HashMap::new();

    for point in points {
        *map.entry(point).or_insert(0) += 1;
    }

    map.values().filter(|count| **count > 1).count()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(data: &str) -> Self {
        let (x, y) = data.split_once(',').unwrap();

        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(data: &str) -> Self {
        let (start, end) = data.split_once("->").unwrap();

        Self {
            start: Point::new(start.trim()),
            end: Point::new(end.trim()),
        }
    }

    pub fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = Point;

    type IntoIter = LineIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator {
            line: self,
            current: Some(self.start),
        }
    }
}

struct LineIterator<'a> {
    line: &'a Line,
    current: Option<Point>,
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            let end = self.line.end;

            let diff = end - current;

            if diff.x == 0 && diff.y == 0 {
                self.current = None;
                return Some(end);
            }

            let step_x = diff.x.min(1).max(-1);
            let step_y = diff.y.min(1).max(-1);

            let next = Point {
                x: current.x + step_x,
                y: current.y + step_y,
            };

            self.current = Some(next);

            return Some(current);
        }

        return None;
    }
}

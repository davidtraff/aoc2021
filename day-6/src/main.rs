fn main() {
    let input: Vec<_> = include_str!("../input")
        .split(',')
        .map(|num| Fish::new(num.parse().unwrap()))
        .collect();

    part_one(input.clone());
    part_two(input.clone());
}

fn part_one(mut input: Vec<Fish>) {
    for _ in 0..80 {
        let mut newly_spawned = input
            .iter_mut()
            .filter_map(|fish| fish.tick())
            .collect();

        input.append(&mut newly_spawned);
    }

    println!("Part 1: {}", input.len());
}

fn part_two(input: Vec<Fish>) {
    let mut count = input.iter()
        .map(|fish| fish.value() as usize)
        .fold([0usize; 9], |mut acc, fish| {
            acc[fish] += 1;
            acc
        });

    for _ in 0..256 {
        count.rotate_left(1);
        count[6] += count[8];
    }

    let result: usize = count.iter().sum();

    println!("Part 2: {}", result);
}

#[derive(Debug, Clone)]
struct Fish(u8);

impl Fish {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn spawn() -> Self {
        Self(8)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn tick(&mut self) -> Option<Self> {
        match self.0 {
            0 => {
                self.0 = 6;
                Some(Fish::spawn())
            }
            _ => {
                self.0 -= 1;
                None
            }
        }
    }
}

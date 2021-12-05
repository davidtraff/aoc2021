fn main() {
    let input = include_str!("../input");
    let mut input = input.split('\r').map(|line| line.trim());

    let draws: Vec<i32> = input
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    input.next().unwrap(); // Blank line between draws and first board.

    let mut boards = vec![];

    loop {
        if let Some(board) = Board::new(&mut input) {
            boards.push(board);
        } else {
            break;
        }
    }

    part_one(boards.clone(), &draws);
    part_two(boards, &draws);
}

fn part_one(mut boards: Vec<Board>, draws: &Vec<i32>) {
    for draw in draws {
        for board in boards.iter_mut() {
            board.draw(*draw);

            if board.has_won() {
                let result = board.sum() * draw;

                println!("Part 1: Result {}", result);

                return;
            }
        }
    }
}

fn part_two(mut boards: Vec<Board>, draws: &Vec<i32>) {
    let mut winner = None;

    for draw in draws {
        for board in boards.iter_mut() {
            if board.has_won() {
                continue;
            }
            
            board.draw(*draw);

            if board.has_won() {
                winner = Some((board.clone(), draw));
            }
        }
    }

    let (winner, value) = winner.unwrap();
    let result = winner.sum() * value;

    println!("Part 2: Result {}", result);
}

#[derive(Clone)]
pub struct Board {
    values: Vec<Vec<(i32, bool)>>,
}

impl Board {
    pub fn new<'a, T: Iterator<Item = &'a str>>(input: &mut T) -> Option<Self> {
        let mut values = vec![];
        for line in input {
            if line.is_empty() {
                break;
            }
    
            let cols: Vec<(i32, bool)> = line
                .split(' ')
                .filter_map(|c| c.parse().ok())
                .map(|c| (c, false))
                .collect();
    
                values.push(cols);
        }
    
        match values.len() {
            0 => None,
            _ => Some(Self { values }),
        }
    }

    fn draw(&mut self, number: i32) {
        for y in 0..self.values.len() {
            let row = &mut self.values[y];

            for x in 0..row.len() {
                let (value, _) = row[x];

                if value == number {
                    row[x] = (value, true);
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        let has_won = self.values.iter()
            .any(|cols| cols.iter().all(|(_, drawn)| *drawn));

        if has_won {
            return true;
        }

        'outer: for x in 0..self.values[0].len() {
            for y in 0..self.values.len() {
                let (_, drawn) = self.values[y][x];

                if !drawn {
                    continue 'outer;
                }
            }

            return true;
        }

        false
    }

    fn sum(&self) -> i32 {
        self.values.iter()
            .flatten()
            .filter(|(_, drawn)| !drawn)
            .map(|(value, _)| value)
            .sum()
    }
}

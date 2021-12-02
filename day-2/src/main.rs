fn main() {
    let input = include_str!("../input");
    let commands = input
        .split('\n')
        .map(Command::new)
        .collect();

    part_one(&commands);
    part_two(&commands);
}

fn part_one(commands: &Vec<Command>) {
    let mut x = 0;
    let mut y = 0;
    for command in commands {
        match command.cmd_type {
            "forward" => x += command.units,
            "up" => y -= command.units,
            "down" => y += command.units,
            _ => panic!("Invalid command!"),
        }
    }

    println!("X: {}, Y: {}, Result: {}", x, y, x * y);
}

fn part_two(commands: &Vec<Command>) {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for command in commands {
        match command.cmd_type {
            "forward" => {
                x += command.units;
                y += aim * command.units;
            },
            "up" => aim -= command.units,
            "down" => aim += command.units,
            _ => panic!("Invalid command!"),
        }
    }

    println!("X: {}, Y: {}, Aim: {}, Result: {}", x, y, aim, x * y);
}

struct Command<'a> {
    pub cmd_type: &'a str,
    pub units: i32,
}

impl<'a> Command<'a> {
    pub fn new(value: &'a str) -> Self {
        let (cmd_type, units) = value.split_once(' ').expect("Invalid instruction");
        let units: i32 = units.trim().parse().unwrap();

        Self {
            cmd_type,
            units,
        }
    }
}

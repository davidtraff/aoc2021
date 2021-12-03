const COLUMN_LEN: usize = 12;
const MASK: i32 = 0b1111_1111_1111;

fn main() {
    let input = include_str!("../input");

    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let mut data: Vec<_> = input.split('\n').map(|line| line.chars()).collect();
    let row_len = data.len();

    // För varje kolumn, för varje rad, ta nästa char från inre iteratorn.
    let gamma = (0..COLUMN_LEN)
        .map(|_| data.iter_mut().map(|c| c.next().unwrap()).collect())
        .map(|line: Vec<char>| {
            (line.into_iter().filter(|c| *c == '1').count() > row_len / 2) as i32
        })
        .rev()
        .fold(0, |acc, bit| (acc << 1) | bit);

    let epsilon = !gamma & MASK;

    println!(
        "Gamma: {}, Epsilon: {}, Result: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn part_two(input: &str) {
    let data: Vec<Vec<_>> = input
        .split('\n')
        .map(|line| line.trim().chars().collect())
        .collect();

    let oxygen_rating = filter(&data, '1');
    let scrubber_rating = filter(&data, '0');

    println!("Oxygen rating: {}, O2 scrubber rating {}, Result {}", oxygen_rating, scrubber_rating, oxygen_rating * scrubber_rating);
}

fn filter(data: &Vec<Vec<char>>, char: char) -> i32 {
    let mut data = data.to_vec();

    for idx in 0..COLUMN_LEN {
        let ones = data
            .iter()
            .map(|chars| chars[idx])
            .filter(|c| *c == char)
            .count();

        let zeroes = data.len() - ones;

        let keeper = if char == '1' {
            (ones >= zeroes) as u32
        } else {
            (ones > zeroes) as u32
        };

        let keeper = char::from_digit(keeper, 10).unwrap();

        data = data
            .into_iter()
            .filter(|chars| chars[idx] == keeper)
            .collect();

        if data.len() == 1 {
            break;
        }
    }

    data[0].iter()
        .map(|c| (*c == '1') as i32)
        .fold(0, |acc, bit| (acc << 1) | bit)
}

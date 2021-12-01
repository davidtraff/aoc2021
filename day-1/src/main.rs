fn main() {
    let input = include_str!("../input");
    let depths: Vec<i32> = input
        .split('\n')
        .map(|l| l.trim().parse().expect("Invalid number"))
        .collect();

    println!(
        "PART 1: Number of increases: {}",
        windowed_depth_increases(&depths, 1)
    );

    println!(
        "PART 2: Number of windowed increases {}",
        windowed_depth_increases(&depths, 3)
    );
}

fn windowed_depth_increases(depths: &Vec<i32>, size: usize) -> usize {
    let windowed = depths
        .windows(size)
        .into_iter()
        .map(|x| x.iter().sum())
        .collect();

    depth_increases(&windowed)
}

fn depth_increases(depths: &Vec<i32>) -> usize {
    let mut increases = 0usize;
    let mut previous = None;
    for value in depths {
        if let Some(previous) = previous {
            if value > previous {
                increases = increases + 1;
            }
        }

        previous = Some(value);
    }

    increases
}

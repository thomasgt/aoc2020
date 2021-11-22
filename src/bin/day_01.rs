use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time,
};

fn get_two_values_with_sum(values: &[i32], n: i32) -> Option<(i32, i32)> {
    let mut reverse_idx = values.len() - 1;
    for idx in 0..values.len() {
        if reverse_idx == idx {
            break;
        }

        loop {
            let sum = values[idx] + values[reverse_idx];
            if sum == n {
                return Some((values[idx], values[reverse_idx]));
            } else if sum > n {
                if reverse_idx == 0 {
                    return None;
                }
                reverse_idx -= 1;
            } else if sum < n {
                break;
            }
        }
    }

    None
}
fn get_three_values_with_sum(values: &[i32], n: i32) -> Option<(i32, i32, i32)> {
    for idx in 0..values.len() {
        let remaining = n - values[idx];
        if remaining < 0 {
            break;
        }

        let others = get_two_values_with_sum(&values[idx..], remaining);
        match others {
            Some((x, y)) => return Some((values[idx], x, y)),
            None => continue,
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = Path::new(&args[1]);
    let n = args[2]
        .parse::<i32>()
        .expect("failed to parse n as an integer");

    let f = File::open(input_path).expect("failed to open input file");
    let r = BufReader::new(f);

    let mut expenses: Vec<i32> = r
        .lines()
        .map(|x| {
            x.expect("failed to read line from input file")
                .parse::<i32>()
                .expect("failed to parse line")
        })
        .collect();

    let stopwatch = time::Instant::now();

    expenses.sort_unstable();

    let pair = get_two_values_with_sum(&expenses, n);
    let triplet = get_three_values_with_sum(&expenses, n);

    let elapsed = stopwatch.elapsed();

    match pair {
        Some((x, y)) => println!("{} x {} = {}", x, y, x * y),
        None => println!("No two expenses that match criteria found"),
    }

    match triplet {
        Some((x, y, z)) => println!("{} x {} x {} = {}", x, y, z, x * y * z),
        None => println!("No three expenses that match criteria found"),
    }

    println!("Took {}s", elapsed.as_secs_f64());
}

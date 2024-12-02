fn part1() {
    let data = "
    22 25 27 28 30 31 32 29
    72 74 75 77 80 81 81
    93 91 90 89 86 85 82 80
    ...
    ";

    let mut safe_reports = 0;

    for line in data.lines() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Split the line into numbers, parse them, and collect into a vector
        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|num| num.parse::<i32>().unwrap()) // Parse each part to i32
            .collect(); // Collect into a Vec

        // Check if levels are all increasing or all decreasing
        let all_increasing = numbers.windows(2).all(|pair| pair[1] > pair[0]);
        let all_decreasing = numbers.windows(2).all(|pair| pair[1] < pair[0]);

        // Check if adjacent levels differ by at least one and at most three
        let valid_differences = numbers
            .windows(2)
            .all(|pair| (1..=3).contains(&(pair[1] - pair[0]).abs()));

        if (all_increasing || all_decreasing) && valid_differences {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {}", safe_reports);
} 

fn part2() {
   let data = "
    22 25 27 28 30 31 32 29
    72 74 75 77 80 81 81
    93 91 90 89 86 85 82 80
    ...
    ";

    let mut safe_reports = 0;

    for line in data.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // Parse the line into numbers
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        if is_safe(&numbers) || can_be_safe_with_one_removed(&numbers) {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {}", safe_reports);
}

/// Checks if a sequence is safe according to the rules
fn is_safe(numbers: &[i32]) -> bool {
    let all_increasing = numbers.windows(2).all(|pair| pair[1] > pair[0]);
    let all_decreasing = numbers.windows(2).all(|pair| pair[1] < pair[0]);
    let valid_differences = numbers
        .windows(2)
        .all(|pair| (1..=3).contains(&(pair[1] - pair[0]).abs()));

    (all_increasing || all_decreasing) && valid_differences
}

/// Checks if removing a single level makes the sequence safe
fn can_be_safe_with_one_removed(numbers: &[i32]) -> bool {
    for i in 0..numbers.len() {
        // Create a new sequence with the ith level removed
        let mut modified_numbers = numbers.to_vec();
        modified_numbers.remove(i);

        // Check if the modified sequence is safe
        if is_safe(&modified_numbers) {
            return true;
        }
    }
    false
}


fn main() {
    part1();
    part2();
}

fn part1() {
     // Define the data as a multiline string
    let data = "
    98415   86712
    21839   96206
    ...
    ";

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in data.lines() {
        // Trim whitespace and split the line into parts
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() == 2 {
            // Parse the left number
            let l: i64 = parts[0]
                .parse()
                .expect("Failed to parse the left number");
            // Parse the right number
            let r: i64 = parts[1]
                .parse()
                .expect("Failed to parse the right number");
            // Add numbers to the respective vectors
            left.push(l);
            right.push(r);
        } else {
            // Handle lines that don't have exactly two parts
            println!("Skipping line: '{}'", line);
        }
    }

    let mut diff = Vec::new();

    left.sort();
    right.sort(); 

    for (l, r) in left.iter().zip(right.iter()) {
        // Cast the values to i64 to allow subtraction and abs()
        let difference = ((*l as i64) - (*r as i64)).abs(); 
        diff.push(difference);
    }

    let value: i64 = diff.iter().sum();
    println!("value: {}", value);

}

fn part2() {
    // Define the data as a multiline string
    let data = "
    98415   86712
    21839   96206
    ...
    ";

    let mut left = Vec::new();
    let mut right = Vec::new();

    // Process each line
    for line in data.lines() {
        // Trim whitespace and split the line into parts
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() == 2 {
            // Parse the left number
            let l: i64 = parts[0]
                .parse()
                .expect("Failed to parse the left number");
            // Parse the right number
            let r: i64 = parts[1]
                .parse()
                .expect("Failed to parse the right number");
            // Add numbers to the respective vectors
            left.push(l);
            right.push(r);
        } else {
            // Handle lines that don't have exactly two parts
            println!("Skipping line: '{}'", line);
        }
    }

    let mut value = 0;

    for i in left.iter() {
        let mut occurrence = 0;

        for x in right.iter() {
            if i == x {
                occurrence += 1;
            }
        }

        let occval = i * occurrence;
        value += occval;
    }

    println!("value {}", value);

}



fn main() {
    part1();
    part2();
}

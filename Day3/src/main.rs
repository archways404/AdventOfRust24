use regex::Regex;

fn part1() {
    let data = "
    ]select(23,564)/$!where()>%mul(747,16)*why()mul(354,748)how()<?mul(29,805)where()mul(480,119)!,why()mul(685,393)(~'&[what()what()mul(376,146)-,<)do()^(mul(735,916)/~~,] what()where()mul(321,623)select()$#what() %#who()<*mul(363,643)where()[mul(360,266),:do()'mul(95,167)who()-select()@[{,)$select()mul(802,119) how()^: {from()mul(147,169)*select())^mul(488,194)$?when()mul(540,154)
    ";

    // Define the regex pattern with capture groups for the two numbers
    let pattern = r"mul\((\d+),(\d+)\)";
    
    // Compile the regex
    let re = Regex::new(pattern).unwrap();
    
    let mut total = 0;

    // Iterate through all matches
    for cap in re.captures_iter(data) {
        // Extract the two numbers as strings
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();
        
        // Multiply them and add to the total
        total += num1 * num2;
        
        println!("Multiplying {} and {} gives {}", num1, num2, num1 * num2);
    }

    println!("Total sum of all multiplications: {}", total);
    
} 

fn part2() {
    let data = "
    ]select(23,564)/$!where()>%mul(747,16)*why()mul(354,748)how()<?mul(29,805)where()mul(480,119)!,why()mul(685,393)(~'&[what()what()mul(376,146)-,<)do()^(mul(735,916)/~~,] what()where()mul(321,623)select()$#what() %#who()<*mul(363,643)where()[mul(360,266),:do()'mul(95,167)who()-select()@[{,)$select()mul(802,119) how()^: {from()mul(147,169)*select())^mul(488,194)$?when()mul(540,154)
    ";

    // Regex patterns
    let instruction_pattern = r"(do\(\)|don't\(\)|mul\(\d+,\d+\))";
    let mul_pattern = r"mul\((\d+),(\d+)\)";

    // Compile regexes
    let instruction_re = Regex::new(instruction_pattern).unwrap();
    let mul_re = Regex::new(mul_pattern).unwrap();

    let mut enabled = true; // `mul` starts enabled
    let mut total = 0;

    // Process all instructions in sequence
    for caps in instruction_re.captures_iter(data) {
        let token = &caps[0];
        if token == "do()" {
            enabled = true; // Enable `mul`
        } else if token == "don't()" {
            enabled = false; // Disable `mul`
        } else if enabled {
            // Process valid `mul` instructions only if enabled
            if let Some(mul_caps) = mul_re.captures(token) {
                let num1: i32 = mul_caps[1].parse().unwrap();
                let num2: i32 = mul_caps[2].parse().unwrap();
                total += num1 * num2;
                println!("Multiplying {} and {} gives {}", num1, num2, num1 * num2);
            }
        }
    }

    println!("Total sum of enabled multiplications: {}", total);
    
} 

fn main() {
    part1();
    part2();
}

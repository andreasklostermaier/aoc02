use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    let mut total_score_1 = 0u32;
    let mut total_score_2 = 0u32;

    if let Ok(lines) = read_lines("./data/input.txt") {

        for combination in lines.flatten() {

            // Total score of part 1 
            total_score_1 += match combination.as_str() {
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3 + 0,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 1 + 6,
                "C Y" => 2 + 0,
                "C Z" => 3 + 3,
                _ => panic!("Invalid game combination!"),
            };

            // Total score of part 2 
            total_score_2 += match combination.as_str() {
                "A X" => 3 + 0,
                "A Y" => 1 + 3,
                "A Z" => 2 + 6,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 2 + 0,
                "C Y" => 3 + 3,
                "C Z" => 1 + 6,
                _ => panic!("Invalid game combination!"),
            }
        }
    }

    // Part 1
    println!("Solution A");
    println!("The total score in part one is {}.", total_score_1);

    // Part 2
    println!("Solution A");
    println!("The total score in part two is {}.", total_score_2);
 
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

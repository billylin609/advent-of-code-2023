use std::fs::read_to_string;

fn main() {
    let data_path = "data/day1_exercise1.txt";
    let lines = read_lines(data_path);
    let mut sum = 0;

    for line in lines {
        let mut nums: Vec<i64> = vec!();
        let mut queue_index = 0;
        let mut number = 0;

        if line.trim() == "" {
            continue;
        } else {
            for character in line.chars() {
                if character >= '0' && character <= '9' {
                    let num = character.to_digit(10).unwrap();
                    let num = num as i64;
                    nums.push(num);
                    queue_index += 1;
                }
            }
            if nums.len() == 0 {
                panic!("vec no size");
            }
            number = nums.get(0).unwrap()*10 + nums.get(queue_index-1).unwrap();
            println!("{}", number);
        }
        sum += number;
    }
    println!("sum: {}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

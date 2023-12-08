use std::fs::read_to_string;

struct PosVal {
    pos: u32,
    val: i64,
}

fn main() {
    let data_path = "data/day1_exercise1.txt";
    let lines = read_lines(data_path);
    let mut sum = 0;

    for line in lines {
        let mut nums: Vec<PosVal> = vec!();
        let mut queue_index = 0;
        let mut number = 0;

        if line.trim() == "" {
            continue;
        } else {
            let out = match_indicies_str(&"0", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 1});
                nums.push(PosVal{pos: max, val: 1});
            }

            let out = match_indicies_str(&"1", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 1});
                nums.push(PosVal{pos: max, val: 1});
            }
            
            let out = match_indicies_str(&"2", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 2});
                nums.push(PosVal{pos: max, val: 2});
            }

            let out = match_indicies_str(&"3", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 3});
                nums.push(PosVal{pos: max, val: 3});
            }
            
            let out = match_indicies_str(&"4", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 4});
                nums.push(PosVal{pos: max, val: 4});
            }

            let out = match_indicies_str(&"5", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 5});
                nums.push(PosVal{pos: max, val: 5});
            }

            let out = match_indicies_str(&"6", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 6});
                nums.push(PosVal{pos: max, val: 6});
            }

            let out = match_indicies_str(&"7", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 7});
                nums.push(PosVal{pos: max, val: 7});
            }
            
            let out = match_indicies_str(&"8", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 8});
                nums.push(PosVal{pos: max, val: 8});
            }
        
            let out = match_indicies_str(&"9", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 9});
                nums.push(PosVal{pos: max, val: 9});
            }
            let out = match_indicies_str(&"zero", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 1});
                nums.push(PosVal{pos: max, val: 1});
            }

            let out = match_indicies_str(&"one", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 1});
                nums.push(PosVal{pos: max, val: 1});
            }
            
            let out = match_indicies_str(&"two", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 2});
                nums.push(PosVal{pos: max, val: 2});
            }

            let out = match_indicies_str(&"three", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 3});
                nums.push(PosVal{pos: max, val: 3});
            }
            
            let out = match_indicies_str(&"four", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 4});
                nums.push(PosVal{pos: max, val: 4});
            }

            let out = match_indicies_str(&"five", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 5});
                nums.push(PosVal{pos: max, val: 5});
            }

            let out = match_indicies_str(&"six", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 6});
                nums.push(PosVal{pos: max, val: 6});
            }

            let out = match_indicies_str(&"seven", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 7});
                nums.push(PosVal{pos: max, val: 7});
            }
            
            let out = match_indicies_str(&"eight", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 8});
                nums.push(PosVal{pos: max, val: 8});
            }
        
            let out = match_indicies_str(&"nine", line.clone());
            if out.len() != 0 {
                let mut min = 1000000;
                let mut max = 0; 
                for val in out.iter() {
                    if val <= &min {
                        min = *val;
                    }
                    if val >= &max {
                        max = *val;
                    }
                }
                nums.push(PosVal{pos: min, val: 9});
                nums.push(PosVal{pos: max, val: 9});
            }


            if nums.len() == 0 {
                panic!("vec no size");
            }

            let mut max_val = 0;
            let mut max_pos = 0;

            let mut min_val = 0;
            let mut min_pos = 100000;

            for num in nums.iter() {
                if num.pos <= min_pos {
                    min_val = num.val;
                    min_pos = num.pos;
                }
                if num.pos >= max_pos {
                    max_val = num.val;
                    max_pos = num.pos;
                }
            }
            number = min_val*10 + max_val;
            println!("{}", number);
        }
        sum += number;
    }
    println!("sum: {}", sum);
}

fn match_indicies_str(val: &str, line: String) -> Vec<u32> {
    line.as_str().match_indices(&val).collect::<Vec<_>>().into_iter().map(|x| x.0 as u32).into_iter().collect()
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

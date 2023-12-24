use std::fs::read_to_string;

fn main() {
    let mut lines = read_lines("data/day4_exercise1.txt");

    let mut number_set = number_collection{winning_number: vec!(), my_number:vec!()};
    let mut sum = 0;

    let mut copy_count: Vec<usize> = vec!();

    for line in lines.iter() {
        if line.trim() == "" {
            continue;
        }
        number_set.fs_to_number_collect(&line);
        sum += number_set.compare_match(&mut copy_count);
        number_set = number_collection{winning_number: vec!(), my_number:vec!()};
    }

    println!("{sum}");

}

struct number_collection {
    winning_number: Vec<usize>,
    my_number: Vec<usize>,
}

impl number_collection {
    fn fs_to_number_collect(&mut self, line: &String) {
        let elements = line.split(" ");

        let mut number_collection = elements.collect::<Vec<&str>>();
        
        loop {
            if number_collection[0].ends_with( ":") {
                break;
            }
            number_collection.remove(0);
        }
        number_collection.remove(0);
        
        number_collection.retain(|&x| x != "");

        let mut case = 0;
        //winning case enter
        for number in number_collection.iter()  {
            if case == 0 {
                if *number == "|" {
                    case+=1;
                } else{
                    self.winning_number.push(number.parse::<usize>().unwrap());
                }
            } else {
                self.my_number.push(number.parse::<usize>().unwrap());
            }
        }
    }

    fn compare_match(&self, copy_count: &mut Vec<usize>) -> usize {
        let mut value = 0;
        for number in &self.my_number {
            if self.winning_number.contains(&number) {
                value += 1
            }
        }
        println!("{value}");

        if copy_count.len() < value {
            for number in 0.. value-copy_count.len() {
                copy_count.push(0);
            }
        }
        for number in 0..value {
            copy_count[number] += 1;
        }
        let instance_count = copy_count[0] + 1;
        copy_count.remove(0);
        return instance_count;
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

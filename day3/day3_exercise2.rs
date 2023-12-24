use std::fs::read_to_string;

enum SymbolType{
    null,
    zero,
    one,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    symbol,
    gear,
}

impl SymbolType {
   fn input_filter(var: &char) -> SymbolType {
       match var {
            '.' => SymbolType::null,
            '*' => SymbolType::gear,
            '0' => SymbolType::zero,
            '1' => SymbolType::one,
            '2' => SymbolType::two,
            '3' => SymbolType::three,
            '4' => SymbolType::four,
            '5' => SymbolType::five,
            '6' => SymbolType::six,
            '7' => SymbolType::seven,
            '8' => SymbolType::eight,
            '9' => SymbolType::nine,
            - => SymbolType::symbol,
       }
   }

    fn output_converter(&self) -> usize {
        match self {
            SymbolType::zero => 0,
            SymbolType::one => 1,
            SymbolType::two => 2,
            SymbolType::three => 3,
            SymbolType::four => 4,
            SymbolType::five => 5,
            SymbolType::six => 6,
            SymbolType::seven => 7,
            SymbolType::eight => 8,
            SymbolType::nine => 9,
            SymbolType::gear => 10,
            SymbolType::symbol => 11,
            _ => 12,
        }
    }
}

fn main() {
    let lines = read_lines("data/day3_exercise1.txt");
    let mut str_matrix: Vec<Vec<SymbolType>> = vec!();

    let mut sum = 0;
    
    for line in lines {
        if line.trim() == "" {
            continue;
        }
        
        let mut inner_char: Vec<SymbolType> = vec!();
        column_length = line.len() - 1;

        for character in line.chars() {
            inner_char.push(SymbolType::input_filter(&character));
        }

        str_matrix.push(inner_char);
    }

    for row_index in 0..row_length+1 {
        for column_index in 0..=column_length {
            let value = str_matrix[row_index][column_index].output_converter();
            if value == 10 {
            }
        }
    }

    println!("{}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

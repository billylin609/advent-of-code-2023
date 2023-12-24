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
}

impl SymbolType {
   fn input_filter(var: &char) -> SymbolType {
       let var = *var;
       if var == '.' {
           SymbolType::null
       } else if var == '0' {
           SymbolType::zero
       } else if var == '1' {
           SymbolType::one
       } else if var == '2' {
           SymbolType::two
       } else if var == '3' {
           SymbolType::three
       } else if var == '4' {
           SymbolType::four
       } else if var == '5' {
           SymbolType::five
       } else if var == '6' {
           SymbolType::six
       } else if var == '7' {
           SymbolType::seven
       } else if var == '8' {
           SymbolType::eight
       } else if var == '9' {
           SymbolType::nine
       } else {
           SymbolType::symbol
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
            SymbolType::symbol => 10,
            _ => 11,
        }
    }
}

fn main() {
    let lines = read_lines("data/day3_exercise1.txt");
    let mut str_matrix: Vec<Vec<SymbolType>> = vec!();

    let mut row_length = lines.len() - 1;
    let mut column_length = 0;

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

    struct value_desc {
        number: usize,
        row: usize,
        column_start: usize,
        column_end:usize,
    }

    let mut count = 0;

    for row_index in 0..row_length+1 {
        let mut new_instance = value_desc {number: 0, row: 0, column_start: 0, column_end: 0};

        for column_index in 0..=column_length+1 {
            let value = if column_index == column_length + 1 {11} else {str_matrix[row_index][column_index].output_converter()};
            if value < 10 {
                if new_instance.number == 0 {
                    new_instance.row = row_index;
                    new_instance.column_start = column_index;
                }
                new_instance.number = new_instance.number * 10 + value;
            } else  {
                if new_instance.number != 0 {
                    new_instance.column_end = column_index - 1;

                    //check if true or false
                    //left bound 
                    let row_u = if new_instance.row > 0 {new_instance.row - 1} else {new_instance.row};
                    let row_m = new_instance.row;
                    let row_d = if new_instance.row < row_length {new_instance.row + 1} else {new_instance.row};
                    
                    let column_left = if new_instance.column_start > 0 {new_instance.column_start - 1} else {new_instance.column_start};
                    let column_right = if new_instance.column_end < column_length {new_instance.column_end + 1} else {new_instance.column_end};

                    let mut value_valid = false;

                    for c_index in column_left..column_right + 1 {
                        if str_matrix[row_u][c_index].output_converter() == 10 {
                            value_valid = true;
                            break;
                        }
                        if str_matrix[row_d][c_index].output_converter() == 10 {
                            value_valid = true;
                            break;
                        }
                        if str_matrix[row_m][c_index].output_converter() == 10 {
                            value_valid = true;
                            break;
                        }
                    }

                    if value_valid == true {
                        count += 1;
                        sum += new_instance.number;    
                    }

                    new_instance = value_desc {number: 0, row: 0, column_start: 0, column_end: 0};
                }
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

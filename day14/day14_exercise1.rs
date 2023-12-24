use std::fs::read_to_string;

enum ObjType {
    void,
    rock,
    fixed,
}

impl ObjType {
    fn to_obj(c: &char) -> ObjType {
	    match c {
	        '.' => ObjType::void,
	        'O' => ObjType::rock,
	        '#' => ObjType::fixed,
	        _ => panic!("unrecognized element"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            ObjType::void => '.',
            ObjType::rock => 'O',
            ObjType::fixed => '#',
        }
    }

    fn print_line(row: &Vec<ObjType>) {
        for c in row {
            print!("{}",c.to_char());
        }
        print!("\n");
    }
}

fn main() {
    let lines: Vec<String> = read_lines("data/day14_exercise1.txt");

    let mut platform: Vec<Vec<ObjType>> = vec!();
    let mut row_len: usize = 0;
    let mut column_len: usize = 0;
    
    for line in lines.iter() {
	    if line.trim() != "" {
	        let mut row: Vec<ObjType> = vec!();
            for c in line.chars() {
		        row.push(ObjType::to_obj(&c));
	        }
            row_len = row.len();
	        platform.push(row);
	        row = vec!();
	    }
    }
    column_len = platform.len();

    for row in platform.iter() {
        ObjType::print_line(&row);
    }

    for j in 0..row_len {
        let mut sub_j_start = 0;
        for i in 0..row_len {
            match platform[i][j] {
                ObjType::fixed => {
                    let mut sum = 0;
                    for sub_j in sub_j_start..j {
                        sum += match platform[i][j] {
                            ObjType::rock => 1,  
                        };
                    }
                },
                _ => continue,
            };
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

use std::fs::read_to_string;

struct DataSet {
    cmp_str: String,
    rank: usize,
    order: usize,
    value: usize,
}

fn main() {
    let lines = read_lines("data/day7_exercise1.txt");

    let mut value_set: Vec<DataSet> = vec!();

    for line in lines.iter() {
        let mut data_instance = DataSet::default();
        let parts = line.split(' ');
        data_instance.cmp_str = parts[0];
        data_instance.value = parts[1].parse::<usize>().unwrap();
        let mut max: [usize; 5] = [0; 5];
        for character in  data_instance.cmp_str.chars() {
            let occurance = data_instance.cmp_str.match_indices(character).collect().len();
            max.(occurance-1) += 1;
        }
        for i in 0..5 {
            max[i] = max[i] / (5-i);
        }
    }
}

hasmap trick
m.entry(key:x).or_insert(default:0) += 1;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

impl Default for DataSet {
    fn default() -> Self {
        DataSet {
            cmp_str: String::new(),
            rank: 0,
            order: 0,
            value: 0,
        }
    }
}

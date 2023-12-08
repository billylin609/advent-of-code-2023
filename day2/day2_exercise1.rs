use std::fs::read_to_string;

enum ErrType {
    pass,
    overflow,
    invalid_color,
}

struct GameDetail {
    id: usize,
    status: bool,
    set_count: usize,
    error_code: ErrType,
}

impl Default for GameDetail {
    fn default () -> GameDetail {
        GameDetail {
            id: 0,
            status: false,
            set_count: 0,
            error_code: ErrType::pass,
        }
    }
}

struct Range {
    start: usize,
    end: usize,
}

fn main() {
    let lines = read_lines("data/day2_exercise1.txt");

    let mut sum = 0;

    for line in lines.iter() {
        if line.trim() == "" {
            continue;
        }

        let mut game_instance: GameDetail = GameDetail::default();
        let mut id_sperator = match_indicies_str(":", &line);
        let mut space_sperator = match_indicies_str(" ", &line);
        let mut semi_column_sperator = match_indicies_str(";", &line);
        let end_of_line = line.len()-1;
        semi_column_sperator.push(end_of_line);

        let mut red_indicator = match_indicies_str("red", &line);
        let mut green_indicator = match_indicies_str("green", &line);
        let mut blue_indicator = match_indicies_str("blue", &line);

        let mut index = 0;

        game_instance.id = get_id(&line, space_sperator.get(0).unwrap(), id_sperator.get(0).unwrap());
        game_instance.set_count = semi_column_sperator.len();

        let mut set_range = Range{start: *id_sperator.get(0).unwrap(), end: *id_sperator.get(0).unwrap()};
        for set_num in 0..game_instance.set_count {
            set_range.end = *semi_column_sperator.get(set_num).unwrap();

            if check_color_valid(&red_indicator, &set_range) {
                let end_location = red_indicator.remove(0) - 1;
                
                let mut start_location = 0;
                for i in space_sperator.iter() {
                    if i > &start_location && i < &end_location {
                        start_location = *i;
                    }
                }

                let red = get_id(&line, &start_location, &end_location);
                
                if red > 12 {
                    game_instance.error_code = ErrType::invalid_color;
                }
            }

            if check_color_valid(&green_indicator, &set_range) {
                let end_location = green_indicator.remove(0) - 1;
            
                let mut start_location = 0;
                for i in space_sperator.iter() {
                    if i > &start_location && i < &end_location {
                        start_location = *i;
                    }
                }

                let green = get_id(&line, &start_location, &end_location);
                
                if green > 13 {
                    game_instance.error_code = ErrType::invalid_color;
                }
            }

            if check_color_valid(&blue_indicator, &set_range) {
                let end_location = blue_indicator.remove(0) - 1;
                
                let mut start_location = 0;
                for i in space_sperator.iter() {
                    if i > &start_location && i < &end_location {
                        start_location = *i;
                    }
                }

                let blue = get_id(&line, &start_location, &end_location);

                if blue > 14 {
                    game_instance.error_code = ErrType::invalid_color;
                }
            }

            set_range.start = set_range.end;
        }
        sum += match game_instance.error_code {
            ErrType::pass => game_instance.id,
            _ => 0,
        };

        println!("sum {}", sum);
    }
}

fn check_color_valid(color_list: &Vec<usize>, set_range: &Range) -> bool {
    for color_index in color_list.iter() {
        if color_index >= &set_range.start && color_index <= &set_range.end {
            return true;
        }
    }
    false
}

fn get_id(line: &String, start_index: &usize, end_index: &usize) -> usize {
    let mut iter = line.char_indices();
    let (start, _) = iter.nth(*start_index).unwrap();
    let (end, _) = iter.nth(*end_index - *start_index - 1).unwrap();
    let id = &line[start..end];
    id.trim().parse::<usize>().unwrap()
}

fn match_indicies_str(val: &str, line: &String) -> Vec<usize> {
    line.as_str().match_indices(&val).collect::<Vec<_>>().into_iter().map(|x| x.0 as usize).into_iter().collect()
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

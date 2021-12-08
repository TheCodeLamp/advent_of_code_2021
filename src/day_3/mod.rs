#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    LOWEST,
    HIGHEST,
}

fn get_final_bin(input: &Vec<String>) -> String {
    let mut total: Vec<usize> = Vec::new();

    for line in input {
        for (i, c) in line.chars().enumerate() {
            if c.eq(&'0') {
                continue;
            }

            match total.get_mut(i) {
                Some(x) => *x += 1,
                None => total.push(1),
            }
        }
    }

    let input_breaking_num = input.len() / 2;

    let final_bin: String = total
        .iter()
        .map(|x| if *x > input_breaking_num { 1 } else { 0 })
        .map(|x| x.to_string())
        .collect();

    return final_bin;
}

pub fn run_part_1() -> i32 {
    let input: Vec<String> = include_str!("input")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let final_bin: String = get_final_bin(&input);
    let inverted_bin: String = final_bin
        .chars()
        .map(|c| if c.eq(&'1') { '0' } else { '1' })
        .collect();

    return i32::from_str_radix(&final_bin.as_str(), 2).unwrap()
        * i32::from_str_radix(&inverted_bin.as_str(), 2).unwrap();
}

pub fn run_part_2() -> i32 {
    let input: Vec<String> = include_str!("input")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let mut _oxygen = input.clone();

    while _oxygen.len() != 1 {
        
    }

    let mut _scrubber = input.clone();

    return 0;
}

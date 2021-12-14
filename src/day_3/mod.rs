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

    //println!("{:?}", final_bin);
    //println!("{:?}", inverted_bin);

    return i32::from_str_radix(&final_bin.as_str(), 2).unwrap()
        * i32::from_str_radix(&inverted_bin.as_str(), 2).unwrap();
}

pub fn run_part_2() -> isize {
    let input: Vec<String> = include_str!("input")
        .lines()
        .map(|x| x.to_string())
        .collect();

        let mut oxygen_generator: Vec<String> = input.clone();
        let mut c02_scrubber = input.clone();
        let mut current_index = 0;
    
        while oxygen_generator.len() > 1 {
            let ones = oxygen_generator.clone().into_iter().filter(|x| {
                &x[current_index..current_index+1] == "1"
            }).count();
            let zeros = oxygen_generator.len() - ones;
    
            if ones >= zeros {
                oxygen_generator = oxygen_generator.into_iter().filter(|x| &x[current_index..current_index+1] == "1").collect()
            } else {
                oxygen_generator = oxygen_generator.into_iter().filter(|x| &x[current_index..current_index+1] == "0").collect()
            }
            current_index += 1;
        }
    
        current_index = 0;
        while c02_scrubber.len() > 1 {
            let ones = c02_scrubber.clone().into_iter().filter(|x| {
                &x[current_index..current_index+1] == "1"
            }).count();
            let zeros = c02_scrubber.len() - ones;
    
            if ones >= zeros {
                c02_scrubber = c02_scrubber.into_iter().filter(|x| &x[current_index..current_index+1] == "0").collect()
            } else {
                c02_scrubber = c02_scrubber.into_iter().filter(|x| &x[current_index..current_index+1] == "1").collect()
            }
            current_index += 1;
        }
        println!("{:?}", c02_scrubber);
        println!("{:?}", oxygen_generator);
    
        let oxygen_generator_value = isize::from_str_radix(&oxygen_generator.join(""), 2).unwrap();
        let c02_scrubber_value = isize::from_str_radix(&c02_scrubber.join(""), 2).unwrap();
    
    
    return oxygen_generator_value * c02_scrubber_value;
}

fn _oxygen(list: Vec<String>) -> Option<String> {
    let mut list = list.clone();

    for index in 0..list.len() {
        let char_kept = _majority_at_index(&list, index);
        list = list
            .iter()
            .filter(|x| x.chars().nth(index).unwrap().eq(&char_kept))
            .map(|x| x.to_owned())
            .collect();
        if list.len() == 1 {
            return Some(list.get(0).unwrap().to_owned());
        }
    }

    return None;
}

fn _scrubber(list: Vec<String>) -> Option<String> {
    let mut list = list.clone();

    for index in 0..list.len() {
        let char_kept = _majority_at_index(&list, index);
        list = list
            .iter()
            .filter(|x| x.chars().nth(index).unwrap().ne(&char_kept))
            .map(|x| x.to_owned())
            .collect();
        if list.len() == 1 {
            return Some(list.get(0).unwrap().to_owned());
        }
    }

    return None;
}

fn _majority_at_index(list: &Vec<String>, index: usize) -> char {
    let mut total_1s = 0;
    let total_nums = list.len();

    for line in list {
        if let Some(c) = line.chars().nth(index) {
            if c.eq(&'1') {
                total_1s += 1;
            }
        } else {
            panic!("Index out of bounds.")
        }
    }

    if total_1s > total_nums / 2 {
        return '1';
    } else {
        return '0';
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1s_in_majority() {
        let temp_input = vec![
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "0".to_string(),
            "0".to_string(),
        ];

        assert_eq!('1', _majority_at_index(&temp_input, 0))
    }

    #[test]
    fn test_0s_in_majority() {
        let temp_input = vec![
            "1".to_string(),
            "1".to_string(),
            "0".to_string(),
            "0".to_string(),
            "0".to_string(),
        ];

        assert_eq!('0', _majority_at_index(&temp_input, 0))
    }
}

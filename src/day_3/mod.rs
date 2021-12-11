use std::{f32::EPSILON, vec};

fn get_parsed_input() -> (usize, Vec<i32>) {
    let temp_input = include_str!("input");

    (
        temp_input.lines().next().unwrap().len(),
        temp_input
            .lines()
            .map(|l| i32::from_str_radix(l, 2).unwrap())
            .collect(),
    )
}

fn bit_is_1_on_pos(input: i32, position: usize) -> bool {
    let mask = 1 << position;
    (input & mask) >> position == 1
}

#[cfg(test)]
mod bit_tests {
    use crate::day_3::bit_is_1_on_pos;

    #[test]
    fn bit_check() {
        let true_check = 0b00110110;

        assert_eq!(bit_is_1_on_pos(true_check, 0), false);
        assert_eq!(bit_is_1_on_pos(true_check, 1), true);
        assert_eq!(bit_is_1_on_pos(true_check, 1), true);
        assert_eq!(bit_is_1_on_pos(true_check, 0), false);
        assert_eq!(bit_is_1_on_pos(true_check, 1), true);
        assert_eq!(bit_is_1_on_pos(true_check, 1), true);
        assert_eq!(bit_is_1_on_pos(true_check, 0), false);
        assert_eq!(bit_is_1_on_pos(true_check, 0), false);
        assert_eq!(bit_is_1_on_pos(true_check, 0), false);
        assert_eq!(bit_is_1_on_pos(true_check, 0), false);
        assert_eq!(bit_is_1_on_pos(true_check, 0), false);
        assert_eq!(bit_is_1_on_pos(true_check, 0), false);
    }
}

fn add_to_vec(vector: &Vec<usize>, input: &i32) -> Vec<usize> {
    let mut output = vector.clone();

    output.reverse();

    for (index, col) in output.iter_mut().enumerate() {
        if bit_is_1_on_pos(*input, index) {
            *col += 1;
        }
    }

    output
}

pub fn run_part_1() -> i32 {
    let (cols, input) = get_parsed_input();
    let max_amount = input.len();

    let (gamma, epsilon) = input
        .iter()
        .fold(vec![0_usize; cols], |acc, x|{ 
            println!("{:?}, {:?}", &acc, acc.len());
            add_to_vec(&acc, x)
        })
        .iter()
        .fold((0_usize, 0_usize), |(gamma, epsilon), x| {
            //println!("{:#018b} : {:#018b}", &gamma, &epsilon);
            if x * 2 > max_amount {
                ((gamma << 1) | 1, (epsilon << 1))
            } else {
                ((gamma << 1), (epsilon << 1) | 1)
            }
        });

    return (gamma * epsilon) as i32;
}

pub fn run_part_2() -> i32 {
    return 0;
}

#[allow(dead_code)]
fn test_folds() {
    let numbers = [1, 2, 3, 4, 5];

    let zero = "0".to_string();

    let result = numbers
        .iter()
        .fold(zero, |acc, &x| format!("({} + {})", acc, x));

    println!("{:?}", result);
}

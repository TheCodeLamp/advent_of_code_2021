use std::{collections::HashSet, fmt::Display};

fn get_input() -> (Vec<usize>, Vec<Board>) {
    let mut input = include_str!("input").lines();
    let wining_nums_str: Vec<usize> = input
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut board_vec: Vec<Board> = Vec::new();

    let mut temp_board_vec: Vec<usize> = Vec::new();
    for line in input {
        //println!("{:?}", temp_board_vec);

        if line.len() < 2 && !temp_board_vec.is_empty() {
            //println!("{:?}", temp_board_vec);

            board_vec.push(temp_board_vec.into());
            temp_board_vec = Vec::new();
        }

        if line.eq("") {
            continue;
        }

        for item in line.split(" ") {
            if item.is_empty() {
                continue;
            }
            temp_board_vec.push(item.parse::<usize>().unwrap());
        }
    }

    (wining_nums_str, board_vec)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Board {
    board: Vec<Vec<usize>>,
}

impl Board {
    fn is_winning_board(&self, winning_numbers: &Vec<usize>) -> bool {
        let row = self
            .board
            .iter()
            .any(|f| f.iter().all(|x| winning_numbers.contains(x)));
        let column = (0..5)
            .map(|b| {
                self.board.iter().fold(Vec::<usize>::new(), |acc, x| {
                    let mut output = acc.clone();
                    output.push(x[b]);
                    //println!("{:?}", output);
                    output
                })
            })
            .any(|f| f.iter().all(|x| winning_numbers.contains(x)));

        return row || column;
    }

    fn get_score(&self, winning_numbers: &Vec<usize>) -> usize {
        //println!("{}", self);
        let list = self.board.iter().fold(Vec::<usize>::new(), |acc, x| {
            let mut output = acc.clone();
            output.append(&mut x.clone());
            output
        });

        //println!("{:?}", list);

        let filtered_list: Vec<usize> = list
            .iter()
            .filter_map(|x| {
                if winning_numbers.contains(x) {
                    None
                } else {
                    Some(*x)
                }
            })
            .collect();

        //println!("{:?}", filtered_list);

        return filtered_list.iter().sum::<usize>() * winning_numbers.last().unwrap();
    }
}

impl From<Vec<usize>> for Board {
    fn from(from: Vec<usize>) -> Self {
        if from.len() != 25 {
            panic!("Not right amount of input: {}", from.len())
        }

        let mut temp = Vec::<Vec<usize>>::new();

        let mut from_iter = from.iter();

        for _ in 0..5 {
            let mut outer_temp: Vec<usize> = Vec::new();
            for _ in 0..5 {
                outer_temp.push(*from_iter.next().unwrap());
            }
            temp.push(outer_temp);
        }

        return Board { board: temp };
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut temp: String = "".to_string();

        for row in &self.board {
            let mut out_row = "".to_string();
            for item in row {
                out_row.push_str(format!("{: >3}", item).as_str());
            }
            temp.push_str(&out_row.as_str());
            temp.push_str("\n");
        }

        write!(f, "{}", temp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_winning_board() {
        let test_board = Board::from(vec![0; 25]);

        assert!(test_board.is_winning_board(&vec![0]));

        let test_board = Board::from((0..25).collect::<Vec<usize>>());

        //println!("{}", test_board);

        assert!(!test_board.is_winning_board(&vec![0]));
        assert!(test_board.is_winning_board(&vec![0, 5, 10, 15, 20]));
        assert!(test_board.is_winning_board(&vec![2, 7, 12, 17, 22]));
    }

    #[test]
    fn test_get_score() {
        let test_board = Board::from((0..25).collect::<Vec<usize>>());

        assert_eq!(
            test_board.get_score(&(0..5).collect::<Vec<usize>>()),
            (5..25).sum::<usize>() * 4
        )
    }
}

pub fn run_part_1() -> Option<usize> {
    let (winning_nums, boards) = get_input();

    let mut checking_nums: Vec<usize> = Vec::new();

    for num in winning_nums {
        checking_nums.push(num);
        let results: Vec<usize> = boards
            .iter()
            .filter_map(|f| {
                if f.is_winning_board(&checking_nums) {
                    Some(f.get_score(&checking_nums))
                } else {
                    None
                }
            })
            .collect();

        //println!("{:?}", results);

        if results.len() > 0 {
            return Some(*results.get(0).unwrap());
        }
    }

    return None;
}

pub fn run_part_2() -> Option<usize> {
    let (winning_nums, boards) = get_input();

    let mut checking_nums: Vec<usize> = Vec::new();

    let mut winners: HashSet<Board> = HashSet::new();

    for num in winning_nums {
        checking_nums.push(num);


        let temp_winners = winners.clone();

        let new_winners: Vec<Board> = boards
            .iter()
            .filter(|f| f.is_winning_board(&checking_nums) && !temp_winners.contains(f)).map(|b| b.clone() ).collect();

        for board in &new_winners {
            winners.insert(board.clone());
        }

        if winners.len() == boards.len(){
            return Some(new_winners.first().unwrap().get_score(&checking_nums));
        }
    }

    return None;
}

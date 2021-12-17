#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct LaternFish {
    timer: i32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct LaternFishSchool {
    school: Vec<LaternFish>,
}

impl LaternFishSchool {
    fn tick(&mut self) {
        let mut new_fish = 0;
        for fish in self.school.iter_mut() {
            if fish.timer == 0 {
                fish.timer = 6;
                new_fish += 1;
            } else {
                fish.timer += -1;
            }
        }

        for _ in 0..new_fish {
            self.school.push(LaternFish { timer: 8 });
        }
    }

    fn count_fish(&self) -> usize {
        self.school.len()
    }

    fn new_from_vec(vec: Vec<i32>) -> Self {
        let fishes: Vec<LaternFish> = vec.iter().map(|x| LaternFish { timer: *x }).collect();

        LaternFishSchool { school: fishes }
    }
}

pub fn run_part_1() -> Option<usize> {
    let input: Vec<i32> = include_str!("input")
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut fish_shool = LaternFishSchool::new_from_vec(input);

    for _ in 0..80 {
        fish_shool.tick();
    }

    return Some(fish_shool.count_fish());
}

pub fn run_part_2() -> Option<usize> {
    None
}

pub fn run_part_1() -> Option<usize> {
    let mut input: Vec<usize> = include_str!("input")
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    input.sort_unstable();

    let mut max_fuel = usize::MAX;

    for dif in 0..=*input.last().unwrap() {
        let mut fuel_usage = 0;

        for num in input.iter() {
            fuel_usage += (*num as i32 - dif as i32).abs() as usize;
        }

        if fuel_usage < max_fuel {
            max_fuel = fuel_usage;
        }
    }

    Some(max_fuel)
}
pub fn run_part_2() -> Option<usize> {
    let mut input: Vec<usize> = include_str!("input")
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    input.sort_unstable();

    let mut max_fuel = usize::MAX;

    for dif in 0..=*input.last().unwrap() {
        let mut fuel_usage = 0;

        for num in input.iter() {
            let n = (*num as i32 - dif as i32).abs();
            fuel_usage += (n*(n+1)/2) as usize;
        }

        if fuel_usage < max_fuel {
            max_fuel = fuel_usage;
        }
    }

    Some(max_fuel)
}

pub fn run_part_1() -> usize {
    let parsed_input: Vec<usize> = include_str!("input").lines().map(|x| x.parse::<usize>().unwrap()).collect();

    return parsed_input.windows(2).filter(|x| x[0] < x[1]).count();
}

pub fn run_part_2() -> usize {
    let parsed_input: Vec<_> = include_str!("input").lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let trip_windows: Vec<_> = parsed_input.windows(3).map(|f| f.into_iter()).collect();

    return trip_windows.windows(2).filter(|x| x[0].clone().sum::<i32>() < x[1].clone().sum()).count();
}

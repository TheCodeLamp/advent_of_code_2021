mod point;
use point::Point;

fn translate(line: &str) -> (i32, i32) {
    let mut temp_split = line.split(" ");

    let dir = temp_split.next().unwrap();
    let amount = temp_split.next().unwrap().parse::<i32>().unwrap();

    return match dir {
        "forward" => (1 * amount, 0),
        "down" => (0, 1 * amount),
        "up" => (0, -1 * amount),
        _ => panic!("Wrong word used: {:?}", dir),
    };
}

pub fn run_part_1() -> i32 {
    let translated_directions: Vec<Point> = include_str!("input")
        .lines()
        .map(|x| Point::from(translate(x)))
        .collect();

    let pos: Point = translated_directions.into_iter().sum();

    return pos.0 * pos.1;
}

pub fn run_part_2() -> i32 {
    let translated_directions: Vec<Point> = include_str!("input")
        .lines()
        .map(|x| Point::from(translate(x)))
        .collect();

    let mut aim = 0_i32;
    let mut pos = Point::default();

    for point in translated_directions{
        if point.0 == 0{
            aim += point.1;
        }else
        if point.1 == 0{
            pos += (point.0, point.0*aim).into();
        } else {
            panic!("Both x and y is used");
        }
    }

    return pos.0 * pos.1;
}

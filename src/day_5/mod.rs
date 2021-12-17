use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(data: (usize, usize)) -> Self {
        Point {
            x: data.0,
            y: data.1,
        }
    }
}

impl Point {
    fn create_line_expanded(&self, point: &Point) -> Vec<Point> {
        if self.x == point.x {
            let (from, to) = {
                if self.y < point.y {
                    (self.y, point.y)
                } else {
                    (point.y, self.y)
                }
            };

            return (from..=to).map(|f| Point { x: self.x, y: f }).collect();
        } else if self.y == point.y {
            let (from, to) = {
                if self.x < point.x {
                    (self.x, point.x)
                } else {
                    (point.x, self.x)
                }
            };

            return (from..=to).map(|f| Point { x: f, y: self.y }).collect();
        } else if (self.x as i32 - point.x as i32).abs() == (self.y as i32 - point.y as i32).abs() {
            let direction: (i32, i32) = (
                (point.x as i32 - self.x as i32).clamp(-1, 1),
                (point.y as i32 - self.y as i32).clamp(-1, 1),
            );
            let distance = (self.x as i32 - point.x as i32).abs();

            return (0..=distance)
                .map(|x| Point {
                    x: (self.x as i32 + direction.0*x) as usize,
                    y: (self.y as i32 + direction.1*x) as usize,
                })
                .collect();
        }

        return Vec::new();
    }
    fn create_line(&self, point: &Point) -> Vec<Point> {
        //Line for part 1
        if self.x == point.x {
            let (from, to) = {
                if self.y < point.y {
                    (self.y, point.y)
                } else {
                    (point.y, self.y)
                }
            };

            (from..=to).map(|f| Point { x: self.x, y: f }).collect()
        } else if self.y == point.y {
            let (from, to) = {
                if self.x < point.x {
                    (self.x, point.x)
                } else {
                    (point.x, self.x)
                }
            };

            (from..=to).map(|f| Point { x: f, y: self.y }).collect()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    #[test]
    fn point_create_line_test() {
        let point1 = Point { x: 0, y: 0 };
        let point2 = Point { x: 0, y: 2 };

        let vector = vec![point1, Point { x: 0, y: 1 }, point2];

        assert_eq!(point1.create_line(&point2), vector);

        let point1 = Point { x: 0, y: 0 };
        let point2 = Point { x: 2, y: 0 };

        let vector = vec![point1, Point { x: 1, y: 0 }, point2];

        assert_eq!(point1.create_line(&point2), vector);

        let point1 = Point { x: 2, y: 0 };
        let point2 = Point { x: 0, y: 0 };

        let vector = vec![point2, Point { x: 1, y: 0 }, point1];

        assert_eq!(point1.create_line(&point2), vector);
    }

    #[test]
    fn point_create_line_expanded_test() {
        let line = Point { x: 0, y: 0 }.create_line_expanded(&Point { x: 3, y: 4 });

        assert!(line.is_empty()); // Checks that the line is empty;

        let must_contain: HashSet<Point> = [
            Point { x: 0, y: 4 },
            Point { x: 1, y: 3 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 1 },
        ]
        .into();

        let line: HashSet<Point> = Point { x: 0, y: 4 }
            .create_line_expanded(&Point { x: 3, y: 1 })
            .into_iter()
            .collect();

        assert!(must_contain.is_subset(&line));

        let must_contain: HashSet<Point> = [
            Point { x: 0, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
        ]
        .into();

        let line: HashSet<Point> = Point { x: 0, y: 0 }
            .create_line_expanded(&Point { x: 3, y: 3 })
            .into_iter()
            .collect();

        println!("{:?}", line);

        assert!(must_contain.is_subset(&line));

        let must_contain: HashSet<Point> = [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 3 },
            Point { x: 3, y: 4 },
        ]
        .into();

        let line: HashSet<Point> = Point { x: 0, y: 1 }
            .create_line_expanded(&Point { x: 3, y: 4 })
            .into_iter()
            .collect();

        assert!(must_contain.is_subset(&line));

        let line: HashSet<Point> = Point { x: 3, y: 4 }
            .create_line_expanded(&Point { x: 0, y: 1 })
            .into_iter()
            .collect();

        assert!(must_contain.is_subset(&line));
    }
}

#[derive(Default)]
struct Map {
    ocupied_spaces: HashMap<Point, usize>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (highest_x, highest_y) = self.highest_xy();

        let mut output: String = "".into();

        for x in 0..=highest_x {
            for y in 0..=highest_y {
                let temp_point = Point { x: x, y: y };
                match self.ocupied_spaces.get(&temp_point) {
                    Some(val) => output.push_str(&val.to_string()),
                    None => output.push(' '),
                }
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

impl Map {
    fn input_list(&mut self, list: Vec<Point>) {
        for point in list {
            let add_amount = match self.ocupied_spaces.get(&point) {
                Some(amount) => amount + 1,
                None => 1,
            };
            self.ocupied_spaces.insert(point, add_amount);
        }
    }

    fn _add_point(&self, point: Point) -> Self {
        let mut temp_set = self.ocupied_spaces.clone();

        match self.ocupied_spaces.get(&point) {
            Some(amount) => temp_set.insert(point, amount + 1),
            None => temp_set.insert(point, 1),
        };
        Map {
            ocupied_spaces: temp_set,
        }
    }

    fn new() -> Self {
        Map::default()
    }

    fn get_amount_over_1(&self) -> usize {
        self.ocupied_spaces.values().filter(|x| **x > 1).count()
    }

    fn highest_xy(&self) -> (usize, usize) {
        let (mut highest_x, mut highest_y) = (0, 0);

        for point in self.ocupied_spaces.keys() {
            if point.x > highest_x {
                highest_x = point.x;
            }
            if point.y > highest_y {
                highest_y = point.y;
            }
        }

        return (highest_x, highest_y);
    }
}

fn get_input() -> Vec<(Point, Point)> {
    include_str!("input")
        .lines()
        .map(|f| {
            let mut point_iter = f.split(" -> ").map(|s| {
                let mut cord_split = s.split(",");
                Point {
                    x: cord_split.next().unwrap().parse::<usize>().unwrap(),
                    y: cord_split.next().unwrap().parse::<usize>().unwrap(),
                }
            });

            (point_iter.next().unwrap(), point_iter.next().unwrap())
        })
        .collect()
}

pub fn run_part_1() -> Option<usize> {
    let map: Map = get_input().iter().fold(Map::new(), |mut acc, x| {
        acc.input_list(x.0.create_line(&x.1));
        acc
    });

    Some(map.get_amount_over_1())
}

pub fn run_part_2() -> Option<usize> {
    let map: Map = get_input().iter().fold(Map::new(), |mut acc, x| {
        acc.input_list(x.0.create_line_expanded(&x.1));
        acc
    });

    Some(map.get_amount_over_1())
}

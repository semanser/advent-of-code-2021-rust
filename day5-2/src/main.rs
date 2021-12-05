use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
    points: Vec<Point>,
}

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Unable to read file");
    let mut segments: Vec<Segment> = Vec::new();

    // Read all the input data
    data.lines().for_each(|line| {
        let mut segment = Segment {
            start: Point::new(0, 0),
            end: Point::new(0, 0),
            points: vec![],
        };

        line.split("->")
            .enumerate()
            .for_each(|(i, line_coordinates)| {
                let coordinates: Vec<i16> = line_coordinates
                    .trim()
                    .split(',')
                    .map(|coordinate| coordinate.parse::<i16>().unwrap())
                    .collect();

                if i == 0 {
                    segment.start = Point {
                        x: coordinates[0],
                        y: coordinates[1],
                    }
                }
                if i == 1 {
                    segment.end = Point {
                        x: coordinates[0],
                        y: coordinates[1],
                    }
                }
            });

        segments.push(segment);
    });

    // Calculate all the coordinates
    segments.iter_mut().for_each(|segment| {
        if segment.start.x == segment.end.x {
            let mut start = segment.start.y;
            let mut end = segment.end.y;

            if segment.start.y > segment.end.y {
                start = segment.end.y;
                end = segment.start.y;
            }
            for y in start..=end {
                let point = Point::new(segment.start.x, y);
                if number_of_duplicates(&segment.points, &point) == 0 {
                    segment.points.push(point);
                }
            }
        } else if segment.start.y == segment.end.y {
            let mut start = segment.start.x;
            let mut end = segment.end.x;

            if segment.start.x > segment.end.x {
                start = segment.end.x;
                end = segment.start.x;
            }
            for x in start..=end {
                let point = Point::new(x, segment.start.y);
                if number_of_duplicates(&segment.points, &point) == 0 {
                    segment.points.push(point);
                }
            }
        } else {
            // Distance is the same for X and Y
            let mut distance: i16 = (segment.start.x - segment.end.x).into();
            let mut distance_traveled: i16 = 0;

            segment.points.push(Point {
                x: segment.start.x,
                y: segment.start.y,
            });
            segment.points.push(Point {
                x: segment.end.x,
                y: segment.end.y,
            });

            while distance.abs() != 0 {
                let mut point = Point::new(0, 0);
                if segment.start.x > segment.end.x {
                    point.x = segment.start.x - distance_traveled;
                } else {
                    point.x = segment.start.x + distance_traveled;
                }

                if segment.start.y > segment.end.y {
                    point.y = segment.start.y - distance_traveled;
                } else {
                    point.y = segment.start.y + distance_traveled;
                }
                distance_traveled += 1;

                if distance < 0 {
                    distance += 1;
                } else {
                    distance -= 1;
                }

                if number_of_duplicates(&segment.points, &point) == 0 {
                    segment.points.push(point);
                }
            }
        }
    });

    // Flat and dedupe all the coordinates
    let mut coordinates_collection: HashMap<String, i16> = HashMap::new();

    segments.iter().for_each(|segment| {
        segment.points.iter().for_each(|point| {
            let x = point.x.to_string();
            let y = point.y.to_string();

            let mut key: String = String::from("");
            key.push_str(&x);
            key.push_str("-");
            key.push_str(&y);

            if coordinates_collection.contains_key(&key) {
                let old_counter = coordinates_collection.get(&key).unwrap().clone();
                coordinates_collection.insert(key, old_counter + 1);
            } else {
                coordinates_collection.insert(key, 0);
            }
        });
    });
    let duplicated = coordinates_collection
        .values()
        .filter(|value| *value != &0)
        .count();

    println!("Duplicated points: {}", duplicated);
}

fn number_of_duplicates(points: &Vec<Point>, point: &Point) -> i16 {
    let all_coordinates_count = points.len();
    let mut number_of_duplicates = 0;

    for i in 0..all_coordinates_count {
        let coordinate = points.get(i).unwrap();

        if coordinate.x == point.x && coordinate.y == point.y {
            number_of_duplicates += 1;
        }
    }

    number_of_duplicates
}

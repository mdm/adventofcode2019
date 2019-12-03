#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Segment {
    direction: Direction,
    length: i32,
}

fn parse_wire() -> Vec<Segment> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    let mut wire = Vec::new();
    let regex = regex::Regex::new(r"(?:([URDL])(\d+))+").unwrap();
    for segment in regex.captures_iter(&input) {
        let direction = match &segment[1] {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => unreachable!()
        };

        let length = segment[2].parse::<i32>().unwrap();

        wire.push(Segment { direction, length });
    }

    return wire;
}

fn measure_wire(wire: &Vec<Segment>) -> [i32; 4] {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut extents: [i32; 4] = [0, 0, 0, 0];

    for segment in wire {
        match &segment.direction {
            Direction::Up => {
                y -= segment.length;
                if y < extents[0] {
                    extents[0] = y;
                }
            }
            Direction::Right => {
                x += segment.length;
                if x > extents[1] {
                    extents[1] = x;
                }
            }
            Direction::Down => {
                y += segment.length;
                if y > extents[2] {
                    extents[2] = y;
                }
            }
            Direction::Left => {
                x -= segment.length;
                if x < extents[3] {
                    extents[3] = x;
                }
            }
        }
    }

    return extents;
}

fn mark_wire(grid: &mut Vec<Vec<u32>>, x_offset: usize, y_offset: usize, wire: &Vec<Segment>) {
    let mut x = x_offset;
    let mut y = y_offset;
    let mut steps = 0;
    for segment in wire {
        for _ in 0..segment.length {
            match segment.direction {
                Direction::Up => {
                    y -= 1;
                }
                Direction::Right => {
                    x += 1;
                }
                Direction::Down => {
                    y += 1;
                }
                Direction::Left => {
                    x -= 1;
                }
            }

            steps += 1;
            grid[y][x] = steps;
        }
    }
}

fn main() {
    let wire1 = parse_wire();
    let extents1 = measure_wire(&wire1);
    let wire2 = parse_wire();
    let extents2 = measure_wire(&wire2);

    let extents = [
        std::cmp::min(extents1[0], extents2[0]),
        std::cmp::max(extents1[1], extents2[1]),
        std::cmp::max(extents1[2], extents2[2]),
        std::cmp::min(extents1[3], extents2[3]),
    ];

    // dbg!(extents);

    let width = (extents[1] - extents[3] + 1) as usize;
    let height = (extents[2] - extents[0] + 1) as usize;
    let x_offset = (0 - extents[3]) as usize;
    let y_offset = (0 - extents[0]) as usize;

    let mut grid1 = vec![vec![0; width]; height];
    let mut grid2 = vec![vec![0; width]; height];

    mark_wire(&mut grid1, x_offset, y_offset, &wire1);
    mark_wire(&mut grid2, x_offset, y_offset, &wire2);

    let mut min_distance = (width + height) as i32;
    for y in 0..height {
        for x in 0..width {
            let distance = (x as i32 - x_offset as i32).abs() + (y as i32 - y_offset as i32).abs();
            if grid1[y][x] > 0 && grid2[y][x] > 0 && distance < min_distance {
                min_distance = distance;
            }
        }
    }

    println!("{}", min_distance);

    let mut min_delay = (width * height) as u32;
    for y in 0..height {
        for x in 0..width {
            let delay = grid1[y][x] + grid2[y][x];
            if grid1[y][x] > 0 && grid2[y][x] > 0 && delay < min_delay {
                min_delay = delay;
            }
        }
    }

    println!("{}", min_delay);
}

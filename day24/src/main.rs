use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum MapTile {
    Bug,
    EmptySpace,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => Position { x: self.x, y: self.y - 1 },
            Direction::Right => Position { x: self.x + 1, y: self.y },
            Direction::Down => Position { x: self.x, y: self.y + 1 },
            Direction::Left => Position { x: self.x - 1, y: self.y },
        }
    }
}

fn display(map: &Vec<Vec<MapTile>>) {
    let snapshot = map.iter().map(|row| {
        row.iter().map(|tile| match tile {
            MapTile::EmptySpace => '.',
            MapTile::Bug => '#',
        }).collect::<String>()
    }).collect::<Vec<_>>().join("\n");

    println!("{}\n---", snapshot);
}

fn adjacent_bugs(map: &Vec<Vec<MapTile>>, position: Position) -> u32 {
    let mut bugs = 0;
    for direction in vec!(Direction::Up, Direction::Right, Direction::Down, Direction::Left) {
        match direction {
            Direction::Up => {
                if position.y == 0 {
                    continue;
                }
            }
            Direction::Right => {
                if position.x == 4 {
                    continue;
                }
            }
            Direction::Down => {
                if position.y == 4 {
                    continue;
                }
            }
            Direction::Left => {
                if position.x == 0 {
                    continue;
                }
            }
        }

        let stepped = position.step(&direction);
        if let Some(row) = map.get(stepped.y) {
            if let Some(tile) = row.get(stepped.x) {
                if *tile == MapTile::Bug {
                    bugs += 1;
                }
            }
        }
    }

    // dbg!(position, bugs);
    return bugs;
}

fn run_timestep(map: Vec<Vec<MapTile>>) -> Vec<Vec<MapTile>> {
    let new_map = map.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, tile)| {
            let bugs = adjacent_bugs(&map, Position { x, y });
            match tile {
                MapTile::EmptySpace => {
                    if bugs == 1 || bugs == 2 {
                        MapTile::Bug
                    } else {
                        MapTile::EmptySpace
                    }
                },
                MapTile::Bug => {
                    if bugs != 1 {
                        MapTile::EmptySpace
                    } else {
                        MapTile::Bug
                    }
                },
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    return new_map;
}

fn biodiversity(map: &Vec<Vec<MapTile>>) -> u64 {
    let mut rating = 0;
    let mut factor = 1;
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == MapTile::Bug {
                rating += factor;
            }
            factor <<= 1;
        }
    }

    return rating;
}

fn main() {
    let stdin = std::io::stdin();
    let mut original_map = stdin.lock().lines().enumerate().map(|(y, row)| {
        row.unwrap().chars().enumerate().map(|(x, character)| match character {
            '.' => MapTile::EmptySpace,
            '#' => MapTile::Bug,
            _ => unreachable!(),
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    display(&map);

    let mut map = original_map.clone();
    let mut history = HashMap::new();
    let mut step = 0;
    loop {
        if history.contains_key(&map) {
            break;
        }

        history.insert(map.clone(), step);

        map = run_timestep(map);
        step += 1;
    }

    display(&map);
    println!("{}", biodiversity(&map));

    let mut map = original_map.clone();
    for _ in 0..10 {
        map = run_timestep_recursive(map);
    }
}

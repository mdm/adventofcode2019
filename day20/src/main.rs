use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum MapTile {
    Floor,
    Wall,
    Portal,
    Space(Option<char>),
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

#[derive(Debug)]
struct KeyLocation {
    position: Position,
    distance: u32,
}

fn display(map: &Vec<Vec<MapTile>>) {
    let snapshot = map.iter().map(|row| {
        row.iter().map(|tile| match tile {
            MapTile::Floor => '.',
            MapTile::Wall => '#',
            MapTile::Portal => '*',
            MapTile::Space(Some(label)) => *label,
            MapTile::Space(None) => ' ',
        }).collect::<String>()
    }).collect::<Vec<_>>().join("\n");

    println!("{}", snapshot);
}

fn identify_portals(
    map: Vec<Vec<MapTile>>
) -> (Vec<Vec<MapTile>>, Position, Position, HashMap<Position, (Position, bool)>) {
    let mut portals_by_name = HashMap::new();

    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let new_map = map.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, tile)| {
            let mut new_tile = tile.clone();
            if let MapTile::Floor = tile {
                let position = Position { x, y };
                for direction in vec!(Direction::Up, Direction::Right, Direction::Down, Direction::Left) {
                    let stepped = position.step(&direction);
                    if let MapTile::Space(Some(label1)) = map[stepped.y][stepped.x] {
                        new_tile = MapTile::Portal;
                        let stepped = stepped.step(&direction);
                        if let MapTile::Space(Some(label2)) = map[stepped.y][stepped.x] {
                            let mut name = String::new();
                            if label1 < label2 {
                                name.push(label1);
                                name.push(label2);
                            } else {
                                name.push(label2);
                                name.push(label1);
                            }
                            if stepped.x == 0 || stepped.x == max_x || stepped.y == 0 || stepped.y == max_y {
                                portals_by_name.entry(name).or_insert(Vec::new()).push((position.clone(), false));
                            } else {
                                portals_by_name.entry(name).or_insert(Vec::new()).push((position.clone(), true));
                            }
                        }
                    }
                }
            }

            new_tile
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut portals = HashMap::new();
    for (_name, endpoints) in portals_by_name.clone() {
        if endpoints.len() == 2 {
            portals.insert(endpoints[0].0.clone(), endpoints[1].clone());
            portals.insert(endpoints[1].0.clone(), endpoints[0].clone());
        }
    }

    // dbg!(&portals);

    return (new_map, portals_by_name.get("AA").unwrap()[0].0.clone(), portals_by_name.get("ZZ").unwrap()[0].0.clone(), portals);
}

fn can_visit(
    map: &Vec<Vec<MapTile>>,
    position: &Position
) -> bool {
    match map[position.y][position.x] {
        MapTile::Floor => true,
        MapTile::Wall => false,
        MapTile::Portal => true,
        MapTile::Space(_) => false,
    }
}

fn shortest_path(
    map: &Vec<Vec<MapTile>>,
    start: &Position,
    target: &Position,
    portals: &HashMap<Position, (Position, bool)>
) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut work = VecDeque::new();

    visited.insert(start.clone());
    distances.insert(start.clone(), 0);
    work.push_back(start.clone());

    while !work.is_empty() {
        if let Some(current_position) = work.pop_front() {
            let mut current_distance = 0;
            if let Some(distance) = distances.get(&current_position) {
                current_distance = *distance;

                if current_position == *target {
                    return Some(current_distance);
                }
            }

            for direction in vec!(Direction::Up, Direction::Right, Direction::Down, Direction::Left) {
                let new_position = current_position.step(&direction);
                if !visited.contains(&new_position) && can_visit(map, &new_position) {
                    visited.insert(new_position.clone());
                    distances.insert(new_position.clone(), current_distance + 1);
                    work.push_back(new_position);
                }
            }

            if let Some(destination) = portals.get(&current_position) {
                let new_position = destination.0.clone();
                if !visited.contains(&new_position) && can_visit(map, &new_position) {
                    visited.insert(new_position.clone());
                    distances.insert(new_position.clone(), current_distance + 1);
                    work.push_back(new_position.clone());
                }
            }
        }
    }

    return None;
}

fn shortest_path_recursive(
    map: &Vec<Vec<MapTile>>,
    start: &Position,
    target: &Position,
    portals: &HashMap<Position, (Position, bool)>
) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut work = VecDeque::new();

    visited.insert((start.clone(), 0));
    distances.insert((start.clone(), 0), 0);
    work.push_back((start.clone(), 0));

    while !work.is_empty() {
        if let Some(current_position) = work.pop_front() {
            // dbg!(&current_position);
            let mut current_distance = 0;
            if let Some(distance) = distances.get(&current_position) {
                current_distance = *distance;

                if current_position.0 == *target && current_position.1 == 0 {
                    return Some(current_distance);
                }
            }

            for direction in vec!(Direction::Up, Direction::Right, Direction::Down, Direction::Left) {
                let new_position = (current_position.0.step(&direction), current_position.1);
                if !visited.contains(&new_position) && can_visit(map, &new_position.0) {
                    visited.insert(new_position.clone());
                    distances.insert(new_position.clone(), current_distance + 1);
                    work.push_back(new_position);
                }
            }

            if let Some(destination) = portals.get(&current_position.0) {
                let new_position = if destination.1 {
                    (destination.0.clone(), current_position.1 - 1)
                } else {
                    (destination.0.clone(), current_position.1 + 1)
                };
                if !visited.contains(&new_position) && can_visit(map, &new_position.0) && new_position.1 >= 0 {
                    visited.insert(new_position.clone());
                    distances.insert(new_position.clone(), current_distance + 1);
                    work.push_back(new_position);
                }
            }
        }
    }

    return None;
}

fn main() {
    let stdin = std::io::stdin();
    let map = stdin.lock().lines().map(|row| {
        row.unwrap().chars().map(|character| match character {
            '.' => MapTile::Floor,
            '#' => MapTile::Wall,
            ' ' => MapTile::Space(None),
            label => if character.is_ascii_uppercase() {
                MapTile::Space(Some(label))
            } else {
                MapTile::Space(None)
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    // display(&map);
    let (map, start, target, portals) = identify_portals(map);
    match shortest_path(&map, &start, &target, &portals) {
        Some(distance) => { println!("{}", distance); }
        None => { println!("No path."); }
    }
    match shortest_path_recursive(&map, &start, &target, &portals) {
        Some(distance) => { println!("{}", distance); }
        None => { println!("No path."); }
    }
}

use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum MapTile {
    Floor,
    Wall,
    Explorer,
    Key(char),
    Door(char),
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
            MapTile::Explorer => '@',
            MapTile::Key(key) => *key,
            MapTile::Door(door) => *door,
        }).collect::<String>()
    }).collect::<Vec<_>>().join("\n");

    println!("{}", snapshot);
}

fn can_visit(
    map: &Vec<Vec<MapTile>>,
    position: &Position,
    held_keys: &HashSet<char>
) -> bool {
    match map[position.y][position.x] {
        MapTile::Floor => true,
        MapTile::Wall => false,
        MapTile::Explorer => true,
        MapTile::Key(_key) => true,
        MapTile::Door(door) => held_keys.contains(&door.to_ascii_lowercase()),
    }
}

fn reachable_keys(
    map: &Vec<Vec<MapTile>>,
    explorer: &Position,
    held_keys: &HashSet<char>
) -> HashMap<char, KeyLocation> {
    let mut key_locations = HashMap::new();

    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut work = VecDeque::new();

    visited.insert(explorer.clone());
    distances.insert(explorer.clone(), 0);
    work.push_back(explorer.clone());

    while !work.is_empty() {
        if let Some(current_position) = work.pop_front() {
            let mut current_distance = 0;
            if let Some(distance) = distances.get(&current_position) {
                current_distance = *distance;

                if let MapTile::Key(key) = map[current_position.y][current_position.x] {
                    key_locations.insert(key, KeyLocation {
                        position: current_position.clone(),
                        distance: *distance
                    });
                }
            }

            let new_position = current_position.step(&Direction::Up);
            if !visited.contains(&new_position) && can_visit(map, &new_position, held_keys) {
                visited.insert(new_position.clone());
                distances.insert(new_position.clone(), current_distance + 1);
                work.push_back(new_position);
            }

            let new_position = current_position.step(&Direction::Right);
            if !visited.contains(&new_position) && can_visit(map, &new_position, held_keys) {
                visited.insert(new_position.clone());
                distances.insert(new_position.clone(), current_distance + 1);
                work.push_back(new_position);
            }

            let new_position = current_position.step(&Direction::Down);
            if !visited.contains(&new_position) && can_visit(map, &new_position, held_keys) {
                visited.insert(new_position.clone());
                distances.insert(new_position.clone(), current_distance + 1);
                work.push_back(new_position);
            }

            let new_position = current_position.step(&Direction::Left);
            if !visited.contains(&new_position) && can_visit(map, &new_position, held_keys) {
                visited.insert(new_position.clone());
                distances.insert(new_position.clone(), current_distance + 1);
                work.push_back(new_position);
            }
        }
    }

    return key_locations;
}

fn collect_keys(
    map: &Vec<Vec<MapTile>>,
    explorers: &Vec<Position>,
    held_keys: &HashSet<char>,
    memo: &mut HashMap<(usize, String), u32>
) -> u32 {
    let mut held_keys_vec = held_keys.iter().cloned().collect::<Vec<char>>();
    held_keys_vec.sort();
    let held_keys_string = held_keys_vec.iter().collect::<String>();

    let width = map[0].len();
    let height = map.len();
    let mut position_hash = 0;
    for explorer in explorers {
        position_hash *= width * height;
        position_hash += explorer.y * width + explorer.x;
    }

    if let Some(saved_min_steps) = memo.get(&(position_hash, held_keys_string.clone())) {
        return *saved_min_steps;
    }

    let mut min_steps = std::u32::MAX;
    for (i, explorer) in explorers.iter().enumerate() {
        let key_locations = reachable_keys(&map, &explorer, &held_keys);

        if key_locations.len() == 0 {
            continue;
        }

        for (key, location) in key_locations {
            let mut new_map = (*map).clone();
            new_map[location.position.y][location.position.x] = MapTile::Explorer;
            new_map[explorer.y][explorer.x] = MapTile::Floor;

            let mut new_explorers = explorers.clone();
            new_explorers[i] = location.position;

            let mut new_held_keys = held_keys.clone();
            new_held_keys.insert(key);

            min_steps = std::cmp::min(
                min_steps,
                location.distance + collect_keys(&new_map, &new_explorers, &new_held_keys, memo)
            );
        }
    }

    if min_steps == std::u32::MAX {
        min_steps = 0;
    }

    memo.insert((position_hash, held_keys_string), min_steps);

    return min_steps;
}

fn main() {
    let stdin = std::io::stdin();
    let mut explorers = Vec::new();
    let mut num_keys = 0;
    let mut map = stdin.lock().lines().enumerate().map(|(y, row)| {
        row.unwrap().chars().enumerate().map(|(x, character)| match character {
            '.' => MapTile::Floor,
            '#' => MapTile::Wall,
            '@' => {
                explorers.push(Position { x, y });
                MapTile::Explorer
            },
            _ => if character.is_ascii_lowercase() {
                num_keys += 1;
                MapTile::Key(character)
            } else {
                MapTile::Door(character)
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    // display(&map);
    println!("{}", collect_keys(&map, &explorers, &HashSet::new(), &mut HashMap::new()));

    let new_walls = vec!(
        explorers[0].clone(),
        explorers[0].step(&Direction::Up),
        explorers[0].step(&Direction::Right),
        explorers[0].step(&Direction::Down),
        explorers[0].step(&Direction::Left),
    );

    for wall in new_walls {
        map[wall.y][wall.x] = MapTile::Wall;
    }

    let new_explorers = vec!(
        explorers[0].step(&Direction::Up).step(&Direction::Left),
        explorers[0].step(&Direction::Up).step(&Direction::Right),
        explorers[0].step(&Direction::Down).step(&Direction::Left),
        explorers[0].step(&Direction::Down).step(&Direction::Right),
    );

    for explorer in new_explorers.clone() {
        map[explorer.y][explorer.x] = MapTile::Explorer;
    }

    // display(&map);
    println!("{}", collect_keys(&map, &new_explorers, &HashSet::new(), &mut HashMap::new()));
}


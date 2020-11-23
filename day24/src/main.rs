use std::io::BufRead;
use std::collections::HashMap;


#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
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

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn display(&self) {
        let snapshot = self.0.iter().map(|row| {
            row.iter().map(|tile| match tile {
                0 => '.',
                1 => '#',
                _ => unreachable!(),
            }).collect::<String>()
        }).collect::<Vec<_>>().join("\n");

        println!("{}\n---", snapshot);
    }

    fn biodiversity(&self) -> u32 {
        let mut rating = 0;
        let mut factor = 1;
        for row in self.0.iter() {
            for tile in row.iter() {
                rating += tile * factor;
                factor <<= 1;
            }
        }
    
        return rating;
    }    
}

#[derive(Debug)]
struct Tile {
    level: i32,
    position: Position,
}

impl Tile {
    fn adjacent_tiles_finite(&self) -> Vec<Tile> {
        let mut tiles = vec!();

        for direction in vec!(Direction::Up, Direction::Right, Direction::Down, Direction::Left) {
            match direction {
                Direction::Up => {
                    if self.position.y == 0 {
                        continue;
                    }
                }
                Direction::Right => {
                    if self.position.x == 4 {
                        continue;
                    }
                }
                Direction::Down => {
                    if self.position.y == 4 {
                        continue;
                    }
                }
                Direction::Left => {
                    if self.position.x == 0 {
                        continue;
                    }
                }
            }

            let stepped = self.position.step(&direction);
            tiles.push(Tile {
                level: self.level,
                position: stepped,
            });
        }

        tiles
    }

    fn adjacent_tiles_infinite(&self) -> Vec<Tile> {
        let mut tiles = vec!();

        if self.position.x == 2 && self.position.y == 2 {
            return tiles;
        }

        for direction in vec!(Direction::Up, Direction::Right, Direction::Down, Direction::Left) {
            match direction {
                Direction::Up => {
                    match self.position.y {
                        0 => {
                            tiles.push(Tile {
                                level: self.level - 1,
                                position: Position { x: 2, y: 1 },
                            });
                        }
                        3 => {
                            match self.position.x {
                                2 => {
                                    for i in 0..5 {
                                        tiles.push(Tile {
                                            level: self.level + 1,
                                            position: Position { x: i, y: 4 },
                                        });
                                    }                                    
                                }
                                _ => {
                                    let stepped = self.position.step(&direction);
                                    tiles.push(Tile {
                                        level: self.level,
                                        position: stepped,
                                    });                
                                }
                            }
                        }
                        _ => {
                            let stepped = self.position.step(&direction);
                            tiles.push(Tile {
                                level: self.level,
                                position: stepped,
                            });                
                        }
                    }
                }
                Direction::Right => {
                    match self.position.x {
                        4 => {
                            tiles.push(Tile {
                                level: self.level - 1,
                                position: Position { x: 3, y: 2 },
                            });
                        }
                        1 => {
                            match self.position.y {
                                2 => {
                                    for i in 0..5 {
                                        tiles.push(Tile {
                                            level: self.level + 1,
                                            position: Position { x: 0, y: i },
                                        });
                                    }                                    
                                }
                                _ => {
                                    let stepped = self.position.step(&direction);
                                    tiles.push(Tile {
                                        level: self.level,
                                        position: stepped,
                                    });                
                                }
                            }
                        }
                        _ => {
                            let stepped = self.position.step(&direction);
                            tiles.push(Tile {
                                level: self.level,
                                position: stepped,
                            });                
                        }
                    }
                }
                Direction::Down => {
                    match self.position.y {
                        4 => {
                            tiles.push(Tile {
                                level: self.level - 1,
                                position: Position { x: 2, y: 3 },
                            });
                        }
                        1 => {
                            match self.position.x {
                                2 => {
                                    for i in 0..5 {
                                        tiles.push(Tile {
                                            level: self.level + 1,
                                            position: Position { x: i, y: 0 },
                                        });
                                    }                                    
                                }
                                _ => {
                                    let stepped = self.position.step(&direction);
                                    tiles.push(Tile {
                                        level: self.level,
                                        position: stepped,
                                    });                
                                }
                            }
                        }
                        _ => {
                            let stepped = self.position.step(&direction);
                            tiles.push(Tile {
                                level: self.level,
                                position: stepped,
                            });                
                        }
                    }
                }
                Direction::Left => {
                    match self.position.x {
                        0 => {
                            tiles.push(Tile {
                                level: self.level - 1,
                                position: Position { x: 1, y: 2 },
                            });
                        }
                        3 => {
                            match self.position.y {
                                2 => {
                                    for i in 0..5 {
                                        tiles.push(Tile {
                                            level: self.level + 1,
                                            position: Position { x: 4, y: i },
                                        });
                                    }                                    
                                }
                                _ => {
                                    let stepped = self.position.step(&direction);
                                    tiles.push(Tile {
                                        level: self.level,
                                        position: stepped,
                                    });                
                                }
                            }
                        }
                        _ => {
                            let stepped = self.position.step(&direction);
                            tiles.push(Tile {
                                level: self.level,
                                position: stepped,
                            });                
                        }
                    }
                }
            }
        }

        tiles
    }

    fn adjacent_tiles(&self, infinite: bool) -> Vec<Tile> {
        if infinite {
            self.adjacent_tiles_infinite()
        } else {
            self.adjacent_tiles_finite()
        }
    }
}

fn run_timestep(mut eris: HashMap<i32, Grid>, infinite: bool) -> HashMap<i32, Grid> {
    if infinite {
        let min_level = eris.keys().min().unwrap().clone();
        eris.insert(min_level - 1, Grid(vec![vec![0; 5]; 5]));

        let max_level = eris.keys().max().unwrap().clone();
        eris.insert(max_level + 1, Grid(vec![vec![0; 5]; 5]));

        let mut new_eris = HashMap::new();
        for (level, grid) in eris.iter() {
            let new_grid = Grid(grid.0.iter().enumerate().map(|(y, row)| {
                row.iter().enumerate().map(|(x, bugs)| {
                    let adjacent_bugs: u32 = Tile {
                        level: *level,
                        position: Position { x, y }
                    }.adjacent_tiles(true).iter().map(|tile| {
                        match eris.get(&tile.level) {
                            Some(g) => g.0[tile.position.y][tile.position.x],
                            None => 0,
                        }
                    }).sum();
                    match bugs {
                        0 => {
                            if adjacent_bugs == 1 || adjacent_bugs == 2 {
                                1
                            } else {
                                0
                            }
                        },
                        1 => {
                            if adjacent_bugs != 1 {
                                0
                            } else {
                                1
                            }
                        },
                        _ => unreachable!()
                    }
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>());
        
            new_eris.insert(*level, new_grid);
        }

        new_eris
    } else {
        if let Some(grid) = eris.get(&0) {
            let new_grid = Grid(grid.0.iter().enumerate().map(|(y, row)| {
                row.iter().enumerate().map(|(x, bugs)| {
                    let adjacent_bugs: u32 = Tile {
                        level: 0,
                        position: Position { x, y }
                    }.adjacent_tiles(false).iter().map(|tile| grid.0[tile.position.y][tile.position.x]).sum();
                    match bugs {
                        0 => {
                            if adjacent_bugs == 1 || adjacent_bugs == 2 {
                                1
                            } else {
                                0
                            }
                        },
                        1 => {
                            if adjacent_bugs != 1 {
                                0
                            } else {
                                1
                            }
                        },
                        _ => unreachable!()
                    }
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>());
        
            eris.insert(0, new_grid);
        }
        eris
    }
}

fn main() {
    let stdin = std::io::stdin();
    let level0 = Grid(stdin.lock().lines().map(|row| {
        row.unwrap().chars().map(|character| match character {
            '.' => 0,
            '#' => 1,
            _ => unreachable!(),
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>());

    // level0.display();

    // part 1
    let mut eris = HashMap::new();
    eris.insert(0, level0.clone());
    let mut history = HashMap::new();
    let mut step = 0;
    loop {
        let biodiversity = eris.get(&0).unwrap().biodiversity();
        if history.contains_key(&biodiversity) {
            break;
        }

        history.insert(biodiversity, step);

        eris = run_timestep(eris, false);
        step += 1;
    }

    // eris.get(&0).unwrap().display();
    println!("{}", eris.get(&0).unwrap().biodiversity());


    // part 2
    let mut eris = HashMap::new();
    eris.insert(0, level0.clone());
    for _step in 0..200 {
        eris = run_timestep(eris, true);
    }

    let bugs: u32 = eris.values().map(|grid| {
        grid.0.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>()
    }).sum();

    // for (level, grid) in eris.iter() {
    //     if level.abs() > 6 {
    //         continue;
    //     }
    //     println!("{}", level);
    //     grid.display();
    // }
    println!("{}", bugs);
}

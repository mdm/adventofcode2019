use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position {
    x: i64,
    y: i64,
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
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_id(&self) -> i64 {
        match self {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Right => 4,
        }
    }

    fn left_hand_side(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn right_hand_side(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug)]
enum Status {
    HitWall,
    Moved,
    FoundTarget,
}

impl Status {
    fn from_id(id: i64) -> Status {
        match id {
            0 => Status::HitWall,
            1 => Status::Moved,
            2 => Status::FoundTarget,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
enum MapTile {
    Robot,
    Wall,
    Floor,
    Start,
    Target,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    orientation: Direction,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            position: Position { x: 0, y: 0 },
            orientation: Direction::Up,
        }
    }

    fn move_forward(&mut self, pretend: bool) -> Position {
        let new_position = self.position.step(&self.orientation);

        if !pretend {
            self.position = new_position.clone();
        }

        new_position
    }

    fn turn(&mut self, last_status: &Status) -> i64 {
        self.orientation = match last_status {
            Status::HitWall => self.orientation.left_hand_side(),
            Status::Moved => self.orientation.right_hand_side(),
            Status::FoundTarget => self.orientation.right_hand_side(),
        };

        self.orientation.to_id()
    }
}

#[derive(Debug)]
struct Context {
    program: Vec<i64>,
    pc: usize,
    inputs: VecDeque<i64>,
    relative_base: usize,
    heap: HashMap<usize, i64>,
}

fn read_helper(context: &mut Context, address: usize) -> i64 {
    if address >= context.program.len() {
        match context.heap.get(&address) {
            Some(value) => {
                return *value;
            }
            None => {
                context.heap.insert(address, 0);
                return 0;
            }
        }
    } else {
        return context.program[address];
    }
}

fn write_helper(context: &mut Context, address: usize, value: i64) {
    if address >= context.program.len() {
        context.heap.insert(address, value);
    } else {
        context.program[address] = value;
    }
}

fn read(context: &mut Context, offset: u32) -> i64 {
    let mode = context.program[context.pc] / (10i64.pow(offset + 1)) % 10;
    // dbg!(mode);

    match mode {
        0 => {
            let address = context.program[context.pc + offset as usize] as usize;
            return read_helper(context, address);
        }
        1 => {
            return context.program[context.pc + offset as usize];
        }
        2 => {
            let address = context.program[context.pc + offset as usize];
            return read_helper(context, (context.relative_base as i64 + address) as usize);
        }
       _ => panic!("Illegal mode flag."),
    }
}

fn write(context: &mut Context, offset: u32, value: i64) {
    let mode = context.program[context.pc] / (10i64.pow(offset + 1)) % 10;

    match mode {
        0 => {
            let address = context.program[context.pc + offset as usize] as usize;
            write_helper(context, address, value);
        }
        2 => {
            let address = context.program[context.pc + offset as usize];
            write_helper(context, (context.relative_base as i64 + address) as usize, value);
        }
       _ => panic!("Illegal mode flag."),
    }
}

fn run(mut context: &mut Context) -> Option<i64> {
    while context.program[context.pc] != 99 {
        // println!("{}: {}", context.pc, context.program[context.pc]);
        match context.program[context.pc] % 100 {
            1 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);
                write(&mut context, 3, operand1 + operand2);

                context.pc += 4;
            }
            2 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);
                write(&mut context, 3, operand1 * operand2);

                context.pc += 4;
            }
            3 => {
                let input = match context.inputs.pop_front() {
                    Some(value) => value,
                    None => 0,
                };

                write(&mut context, 1, input);

                context.pc += 2;
            }
            4 => {
                let operand = read(&mut context, 1);

                context.pc += 2;
                return Some(operand);
            }
            5 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);

                if operand1 != 0 {
                    context.pc = operand2 as usize;
                } else {
                    context.pc += 3;
                }
            }
            6 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);

                if operand1 == 0 {
                    context.pc = operand2 as usize;
                } else {
                    context.pc += 3;
                }
            }
            7 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);

                if operand1 < operand2 {
                    write(&mut context, 3, 1);
                } else {
                    write(&mut context, 3, 0);
                }
                context.pc += 4;
            }
            8 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);

                if operand1 == operand2 {
                    write(&mut context, 3, 1);
                } else {
                    write(&mut context, 3, 0);
                }
                context.pc += 4;
            }
            9 => {
                let operand = read(&mut context, 1);

                context.relative_base = (context.relative_base as i64 + operand) as usize;
                context.pc += 2;
            }
            _ => panic!("Illegal operation."),
        }
    }

    return None;
}

fn display(maze: &HashMap::<Position, MapTile>) {

    let min_x = maze.iter().fold(std::i64::MAX, |min, panel| {
        std::cmp::min(min, panel.0.x)
    });

    let max_x = maze.iter().fold(std::i64::MIN, |max, panel| {
        std::cmp::max(max, panel.0.x)
    });

    let min_y = maze.iter().fold(std::i64::MAX, |min, panel| {
        std::cmp::min(min, panel.0.y)
    });

    let max_y = maze.iter().fold(std::i64::MIN, |max, panel| {
        std::cmp::max(max, panel.0.y)
    });

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let snapshot = (0..height).map(|y| {
        (0..width).map(|x| {
            let position = Position { x: x + min_x, y: y + min_y };
            match maze.get(&position) {
                Some(color) => {
                    match color {
                        MapTile::Robot => '*',
                        MapTile::Wall => '#',
                        MapTile::Target => 'X',
                        MapTile::Start => 'O',
                        MapTile::Floor => ' ',
                    }
                }
                None => {
                    ' '
                }
            }
        }).collect::<String>()
    }).collect::<Vec<String>>().join("\n");

    println!("{}\n---", snapshot);
}

fn shortest_path(maze: &HashMap::<Position, MapTile>, start: &Position, target: &Position) -> i64 {
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut work = VecDeque::new();

    visited.insert(start.clone());
    distances.insert(start.clone(), 0);
    work.push_back(start.clone());

    let mut max_distance = 0;
    while !work.is_empty() {
        let current_position = work.pop_front().unwrap();

        let current_distance = *distances.get(&current_position).unwrap();

        if current_position == *target {
            return current_distance;
        }

        if current_distance > max_distance {
            max_distance = current_distance;
        }

        let new_position = current_position.step(&Direction::Up);
        if !visited.contains(&new_position) && maze.get(&new_position).unwrap() != &MapTile::Wall {
            visited.insert(new_position.clone());
            distances.insert(new_position.clone(), distances.get(&current_position).unwrap() + 1);
            work.push_back(new_position);
        }

        let new_position = current_position.step(&Direction::Down);
        if !visited.contains(&new_position) && maze.get(&new_position).unwrap() != &MapTile::Wall {
            visited.insert(new_position.clone());
            distances.insert(new_position.clone(), distances.get(&current_position).unwrap() + 1);
            work.push_back(new_position);
        }

        let new_position = current_position.step(&Direction::Left);
        if !visited.contains(&new_position) && maze.get(&new_position).unwrap() != &MapTile::Wall {
            visited.insert(new_position.clone());
            distances.insert(new_position.clone(), distances.get(&current_position).unwrap() + 1);
            work.push_back(new_position);
        }

        let new_position = current_position.step(&Direction::Right);
        if !visited.contains(&new_position) && maze.get(&new_position).unwrap() != &MapTile::Wall {
            visited.insert(new_position.clone());
            distances.insert(new_position.clone(), distances.get(&current_position).unwrap() + 1);
            work.push_back(new_position);
        }
    }

    return max_distance;
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();
    let mut program = Vec::new();
    for token in input.split(",") {
        program.push(token.parse::<i64>().expect(token));
    }

    let mut context = Context {
        program: program.clone(),
        pc: 0,
        inputs: VecDeque::new(),
        relative_base: 0,
        heap: HashMap::new(),
    };

    let mut maze = HashMap::<Position, MapTile>::new();
    let mut robot = Robot::new();
    let start = Position { x: 0, y: 0 };
    let mut target = start.clone();
    maze.insert(start.clone(), MapTile::Start);
    context.inputs.push_back(robot.turn(&Status::Moved));
    loop {
        // if steps % 100 == 0 {
        //     maze.insert(robot.position.clone(), MapTile::Robot);
        //     display(&maze);
        //     maze.insert(robot.position.clone(), MapTile::Floor);
        //     maze.insert(start.clone(), MapTile::Start);
        // }

        // dbg!(&robot);

        let status = match run(&mut context) {
            Some(output) => {
                Status::from_id(output)
            }
            None => {
                break;
            }
        };

        // dbg!(&robot, &status);

        match status {
            Status::HitWall => {
                maze.insert(robot.move_forward(true), MapTile::Wall);
            }
            Status::Moved => {
                maze.insert(robot.move_forward(false), MapTile::Floor);
                if robot.position == start {
                    maze.insert(robot.position.clone(), MapTile::Start);
                    break;
                }
            }
            Status::FoundTarget => {
                maze.insert(robot.move_forward(false), MapTile::Target);
                target = robot.position.clone();
            }
        }


        context.inputs.push_back(robot.turn(&status));
    }

    // display(&maze);
    // dbg!(&robot.position, &steps, maze.get(&target));

    println!("{}", shortest_path(&maze, &start, &target));
    println!("{}", shortest_path(&maze, &target, &Position { x: 1000, y: 1000 }));
}

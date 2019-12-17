use std::collections::HashMap;
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

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_ascii(ascii_code: i64) -> Direction {
        match ascii_code {
            94 => Direction::Up,
            62 => Direction::Right,
            118 => Direction::Down,
            60 => Direction::Left,
            _ => unreachable!(),
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

#[derive(PartialEq, Debug, Clone)]
enum Command {
    MoveForward,
    TurnLeft,
    TurnRight,
}

#[derive(PartialEq, Debug)]
enum MapTile {
    Robot,
    Scaffold,
    Intersection,
    OpenSpace,
}

#[derive(Debug, Clone)]
struct Robot {
    position: Position,
    orientation: Direction,
}

impl Robot {
    fn new(position: Position, orientation: Direction) -> Robot {
        Robot {
            position,
            orientation,
        }
    }

    fn move_forward(&mut self, pretend: bool) -> Position {
        let new_position = self.position.step(&self.orientation);

        if !pretend {
            self.position = new_position.clone();
        }

        new_position
    }

    fn turn_left(&mut self) -> Direction {
        self.orientation = self.orientation.left_hand_side();
        self.orientation.clone()
    }

    fn turn_right(&mut self) -> Direction {
        self.orientation = self.orientation.right_hand_side();
        self.orientation.clone()
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

fn display(map: &HashMap::<Position, MapTile>) {

    let min_x = map.iter().fold(std::i64::MAX, |min, panel| {
        std::cmp::min(min, panel.0.x)
    });

    let max_x = map.iter().fold(std::i64::MIN, |max, panel| {
        std::cmp::max(max, panel.0.x)
    });

    let min_y = map.iter().fold(std::i64::MAX, |min, panel| {
        std::cmp::min(min, panel.0.y)
    });

    let max_y = map.iter().fold(std::i64::MIN, |max, panel| {
        std::cmp::max(max, panel.0.y)
    });

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let snapshot = (0..height).map(|y| {
        (0..width).map(|x| {
            let position = Position { x: x + min_x, y: y + min_y };
            match map.get(&position) {
                Some(color) => {
                    match color {
                        MapTile::Robot => '*',
                        MapTile::Scaffold => '#',
                        MapTile::Intersection => 'O',
                        MapTile::OpenSpace => '.',
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

fn all_paths(
    map: &HashMap::<Position, MapTile>,
    robot: Robot,
    visited: &HashMap<Position, u8>,
    path_so_far: Vec<Command>
) -> Vec<Vec<Command>> {
    let mut open_directions = 0;
    let mut paths_taken = Vec::new();

    let mut new_visited = visited.clone();
    // if new_visited.contains_key(&robot.position) {
    //     dbg!(
    //         &robot.position,
    //         map.get(&robot.position)
    //     );
    // }
    *new_visited.entry(robot.position.clone()).or_insert(0) += 1;

    let mut new_robot = robot.clone();
    let new_position = new_robot.move_forward(false);
    if (map.get(&new_position) == Some(&MapTile::Scaffold) && !new_visited.contains_key(&new_position))
    || (map.get(&new_position) == Some(&MapTile::Intersection) && new_visited.get(&new_position) != Some(&2)) {
        open_directions += 1;
        let mut new_path_so_far = path_so_far.clone();
        new_path_so_far.push(Command::MoveForward);
        paths_taken.extend(all_paths(map, new_robot, &new_visited, new_path_so_far));
    }

    let mut new_robot = robot.clone();
    new_robot.turn_left();
    let new_position = new_robot.move_forward(false);
    if (map.get(&new_position) == Some(&MapTile::Scaffold) && !new_visited.contains_key(&new_position))
    || (map.get(&new_position) == Some(&MapTile::Intersection) && new_visited.get(&new_position) != Some(&2)) {
        open_directions += 1;
        let mut new_path_so_far = path_so_far.clone();
        new_path_so_far.push(Command::TurnLeft);
        new_path_so_far.push(Command::MoveForward);
        paths_taken.extend(all_paths(map, new_robot, &new_visited, new_path_so_far));
    }

    let mut new_robot = robot.clone();
    new_robot.turn_right();
    let new_position = new_robot.move_forward(false);
    if (map.get(&new_position) == Some(&MapTile::Scaffold) && !new_visited.contains_key(&new_position))
    || (map.get(&new_position) == Some(&MapTile::Intersection) && new_visited.get(&new_position) != Some(&2)) {
        open_directions += 1;
        let mut new_path_so_far = path_so_far.clone();
        new_path_so_far.push(Command::TurnRight);
        new_path_so_far.push(Command::MoveForward);
        paths_taken.extend(all_paths(map, new_robot, &new_visited, new_path_so_far));
    }

    if open_directions == 0 {
        paths_taken.push(path_so_far);
    }

    return paths_taken;
}

fn path_length(path: &Vec<Command>) -> usize {
    path.iter().filter(|command| *command == &Command::MoveForward).count()
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

    let mut map = HashMap::<Position, MapTile>::new();
    let mut cursor = Position { x: 0, y: 0 };
    let mut robot = None;
    let mut width = 0;
    let mut height = 0;
    loop {
        let ascii_code = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        match ascii_code {
            35 => {
                map.insert(cursor.clone(), MapTile::Scaffold);
                cursor.x += 1;
            }
            46 => {
                map.insert(cursor.clone(), MapTile::OpenSpace);
                cursor.x += 1;
            }
            10 => {
                cursor.x = 0;
                cursor.y += 1;
            }
            _ => {
                robot = Some(Robot::new(cursor.clone(), Direction::from_ascii(ascii_code)));
                cursor.x += 1;
            }
        }

        width = std::cmp::max(width, cursor.x);
        height = std::cmp::max(height, cursor.y);
    }

    let mut alignment_sum = 0;
    let mut scaffold_length = 1;
    for y in 0..height {
        for x in 0..width {
            let position = Position { x, y };
            if map.get(&position) != Some(&MapTile::Scaffold) {
                continue;
            }

            scaffold_length += 1;

            if map.get(&position.step(&Direction::Up)) == Some(&MapTile::Scaffold)
            && map.get(&position.step(&Direction::Right)) == Some(&MapTile::Scaffold)
            && map.get(&position.step(&Direction::Down)) == Some(&MapTile::Scaffold)
            && map.get(&position.step(&Direction::Left)) == Some(&MapTile::Scaffold) {
                map.insert(position.clone(), MapTile::Intersection);
                alignment_sum += x * y;
            }
        }
    }

    display(&map);
    println!("{}", alignment_sum);
    println!("{}", scaffold_length);

    let mut context = Context {
        program: program.clone(),
        pc: 0,
        inputs: VecDeque::new(),
        relative_base: 0,
        heap: HashMap::new(),
    };

    context.program[0] = 2;

    let paths = all_paths(&map, robot.unwrap(), &HashMap::new(), Vec::new());

    let max_length = paths.iter().map(|path| path_length(path)).fold(0, |max, length| std::cmp::max(max, length));
    let candidates = paths.iter().filter(|path| path_length(path) == max_length).collect::<Vec<_>>();
    dbg!(max_length, paths.len(), candidates.len());
}

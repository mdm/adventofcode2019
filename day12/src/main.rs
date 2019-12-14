#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn step_time(moons: &mut Vec<Moon>) {
    for a in 0..moons.len() {
        for b in 0..moons.len() {
            if b >= a {
                continue;
            }

            match moons[a].position.x.cmp(&moons[b].position.x) {
                std::cmp::Ordering::Less => {
                    moons[a].velocity.x += 1;
                    moons[b].velocity.x -= 1;
                }
                std::cmp::Ordering::Greater => {
                    moons[a].velocity.x -= 1;
                    moons[b].velocity.x += 1;
                }
                std::cmp::Ordering::Equal => {}
            }

            match moons[a].position.y.cmp(&moons[b].position.y) {
                std::cmp::Ordering::Less => {
                    moons[a].velocity.y += 1;
                    moons[b].velocity.y -= 1;
                }
                std::cmp::Ordering::Greater => {
                    moons[a].velocity.y -= 1;
                    moons[b].velocity.y += 1;
                }
                std::cmp::Ordering::Equal => {}
            }

            match moons[a].position.z.cmp(&moons[b].position.z) {
                std::cmp::Ordering::Less => {
                    moons[a].velocity.z += 1;
                    moons[b].velocity.z -= 1;
                }
                std::cmp::Ordering::Greater => {
                    moons[a].velocity.z -= 1;
                    moons[b].velocity.z += 1;
                }
                std::cmp::Ordering::Equal => {}
            }
        }
    }

    for i in 0..moons.len() {
        moons[i].position.x += moons[i].velocity.x;
        moons[i].position.y += moons[i].velocity.y;
        moons[i].position.z += moons[i].velocity.z;
    }
}

fn main() {
    let regex = regex::Regex::new(r"<x=(.+), y=(.+), z=(.+)>").unwrap();
    let mut original_moons = Vec::new();
    for _ in 0..4 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim_end().to_string();

        let captures = regex.captures(&input).unwrap();
        let moon = Moon {
            position: Vector {
                x: captures[1].parse::<i32>().unwrap(),
                y: captures[2].parse::<i32>().unwrap(),
                z: captures[3].parse::<i32>().unwrap(),
            },
            velocity: Vector {
                x: 0,
                y: 0,
                z: 0,
            }
        };
        original_moons.push(moon);
    }

    let mut moons = original_moons.clone();
    for _ in 0..1000 {
        step_time(&mut moons);
    }

    let mut sum_total = 0;
    for i in 0..moons.len() {
        let pot = moons[i].position.x.abs() + moons[i].position.y.abs() + moons[i].position.z.abs();
        let kin = moons[i].velocity.x.abs() + moons[i].velocity.y.abs() + moons[i].velocity.z.abs();
        sum_total += pot * kin;
    }

    println!("{}", sum_total);


    let mut periods = Vec::new();
    let mut firsts = Vec::new();
    let mut moons = original_moons.clone();
    let mut history = std::collections::HashMap::new();
    let mut steps = 0 as i64;
    loop {
        let mut xs = moons.iter().map(|moon| moon.position.x).collect::<Vec<i32>>();
        xs.extend(moons.iter().map(|moon| moon.velocity.x).collect::<Vec<i32>>());
        history.insert(xs, steps);

        step_time(&mut moons);

        steps += 1;

        let mut xs = moons.iter().map(|moon| moon.position.x).collect::<Vec<i32>>();
        xs.extend(moons.iter().map(|moon| moon.velocity.x).collect::<Vec<i32>>());
        // dbg!(&xs);
        if let Some(first) = history.get(&xs) {
            periods.push(steps - first);
            firsts.push(first.clone());
            break;
        }
    }

    let mut moons = original_moons.clone();
    let mut history = std::collections::HashMap::new();
    let mut steps = 0 as i64;
    loop {
        let mut ys = moons.iter().map(|moon| moon.position.y).collect::<Vec<i32>>();
        ys.extend(moons.iter().map(|moon| moon.velocity.y).collect::<Vec<i32>>());
        history.insert(ys, steps);

        step_time(&mut moons);

        steps += 1;

        let mut ys = moons.iter().map(|moon| moon.position.y).collect::<Vec<i32>>();
        ys.extend(moons.iter().map(|moon| moon.velocity.y).collect::<Vec<i32>>());
        // dbg!(&ys);
        if let Some(first) = history.get(&ys) {
            periods.push(steps - first);
            firsts.push(first.clone());
            break;
        }
    }

    let mut moons = original_moons.clone();
    let mut history = std::collections::HashMap::new();
    let mut steps = 0 as i64;
    loop {
        let mut zs = moons.iter().map(|moon| moon.position.z).collect::<Vec<i32>>();
        zs.extend(moons.iter().map(|moon| moon.velocity.z).collect::<Vec<i32>>());
        history.insert(zs, steps);

        step_time(&mut moons);

        steps += 1;

        let mut zs = moons.iter().map(|moon| moon.position.z).collect::<Vec<i32>>();
        zs.extend(moons.iter().map(|moon| moon.velocity.z).collect::<Vec<i32>>());
        // dbg!(&zs);
        if let Some(first) = history.get(&zs) {
            periods.push(steps - first);
            firsts.push(first.clone());
            break;
        }
    }

    // dbg!(&periods);
    // dbg!(&firsts);

    let steps = periods.iter().fold(1, |acc, period| {
        // dbg!(lcm(acc, *period));
        lcm(acc, *period)
    });
    println!("{}", steps);
}

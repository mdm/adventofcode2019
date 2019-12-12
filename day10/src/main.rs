use std::io::BufRead;

#[derive(Debug, PartialEq, Clone)]
struct Asteroid {
    x: i32,
    y: i32,
}

fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    return a;
}

fn can_detect(station: &Asteroid, target: &Asteroid, asteroids: &Vec<Asteroid>) -> bool {
    if target == station {
        return false;
    }

    if target.x == station.x {
        for obstacle in asteroids {
            if obstacle == target || obstacle == station {
                continue;
            }

            let same_half_y = (station.y - target.y).signum() == (station.y - obstacle.y).signum();

            if !same_half_y {
                continue;
            }

            if obstacle.x == target.x && (station.y - obstacle.y).abs() < (station.y - target.y).abs() {
                return false;
            }
        }

        return true;
    }

    if target.y == station.y {
        for obstacle in asteroids {
            if obstacle == target || obstacle == station {
                continue;
            }

            let same_half_x = (station.x - target.x).signum() == (station.x - obstacle.x).signum();

            if !same_half_x {
                continue;
            }

            if obstacle.y == target.y && (station.x - obstacle.x).abs() < (station.x - target.x).abs() {
                return false;
            }
        }

        return true;
    }

    let target_distance_x = (station.x - target.x).abs() as u32;
    let target_distance_y = (station.y - target.y).abs() as u32;

    for obstacle in asteroids {
        if obstacle == target || obstacle == station {
            continue;
        }

        let same_half_x = (station.x - target.x).signum() == (station.x - obstacle.x).signum();
        let same_half_y = (station.y - target.y).signum() == (station.y - obstacle.y).signum();

        if !same_half_x || !same_half_y {
            continue;
        }

        let obstacle_distance_x = (station.x - obstacle.x).abs() as u32;
        let obstacle_distance_y = (station.y - obstacle.y).abs() as u32;

        let target_distance_2 = target_distance_x * target_distance_x + target_distance_y * target_distance_y;
        let obstacle_distance_2 = obstacle_distance_x * obstacle_distance_x + obstacle_distance_y * obstacle_distance_y;

        if obstacle_distance_2 >= target_distance_2 {
            continue;
        }

        let gcd_x = gcd(target_distance_x, obstacle_distance_x);
        let gcd_y = gcd(target_distance_y, obstacle_distance_y);

        let obstacle_on_line = obstacle_distance_x / gcd_x == obstacle_distance_y / gcd_y;
        let target_on_line = target_distance_x / gcd_x == target_distance_y / gcd_y;

        if obstacle_on_line && target_on_line {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut asteroids = Vec::<Asteroid>::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, character) in line.unwrap().trim_end().chars().enumerate() {
            if character == '#' {
                asteroids.push(Asteroid { x: x as i32, y: y as i32 });
            }
        }
    }

    let mut max_detected = 0;
    let mut argmax = &asteroids[0];
    for station in &asteroids {
        let mut detected = 0;

        for target in &asteroids {
            if can_detect(station, target, &asteroids) {
                detected += 1;
            }
        }

        if detected > max_detected {
            max_detected = detected;
            argmax = station;
        }
    }

    println!("{}", max_detected);

    let mut detectable = Vec::<Asteroid>::new();
    for target in &asteroids {
        if can_detect(argmax, target, &asteroids) {
            detectable.push((*target).clone());
        }
    }

    // argmax = &Asteroid { x: 8, y: 3 };
    // let mut detectable = vec!(
    //     Asteroid { x: 9, y: 0 },
    //     Asteroid { x: 10, y: 0 },
    //     Asteroid { x: 8, y: 1 },
    //     Asteroid { x: 9, y: 1 },
    //     Asteroid { x: 11, y: 1 },
    //     Asteroid { x: 12, y: 1 },
    //     Asteroid { x: 15, y: 1 },
    //     Asteroid { x: 9, y: 2 },
    //     Asteroid { x: 11, y: 2 }
    // );

    detectable.sort_by(|a, b| {
        let a_x = (a.y - argmax.y) as f64;
        let a_y = (a.x - argmax.x) as f64;
        let score_a = std::f64::consts::PI - (a_y).atan2(a_x);

        let b_x = (b.y - argmax.y) as f64;
        let b_y = (b.x - argmax.x) as f64;
        let score_b = std::f64::consts::PI - (b_y).atan2(b_x);

        score_a.partial_cmp(&score_b).unwrap()
    });

    // dbg!(detectable);

    // dbg!(can_detect(argmax, argmax, &asteroids), detectable.len(), &detectable[198], &detectable[199], &detectable[200]);
    println!("{}", detectable[199].x * 100 + detectable[199].y);
}

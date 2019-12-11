use std::io::BufRead;

#[derive(Debug, PartialEq)]
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
    for station in &asteroids {
        let mut blocked = 0;

        'target: for target in &asteroids {
            if target.x == station.x {
                for obstacle in &asteroids {
                    if obstacle == target || obstacle == station {
                        continue;
                    }

                    let same_half_y = (station.y - target.y).signum() == (station.y - obstacle.y).signum();

                    if !same_half_y {
                        continue;
                    }

                    if obstacle.x == target.x && (station.y - obstacle.y).abs() < (station.y - target.y).abs() {
                        blocked += 1;
                        continue 'target;
                    }
                }

                continue 'target;
            }

            if target.y == station.y {
                for obstacle in &asteroids {
                    if obstacle == target || obstacle == station {
                        continue;
                    }

                    let same_half_x = (station.x - target.x).signum() == (station.x - obstacle.x).signum();

                    if !same_half_x {
                        continue;
                    }

                    if obstacle.y == target.y && (station.x - obstacle.x).abs() < (station.x - target.x).abs() {
                        blocked += 1;
                        continue 'target;
                    }
                }

                continue 'target;
            }

            let target_distance_x = (station.x - target.x).abs() as u32;
            let target_distance_y = (station.y - target.y).abs() as u32;

            for obstacle in &asteroids {
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
                    blocked += 1;
                    continue 'target;
                }
            }
        }

        if asteroids.len() - blocked - 1 > max_detected {
            max_detected = asteroids.len() - blocked - 1;
        }
    }

    println!("{}", asteroids.len());
    println!("{}", max_detected);
}

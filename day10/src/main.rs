use std::io::BufRead;

#[derive(Debug)]
struct Asteroid {
    x: i32,
    y: i32,
}

fn main() {
    let mut asteroids = Vec::<Asteroid>::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, character) in line.unwrap().trim_end().chars().enumerate() {
            if character == '#' {
                asteroids.push(Asteroid { x: x as i32, y: y as i32 });
                // dbg!(asteroids.last());
            }
        }
    }

    let mut max_detected = 0;
    for station in &asteroids {
        let mut blocked = 0;

        // if !(station.x == 4 <&& station.y == 0) {
        //     continue;
        // }

        // dbg!(station);

        for target in &asteroids {
            if target.x < station.x || target.y < station.y {
                continue;
            }

            // dbg!(target);

            if target.x == station.x {
                'loop1: for y in (station.y + 1)..target.y {
                    for obstacle in &asteroids {
                        if obstacle.x == target.x && obstacle.y == y {
                            // println!("BLOCKED 1");
                            // dbg!(obstacle);
                            blocked += 1;
                            break 'loop1;
                        }
                    }
                }
            }

            if target.y == station.y {
                'loop2: for x in (station.x + 1)..target.x {
                    for obstacle in &asteroids {
                        if obstacle.x == x && obstacle.y == target.y {
                            // println!("BLOCKED 2");
                            // dbg!(obstacle);
                            blocked += 1;
                            break 'loop2;
                        }
                    }
                }
            }

            'loop3: for x in 1..(target.x - station.x) {
                if (x * (target.y - station.y)) % (target.x - station.x) == 0 {
                    for obstacle in &asteroids {
                        if obstacle.x == station.x + x && obstacle.y == (x * (target.y - station.y)) / (target.x - station.x) {
                            // println!("BLOCKED 3");
                            // dbg!(x, target.y - station.y, target.x - station.x);
                            blocked += 1;
                            break 'loop3;
                        }
                    }
                }
            }
        }

        dbg!(station, asteroids.len() - blocked - 1);
        if asteroids.len() - blocked - 1 > max_detected {
            max_detected = asteroids.len() - blocked - 1;
        }
    }

    println!("{}", asteroids.len());
    println!("{}", max_detected);
}

use std::io::BufRead;
use std::collections::HashMap;

fn count_orbits(orbit_tree: &HashMap<String, Vec<String>>, center: &str, depth: u32) -> u32 {
    let mut orbits = depth;

    if let Some(satellites) = orbit_tree.get(center) {
        for satellite in satellites {
            orbits += count_orbits(orbit_tree, satellite, depth + 1);
        }
    }

    return orbits;
}

#[derive(Debug)]
struct TransferInfo {
    you: u32,
    santa: u32,
    parent: u32,
}

fn count_transfers(
    orbit_tree: &HashMap<String, Vec<String>>,
    center: &str,
    depth: u32,
) -> TransferInfo {
    if center == "YOU" {
        return TransferInfo { you: depth, santa: 0, parent: 0 };
    }

    if center == "SAN" {
        return TransferInfo { you: 0, santa: depth, parent: 0 };
    }

    let mut you = 0;
    let mut santa = 0;
    let mut parent = 0;

    if let Some(satellites) = orbit_tree.get(center) {
        for satellite in satellites {
            let transfer_info = count_transfers(orbit_tree, satellite, depth + 1);

            if transfer_info.you > 0 {
                you = transfer_info.you;
            }

            if transfer_info.santa > 0 {
                santa = transfer_info.santa;
            }

            if transfer_info.parent > 0 {
                parent = transfer_info.parent;
            }

            if you > 0 && santa > 0 && depth > parent {
                return TransferInfo { you, santa, parent: depth };
            }
        }
    }

    return TransferInfo { you, santa, parent };
}

fn main() {
    let mut orbit_tree = HashMap::<String, Vec<String>>::new();

    for line in std::io::stdin().lock().lines() {
        let raw_line = line.unwrap();
        let mut split = raw_line.split(")");
        let center = split.next().unwrap().to_string();
        let satellite = split.next().unwrap().to_string();

        // dbg!(&center, &satellite);
        match orbit_tree.get_mut(&center) {
            Some(satellites) => {
                satellites.push(satellite);
            }
            None => {
                orbit_tree.insert(center, vec!(satellite));
            }
        }
    }

    // dbg!(&orbit_tree);
    let orbits = count_orbits(&orbit_tree, "COM", 0);
    println!("{}", orbits);
    let transfer_info = count_transfers(&orbit_tree, "COM", 0);
    // dbg!(&transfer_info);
    println!("{}", transfer_info.you + transfer_info.santa - 2 * transfer_info.parent - 2);
}

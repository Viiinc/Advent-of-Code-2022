use std::{fs, path::Path, collections::HashMap, i32::MIN, i32::MAX};

use regex::Regex;

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    let mut sensors: HashMap<(i32,i32), (i32,i32)> = HashMap::new();
    
    let mut minx = MAX;
    let mut maxx = MIN;
    let mut maxdist = 0;
    
    let mut candidates = Vec::new();
    
    for cap in re.captures_iter(&data) {
        let sensor = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
        let beacon = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
        sensors.insert(sensor, beacon);
        minx = minx.min(beacon.0);
        maxx = maxx.max(beacon.0);
        let distance = dist(sensor, beacon);
        // We know that the unique unsensed spot for part 2 MUST be just one further away than the closest beacon from some of the sensors (otherwise it would not be unique)
        // Double add of the corners is not an issue asymptotically, neither are the coordinates that are outside of the considered range (filtered later)
        for i in 0..=(distance + 1) {
            candidates.push((sensor.0 + i, sensor.1 - i + distance + 1));
            candidates.push((sensor.0 + i, sensor.1 + (i - distance - 1)));
            candidates.push((sensor.0 - i, sensor.1 - i + distance + 1));
            candidates.push((sensor.0 - i, sensor.1 + (i - distance - 1)));
        }
        maxdist = maxdist.max(distance);
    };

    minx -= maxdist;
    maxx += maxdist;
    
    let mut part1 = 0;
        
    let height = 2000000;
    for x in minx..=maxx {
        if sensors.iter().any(|(sensor, beacon)| dist(*sensor, *beacon) >= dist(*sensor, (x, height))) {
            if !sensors.iter().any(|(_, beacon)| *beacon == (x, height)) {
                part1 += 1
            }
        };
    }

    let max_coordinate = 4000000;
    // Filter coordinates outside of range
    let uncovered = candidates.iter().filter(|candidate| candidate.0 >= 0 && candidate.0 <= max_coordinate && candidate.1 >= 0 && candidate.1 <= max_coordinate)
    // Find the unique coordinate out of the sensors' range        
        .find(|candidate| sensors.iter().all(|(sensor, beacon)| dist(*sensor, *beacon) < dist(**candidate, *sensor))).unwrap();

    let part2 = max_coordinate as i64 * uncovered.0 as i64 + uncovered.1 as i64;

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

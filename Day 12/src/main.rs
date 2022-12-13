use std::{fs, path::Path, i32::MAX};
extern crate queues;
use queues::*;

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let map = data.split("\n")
        .map(|s| s.split("").filter(|s| s.len() > 0).map(|s| s.chars().nth(0).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut dist = map.iter().map(|r| r.iter().map(|_| MAX).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut q: Queue<(usize, usize)> = Queue::new();

    let mut start = (0, 0);
    let mut end = (0,0);
    for i in 0..map.len() {
        if map[i][0] == 'S' {
            start = (i,0 as usize);
        }
        for j in 0..map[i].len() {
            if map[i][j] == 'E' {
                end = (i,j);
            }
        }
    };

    q.add(end).expect("Wth, couldn't add to queue");
    dist[end.0][end.1] = 0;
    while q.size() > 0{
        let curr = q.remove().unwrap();
        let mut candidates = vec![(curr.0 + 1,curr.1), (curr.0, curr.1 + 1)];
        if curr.0 > 0 {candidates.push((curr.0 - 1, curr.1));}
        if curr.1 > 0 {candidates.push((curr.0, curr.1 - 1));}
        let steps = dist[curr.0][curr.1];
        let level = if map[curr.0][curr.1] == 'E' {'z'} else if map[curr.0][curr.1] == 'S' {'a'} else {map[curr.0][curr.1]};
        candidates.iter().filter(|c| c.0 < map.len() && c.1 < map[0].len()).for_each(|(i, j)| {
            let candidate = if map[*i][*j] == 'E' {'z'} else if map[*i][*j] == 'S' {'a'} else {map[*i][*j]};
            if (((level as u8) < (candidate as u8)) || ((level as u8) - (candidate as u8) <= 1)) && steps + 1 < dist[*i][*j] {
                dist[*i][*j] = steps + 1;
                q.add((*i,*j)).expect("Wth, couldn't add to queue");
            }
        });
    }

    let part1 = dist[start.0][start.1];
    let mut part2 = MAX;
    // Given input structure could just look in first column, but for thoroughness' sake scanning whole map:
    for i in 0..dist.len() {
        for j in 0..dist[i].len() {
            if dist[i][j] < part2 && map[i][j] == 'a' {part2 = dist[i][j]};
        }
    }

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

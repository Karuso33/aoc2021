use std::collections::HashSet;

use crate::get_input_lines;

const INPUT: &str = "problems/problem17";

fn parse_range(s: &str) -> Option<(i64, i64)> {
    let mut split = s.split("..");

    let start = split.next()?.parse::<i64>().ok()?;
    let end = split.next()?.parse::<i64>().ok()?;

    Some((start, end))
}

pub fn solve() -> crate::Result<()> {
    let line = get_input_lines(INPUT)?.next()
        .ok_or(crate::Error::NoInput)?;

    let line = &line["target area: ".len()..];
    let mut split = line.split(", ");

    let x_range = &split.next()
        .ok_or(crate::Error::InvalidInput)?["x=".len()..];

    let y_range = &split.next()
        .ok_or(crate::Error::InvalidInput)?["x=".len()..];

    let (start_x, end_x): (i64, i64) = parse_range(x_range).ok_or(crate::Error::InvalidInput)?;
    let (start_y, end_y): (i64, i64) = parse_range(y_range).ok_or(crate::Error::InvalidInput)?;

    let mut y_solutions = Vec::new();

    for y in start_y..=end_y {
        // In this iteration, we determine all (t, v0) that make
        // the probe land in y.

        // Note that if v0 is some velocity, and v(t) = v0 - t, then
        // s(t) = \sum_{i = 0}^(t - 1) v(i) = t v0 - 0.5 t (t - 1)
        // so
        // 2 v0 = t - 1 + 2 s(t) / t
        // From this, we see that t is a divisor 2 s(t), i.e. t <= 2 |y| and that
        // v0 = 1/2 (t - 1 + 2 s(t) / t)
        for t in 1..=2*y.abs() {
            let v0 = (t - 1 + (2 * y) / t) / 2;
            let sy = t * v0 - (t*t - t) / 2;

            if sy == y {
                y_solutions.push((t, v0))
            }
        }
    }

    // Sort by velocity, biggest velocity will be at the bottom
    y_solutions.sort_by_key(|&(t, v)| (v, t));
    y_solutions.dedup();

    // Now, try and realise every y solution by finding a corresponding v0_x.

    let mut prob1 = None;

    let mut possible_velocities: HashSet<(i64, i64)> = Default::default();

    for &(t, v0_y) in y_solutions.iter().rev() {
        // Observe that v0 <= end_x, because otherwise we would overshoot the
        // target in the first step

        for v0_x in 0..=end_x {
            let sx = if v0_x >= t {
                v0_x * t - (t * t - t) / 2
            } else {
                // We have reached "terminal velocity" and are therefore not moving anymore
                (v0_x * v0_x + v0_x) / 2
            };

            if start_x <= sx && sx <= end_x {
                if prob1.is_none() {
                    prob1 = Some((v0_y * v0_y + v0_y) / 2);
                }

                possible_velocities.insert((v0_x, v0_y));
            }
        }
    }

    println!("Problem 1: {}", prob1.unwrap());
    println!("Problem 2: {}", possible_velocities.len());

    Ok(())
}
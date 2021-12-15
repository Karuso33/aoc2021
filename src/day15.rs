use std::{cmp::Ordering, collections::BinaryHeap};

use crate::get_input_lines;

const INPUT: &str = "problems/problem15";

// The following code is an only slightly modified version of
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
const INF: u64 = u64::MAX;

#[derive(Clone, Debug)]
struct Edge {
    node: usize,
    cost: u64,
}

fn dijkstra(
    adj_list: &Vec<Vec<Edge>>,
    start: usize,
    goal: Option<usize>,
) -> (Vec<u64>, Vec<usize>) {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: u64,
        position: usize,
    }

    // Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            // Notice that the we flip the ordering on costs.
            // In case of a tie we compare positions - this step is necessary
            // to make implementations of `PartialEq` and `Ord` consistent.
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
        }
    }

    // `PartialOrd` needs to be implemented as well.
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let n = adj_list.len();

    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = vec![INF; n];

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    let mut prev = vec![n + 1; n];

    while let Some(State { cost, position }) = heap.pop() {
        if Some(position) == goal {
            break;
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
                prev[next.position] = position;
            }
        }
    }

    (dist, prev)
}

fn shortest_path(
    adj_list: &Vec<Vec<Edge>>,
    start: usize,
    goal: usize,
) -> Option<(Vec<usize>, u64)> {
    let (dist, prev) = dijkstra(adj_list, start, Some(goal));

    if dist[goal] < INF {
        let mut rev_path = vec![goal];
        let mut current = goal;

        while current != start {
            current = prev[current];
            rev_path.push(current);
        }

        rev_path.reverse();
        Some((rev_path, dist[goal]))
    } else {
        None
    }
}

fn checked_add(a: usize, b: isize) -> Option<usize> {
    if b >= 0 {
        a.checked_add(b as usize)
    } else {
        a.checked_sub(-b as usize)
    }
}

fn in_bounds(
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    let nx = checked_add(x, dx)?;
    let ny = checked_add(y, dy)?;

    if nx < width && ny < height {
        Some((nx, ny))
    } else {
        None
    }
}

pub fn solve() -> crate::Result<()> {
    let lines = get_input_lines(INPUT)?;

    let mut grid = Vec::new();
    let mut width = 0;

    for line in lines {
        if width == 0 {
            width = line.len();
        }

        for c in line.chars() {
            grid.push(c.to_digit(10).ok_or(crate::Error::InvalidInput)? as u8);
        }
    }

    let width = width;
    let height = grid.len() / width;

    // Part 1

    let n1 = width * height;
    let mut adj_list1 = (0..n1).map(|_| Vec::new()).collect::<Vec<_>>();

    for x in 0..width {
        for y in 0..height {
            for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some((nx, ny)) = in_bounds(x, y, dx, dy, width, height) {
                    let node = nx + ny * width;
                    let cost = grid[node] as u64;

                    adj_list1[x + y * width].push(Edge { node, cost });
                }
            }
        }
    }

    let (_, dist1) = shortest_path(&adj_list1, 0, n1 - 1).unwrap();
    println!("Problem 1: {}", dist1);

    // Part 2

    let modified_cost = |x: usize, y: usize| {
        let base_cost = grid[(x % width) + (y % height) * width] as u64;
        let i = (x / width) as u64 + (y / height) as u64;
        (base_cost - 1 + i) % 9 + 1
    };

    let n2 = (5 * width) * (5 * height);
    let mut adj_list2 = (0..n2).map(|_| Vec::new()).collect::<Vec<_>>();

    for x in 0..5 * width {
        for y in 0..5 * height {
            for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some((nx, ny)) = in_bounds(x, y, dx, dy, 5 * width, 5 * height) {
                    let node = nx + ny * (5 * width);
                    let cost = modified_cost(nx, ny);

                    adj_list2[x + y * (5 * width)].push(Edge { node, cost });
                }
            }
        }
    }

    let (_, dist2) = shortest_path(&adj_list2, 0, n2 - 1).unwrap();
    println!("Problem 2: {}", dist2);

    Ok(())
}

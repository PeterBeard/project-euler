// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// By starting at the top of the triangle below and moving to adjacent
// numbers on the row below, the maximum total from top to bottom is 23.
// 
// 3
// 7 4
// 2 4 6
// 8 5 9 3
// 
// That is, 3 + 7 + 4 + 9 = 23.
// 
// Find the maximum total from top to bottom of the triangle below:
// 
// 75
// 95 64
// 17 47 82
// 18 35 87 10
// 20 04 82 47 65
// 19 01 23 75 03 34
// 88 02 77 73 07 63 67
// 99 65 04 28 06 16 70 92
// 41 41 26 56 83 40 80 70 33
// 41 48 72 33 47 32 37 16 94 29
// 53 71 44 65 25 43 91 52 97 51 14
// 70 11 33 28 77 73 17 78 39 68 17 57
// 91 71 52 38 17 14 91 43 58 50 27 29 48
// 63 66 04 68 89 53 67 30 73 16 69 87 40 31
// 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
// 
// NOTE: As there are only 16384 routes, it is possible to solve this problem
// by trying every route. However, Problem 67, is the same challenge with a
// triangle containing one-hundred rows; it cannot be solved by brute force,
// and requires a clever method! ;o)
#![feature(test)]
extern crate test;

/// Find the maximum path from the start node to an end node
fn maximum_path_sum(graph: &[Vec<u32>]) -> u32 {
    let mut sums: Vec<Vec<u32>> = Vec::new();
    sums.extend(graph.iter().cloned());
    
    // Calculate the sum of each value and its largest neighbor
    // If we replace each value with that sum, then whatever value is left at
    // the top of the tree will be the sum of the largest path since the large
    // values will propagate up through the tree.
    for y in (0..graph.len() - 1).rev() {
        for x in 0..graph[y].len() {
            let value = graph[y][x];
            // Neighbors down and right, pick the bigger one
            let n_down = sums[y+1][x];
            let n_right = sums[y+1][x+1];
            sums[y][x] = if n_down > n_right { n_down + value } else { n_right + value };
        }
    }
    // Return the top of the tree
    sums[0][0]
}

pub fn solution() -> u32 {
    // Build the graph
    let mut graph: Vec<Vec<u32>> = Vec::new();
    graph.push(vec![75]);
    graph.push(vec![95, 64]);
    graph.push(vec![17, 47, 82]);
    graph.push(vec![18, 35, 87, 10]);
    graph.push(vec![20, 04, 82, 47, 65]);
    graph.push(vec![19, 01, 23, 75, 03, 34]);
    graph.push(vec![88, 02, 77, 73, 07, 63, 67]);
    graph.push(vec![99, 65, 04, 28, 06, 16, 70, 92]);
    graph.push(vec![41, 41, 26, 56, 83, 40, 80, 70, 33]);
    graph.push(vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29]);
    graph.push(vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14]);
    graph.push(vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57]);
    graph.push(vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48]);
    graph.push(vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31]);
    graph.push(vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23]);

    maximum_path_sum(&graph[..])
}

fn main() {
    println!("The sum of the maximum path is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(1074, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}

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

/// Find the maximum path from the start node to an end node
fn maximum_path(graph: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut path: Vec<u8> = Vec::new();
    let mut sums: Vec<Vec<u32>> = Vec::new();
    sums.push(vec!(graph[0][0] as u32));
    
    // Calculate the sum of each value and its largest neighbor
    // The maximum path is the path between the largest sums in each row
    for y in 1..graph.len() {
        let mut sum_row: Vec<u32> = Vec::new();
        for x in 0..graph[y].len() {
            let value = graph[y][x] as u32;
            if x == 0 {
                // Only up
                sum_row.push(sums[y-1][x] + value);
            } else if x == graph[y].len() - 1 {
                // Only left
                sum_row.push(sums[y-1][x-1] + value);
            } else {
                // Neighbors up and left, pick the bigger one
                let n_up = sums[y-1][x];
                let n_left = sums[y-1][x-1];
                if n_up > n_left {
                    sum_row.push(n_up + value);
                } else {
                    sum_row.push(n_left + value);
                }
            }
        }
        sums.push(sum_row);
    }

    // The maximum path passes through the maximum value in each row
    for y in 0..graph.len() {
        let mut max_val = 0;
        let mut max_sum = 0;
        for x in 0..graph[y].len() {
            if sums[y][x] > max_sum {
                max_val = graph[y][x];
                max_sum = sums[y][x];
            }
        }
        path.push(max_val);
    }
    path
}

fn main() {
    // Build the graph
    let mut graph: Vec<Vec<u8>> = Vec::new();
    /*
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
    */
    graph.push(vec![3]);
    graph.push(vec![7, 4]);
    graph.push(vec![2, 4, 6]);
    graph.push(vec![8, 5, 9, 3]);

    let path = maximum_path(&graph);
    let mut path_sum = 0;

    // Show the maximum path
    for i in 0..path.len() {
        if i < path.len() - 1 {
            print!("{} -> ", path[i]);
        } else {
            print!("{}\n", path[i]);
        }
    }

    // Calculate the sum
    for v in path {
        path_sum += v as u32;
    }
    println!("The sum of the maximum path is {}", path_sum);
}

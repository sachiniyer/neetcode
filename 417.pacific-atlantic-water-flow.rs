// use std::collections::HashSet;

// impl Solution {
//     pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         fn traversal(
//             x: usize,
//             y: usize,
//             direction: bool,
//             m: &mut HashSet<(usize, usize)>,
//             heights: &Vec<Vec<i32>>,
//         ) -> i32 {
//             let mut adjs;
//             if direction {
//                 if x >= heights[0].len() - 1 && y >= heights.len() - 1 {
//                     m.insert((y, x));
//                     return heights[y][x];
//                 } else if x >= heights[0].len() - 1 {
//                     m.insert((y, x));
//                     adjs = vec![(x, y + 1)];
//                 } else if y >= heights.len() - 1 {
//                     m.insert((y, x));
//                     adjs = vec![(x + 1, y)];
//                 } else {
//                     adjs = vec![(x + 1, y), (x, y + 1)];
//                 }
//             } else {
//                 if x <= 0 && y <= 0 {
//                     m.insert((y, x));
//                     return heights[y][x];
//                 } else if x <= 0 {
//                     m.insert((y, x));
//                     adjs = vec![(x, y - 1)];
//                 } else if y <= 0 {
//                     m.insert((y, x));
//                     adjs = vec![(x - 1, y)];
//                 } else {
//                     adjs = vec![(x - 1, y), (x, y - 1)];
//                 }
//             }
//             let mut escape = false;
//             let mut res = -1;
//             for a in adjs {
//                 if heights[0].len() > a.0 && a.0 >= 0 && heights.len() > a.1 && a.1 >= 0 {
//                     if heights[a.1][a.0] >= heights[y][x] {
//                         escape = true;
//                         res = traversal(a.0, a.1, direction, m, heights);

//                         if res == heights[y][x] {
//                             m.insert((y, x));
//                         }
//                     }
//                 }
//             }
//             if !escape {
//                 m.insert((y, x));
//                 return heights[y][x];
//             }
//             res
//         }

//         let mut pacific: HashSet<(usize, usize)> = HashSet::new();

//         // create pacific hashset
//         for x in 0..heights[0].len() {
//             pacific.insert((x, 0));
//             traversal(x, 0, true, &mut pacific, &heights);
//         }
//         for y in 0..heights.len() {
//             pacific.insert((0, y));
//             traversal(0, y, true, &mut pacific, &heights);
//         }
//         // println!("{:?}", pacific);

//         let mut atlantic: HashSet<(usize, usize)> = HashSet::new();

//         // create atlantic hashset
//         for x in 0..heights[0].len() {
//             atlantic.insert((x, heights.len() - 1));
//             traversal(x, heights.len() - 1, false, &mut atlantic, &heights);
//         }
//         for y in 0..heights.len() {
//             atlantic.insert((heights.len() - 1, y));
//             traversal(heights[0].len() - 1, y, false, &mut atlantic, &heights);
//         }

//         // println!("{:?}", atlantic);
//         pacific
//             .intersection(&atlantic)
//             .map(|&(a, b)| vec![a as i32, b as i32])
//             .collect()
//     }
// }

use std::collections::HashSet;
impl Solution {
    pub fn pacific_atlantic(mut heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(
            set: &mut HashSet<(i32, i32)>,
            arr: &mut Vec<Vec<i32>>,
            r: usize,
            c: usize,
            prev_height: i32,
        ) {
            if prev_height <= arr[r][c] && set.insert((r as i32, c as i32)) {
                // Up
                if r > 0 {
                    dfs(set, arr, r - 1, c, arr[r][c]);
                }
                // Down
                if r < arr.len() - 1 {
                    dfs(set, arr, r + 1, c, arr[r][c]);
                }
                // Left
                if c > 0 {
                    dfs(set, arr, r, c - 1, arr[r][c]);
                }
                // Right
                if c < arr[r].len() - 1 {
                    dfs(set, arr, r, c + 1, arr[r][c]);
                }
            }
        }

        let mut set_pacific = HashSet::default();
        let mut set_atlantic = HashSet::default();

        let num_rows = heights.len() - 1;
        let num_cols = heights[0].len() - 1;

        // We only check for heights on the outer bounds of the `heights` grid
        for c in 0..=num_cols {
            let h_pacific = heights[0][c];
            let h_atlantic = heights[num_rows][c];
            dfs(&mut set_pacific, &mut heights, 0, c, h_pacific);
            dfs(&mut set_atlantic, &mut heights, num_rows, c, h_atlantic);
        }
        for r in 0..=num_rows {
            let h_pacific = heights[r][0];
            let h_atlantic = heights[r][num_cols];
            dfs(&mut set_pacific, &mut heights, r, 0, h_pacific);
            dfs(&mut set_atlantic, &mut heights, r, num_cols, h_atlantic);
        }

        set_atlantic
            .intersection(&set_pacific)
            .into_iter()
            .map(|&(r, c)| vec![r, c])
            .collect()
    }
}

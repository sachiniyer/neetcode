use std::collections::VecDeque;
impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let EMPTY: i32 = 2147483647;
        let GATE: i32 = 0;

        let mut gates: VecDeque<(usize, usize)> = VecDeque::new();

        for y in 0..rooms.len() {
            for x in 0..rooms[y].len() {
                if rooms[y][x] == GATE {
                    gates.push_front((y, x));
                }
            }
        }
        let mut distance = 0;
        while gates.len() > 0 {
            distance += 1;
            let mut children: VecDeque<(usize, usize)> = VecDeque::new();
            while gates.len() > 0 {
                let (y, x) = gates.pop_front().unwrap();

                let adjs = vec![(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];

                for a in adjs {
                    if a.0 >= 0 && a.1 >= 0 && a.0 < rooms.len() && a.1 < rooms[0].len() {
                        if rooms[a.0][a.1] == EMPTY {
                            rooms[a.0][a.1] = distance;
                            children.push_front((a.0, a.1));
                        }
                    }
                }
            }
            gates = children;
        }
    }
}

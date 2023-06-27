impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut i = cost[0];
        let mut j = cost[1];

        for n in 2..cost.len() {
            let temp = j;
            j = i.min(j) + cost[n];
            i = temp;
        }
        i.min(j)
    }
}

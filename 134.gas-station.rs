impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_gain = 0;
        let mut curr_gain = 0;
        let mut res = 0;

        for i in 0..gas.len() {
            total_gain += gas[i] - cost[i];
            curr_gain += gas[i] - cost[i];

            if curr_gain < 0 {
                curr_gain = 0;
                res = i + 1;
            }
        }
        match total_gain {
            t if t >= 0 => res as i32,
            _ => -1,
        }
    }
}

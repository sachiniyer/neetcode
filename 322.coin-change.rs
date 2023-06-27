impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut table: Vec<i32> = vec![0; amount as usize + 1];

        for i in 1..table.len() {
            let mut minimum = i32::MAX;

            for j in coins.iter() {
                let j = *j;
                if (j as usize) <= i {
                    let child = table[i - (j as usize)];
                    if child != -1 {
                        minimum = minimum.min(child);
                    }
                }
            }

            if minimum == i32::MAX {
                table[i] = -1;
            } else {
                table[i] = minimum + 1;
            }
        }

        table[amount as usize]
    }
}

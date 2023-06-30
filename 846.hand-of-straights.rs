// rchaser53

use std::collections::BTreeMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len();
        let m = group_size as usize;
        if hand.len() % m != 0 {
            return false;
        }
        let x = hand.len() / m;

        let mut btree = BTreeMap::new();
        for v in hand {
            *btree.entry(v).or_insert(0) += 1;
        }

        for _ in 0..x {
            let (&min, _) = btree.iter().next().unwrap();
            for i in 0..m {
                let cv = min + i as i32;
                if let Some(num) = btree.get_mut(&cv) {
                    *num -= 1;
                    if *num == 0 {
                        btree.remove(&cv);
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }
}

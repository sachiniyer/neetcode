impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l: i32 = 1;
        let mut r: i32 = *piles.iter().max().unwrap();
        if r - l == 0 {
            return 1;
        }
        if r - l == 1 {
            if piles
                .iter()
                .map(|&x| (x as f64 / r as f64).ceil() as i32)
                .sum::<i32>()
                <= h
            {
                return r;
            }
            return l;
        }
        while l + 1 < r {
            let m: i32 = ((r - l) / 2) + l;
            let s: i32 = piles
                .iter()
                .map(|&x| (x as f64 / m as f64).ceil() as i32)
                .sum();
            if s > h {
                l = m;
            } else {
                r = m;
            }
        }
        if piles
            .iter()
            .map(|&x| (x as f64 / l as f64).ceil() as i32)
            .sum::<i32>()
            <= h
        {
            return l;
        }
        r
    }
}

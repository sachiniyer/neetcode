impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let a;
        let b;
        if nums1.len() < nums2.len() {
            a = nums1;
            b = nums2;
        } else {
            a = nums2;
            b = nums1;
        }

        let mut l = 0;
        let mut r = a.len();

        let mut ai;
        let mut bi;

        while l <= r {
            ai = (l + r) / 2;
            bi = (a.len() + b.len() + 1) / 2 - ai;

            let al = if ai == 0 { std::i32::MIN } else { a[ai - 1] };
            let ar = if ai == a.len() { std::i32::MAX } else { a[ai] };
            let bl = if bi == 0 { std::i32::MIN } else { b[bi - 1] };
            let br = if bi == b.len() { std::i32::MAX } else { b[bi] };

            if al <= br && bl <= ar {
                if (a.len() + b.len()) % 2 == 0 {
                    return f64::from(al.max(bl) + ar.min(br)) / 2.0;
                } else {
                    return f64::from(al.max(bl));
                }
            } else if al > br {
                r = ai - 1;
            } else {
                l = ai + 1;
            }
        }
        panic!("Issue");
    }
}

use std::collections::HashMap;
struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_insert(Vec::new()).push((timestamp, value));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(nums) = self.map.get(&key) {
            let mut l: usize = 0;
            let mut r: usize = nums.len() - 1;
            if nums.len() == 1 {
                if nums[0].0 <= timestamp {
                    return nums[0].1.clone();
                }
                else {
                    return "".to_string();
                }
            }
            if nums.len() == 2 {
                if nums[1].0 <= timestamp {
                    return nums[1].1.clone();
                } else if nums[0].0 <= timestamp {
                    return nums[0].1.clone();
                }
                else {
                    return "".to_string();
                }
            }
            while l + 1 < r {
                let m = ((r - l) / 2) + l;
                if nums[m].0 > timestamp {
                    r = m;
                } else {
                    l = m;
                }
            }
            if nums[r].0 <= timestamp {
                return nums[r].1.clone();
            }
            if nums[l].0 <= timestamp {
                return nums[l].1.clone();
            }
        }
        "".to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

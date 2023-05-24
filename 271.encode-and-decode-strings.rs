struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {
        }
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let l = strs.len();
        let l = format!("{:03}", l);
        let mut full_str = "".to_owned() + &l;
        for s in &strs {
            full_str += &format!("{:03}", s.len()).to_string();
        }
        for s in strs {
            full_str += &s;
        }
        full_str
    }

    fn decode(&self, s: String) -> Vec<String> {
        use std::collections::VecDeque;
        let snums = &s[0..3];
        let snums = snums.parse::<usize>().unwrap();
        let mut slens: VecDeque<usize> = VecDeque::new();
        for i in 1..=snums {
            let slen = &s[(i*3)..(i*3 + 3)];
            let slen = slen.parse::<usize>().unwrap();
            slens.push_back(slen);
        }
        let mut res: Vec<String> = Vec::new();
        let mut i = snums*3 + 3;
        while i < s.len() {
            let l = slens.pop_front().unwrap();
            if l == 0 {
                res.push("".to_string());
            }
            else {
                let n = s[i..(i+l)].to_string();
                res.push(n);
                i = i+l;
            }
        }
        while !slens.is_empty() {
            let l = slens.pop_front().unwrap();
            if l == 0 {
                res.push("".to_string());
            }
            else {
                println!("{}", s);
                println!("{}", l);
            }
        }
        res
    }

}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */

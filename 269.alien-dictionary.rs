use std::collections::HashMap;

pub fn find_relationship(s1: String, s2: String) -> (char, char) {
    let mut chars1: Vec<char> = s1.chars().collect();
    let mut chars2: Vec<char> = s2.chars().collect();
    let mut i = 0;
    let mut j = 0;
    while i < chars1.len() && j < chars2.len() {
        if chars1[i] != chars2[j] {
            return (chars1[i], chars2[j]);
        }
        i += 1;
        j += 1;
    }
    if i < chars1.len() {
        return (chars1[i], ' ');
    } else if j < chars2.len() {
        return (' ', chars2[j]);
    }
    (' ', ' ')
}

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut edges = HashMap::new();
        let mut count_edges = HashMap::new();
        for i in 0..words.len() - 1 {
            let (c1, c2) = find_relationship(words[i].clone(), words[i + 1].clone());
            if c1 != ' ' && c2 != ' ' {
                edges.entry(c1).or_insert(vec![]).push(c2);
                *count_edges.entry(c2).or_insert(0) += 1;
                count_edges.entry(c1).or_insert(0);
            }
            if c1 != ' ' && c2 == ' ' {
                return "".to_string();
            }
        }
        let mut res = "".to_string();

        for word in words.iter() {
            for c in word.chars() {
                if !count_edges.contains_key(&c) {
                    count_edges.insert(c, 0);
                }
            }
        }

        let mut queue = vec![];
        for (c, count) in count_edges.iter() {
            if *count == 0 {
                queue.push(*c);
            }
        }
        while !queue.is_empty() {
            let c = queue.pop().unwrap();
            res.push(c);
            if let Some(v) = edges.get(&c) {
                for &c2 in v.iter() {
                    *count_edges.get_mut(&c2).unwrap() -= 1;
                    if *count_edges.get(&c2).unwrap() == 0 {
                        queue.push(c2);
                    }
                }
            }
        }
        if res.len() != count_edges.len() {
            return "".to_string();
        }

        res
    }
}

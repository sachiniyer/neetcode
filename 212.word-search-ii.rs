// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use std::rc::Rc;

// struct TrieNode {
//     pub m: HashMap<char, (TrieNode, bool)>,
//     pub v: char,
// }

// impl TrieNode {
//     pub fn new(v: char) -> Self {
//         Self {
//             m: HashMap::new(),
//             v,
//         }
//     }
// }

// struct Trie {
//     pub root: TrieNode,
// }

// impl Trie {
//     fn new() -> Self {
//         Self {
//             root: TrieNode::new('-'),
//         }
//     }

//     fn insert(&mut self, word: String) {
//         let mut node = &mut self.root;
//         let w: Vec<char> = word.chars().collect();
//         for s in 0..w.len() - 1 {
//             let mut entry = node.m.entry(w[s]).or_insert((TrieNode::new(w[s]), false));
//             node = &mut entry.0;
//         }
//         (*node
//             .m
//             .entry(*w.last().unwrap())
//             .or_insert((TrieNode::new(*w.last().unwrap()), true)))
//         .1 = true;
//     }

//     fn search(&self, word: String) -> (bool, bool) {
//         let mut node = &self.root;
//         let w: Vec<char> = word.chars().collect();
//         for s in 0..w.len() - 1 {
//             match node.m.get(&w[s]) {
//                 Some(n) => node = &n.0,
//                 None => return (false, false),
//             }
//         }
//         match node.m.get(&w.last().unwrap()) {
//             Some(n) => (true, n.1),
//             None => (false, false),
//         }
//     }

//     fn delete(&mut self, word: String) {
//         let mut node = &mut self.root;
//         let w: Vec<char> = word.chars().collect();
//         let mut nodes: VecDeque<&mut TrieNode> = VecDeque::new();
//         nodes.push_back(&mut self.root);
//         for s in 0..w.len() {
//             match nodes.back_mut().unwrap().m.get_mut(&w[s]) {
//                 Some(n) => {
//                     nodes.push_back(&mut n.0);
//                 }
//                 None => break,
//             }
//         }
//         let n = nodes.pop_back().unwrap();
//         n.m.remove(&w.last().unwrap());
//         let mut cont = true;
//         for s in w.len() - 2..=0 {
//             if cont {
//                 match nodes.pop_back() {
//                     Some(n) => {
//                         if n.m.get(&w[s]).unwrap().1 == false {
//                             n.m.remove(&w[s]);
//                         } else {
//                             break;
//                         }
//                     }
//                     None => break,
//                 }
//             }
//         }
//     }
// }

// impl Solution {
//     pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
//         let mut trie = Trie::new();
//         for w in words {
//             trie.insert(w);
//         }

//         let mut res: HashSet<String> = HashSet::new();

//         fn wordlist(
//             node: &TrieNode,
//             prefix: &String,
//             ix: usize,
//             iy: usize,
//             board: &Vec<Vec<char>>,
//             used: &HashSet<String>,
//             res: &mut HashSet<String>,
//         ) {
//             let mut prefix = prefix.to_owned();
//             let mut used = used.to_owned();

//             let adjs = vec![(ix + 1, iy), (ix - 1, iy), (ix, iy + 1), (ix, iy - 1)];
//             for i in adjs {
//                 if i.0 < board.len() && i.1 < board[0].len() {
//                     for (k, v) in node.m.iter() {
//                         if *k == board[i.0][i.1]
//                             && !used.contains(&(i.0.to_string() + &i.1.to_string()))
//                         {
//                             prefix.push(*k);
//                             if v.1 {
//                                 res.insert(prefix.clone());
//                             }
//                             used.insert((i.0.to_string() + &i.1.to_string()));
//                             wordlist(&v.0, &prefix, i.0, i.1, board, &used, res);
//                             used.remove(&(i.0.to_string() + &i.1.to_string()));
//                             prefix.pop();
//                         }
//                     }
//                 }
//             }
//         }

//         for i in 0..board.len() {
//             for j in 0..board[0].len() {
//                 for (k, v) in trie.root.m.iter() {
//                     let mut used = HashSet::new();
//                     let mut prefix = String::from("");
//                     if *k == board[i][j] {
//                         prefix.push(*k);
//                         if v.1 {
//                             res.insert(prefix.clone());
//                         }
//                         used.insert((i.to_string() + &j.to_string()));
//                         wordlist(&v.0, &prefix, i, j, &board, &used, &mut res)
//                     }
//                 }
//             }
//         }

//         res.into_iter().collect()
//     }
// }
use std::collections::HashSet;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    word: Option<String>,
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie: Trie = Default::default();
        for word in words.iter() {
            let mut node = &mut trie;
            for c in word.as_bytes() {
                node =
                    node.children[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
            }
            node.word = Some(word.clone());
        }
        let mut answer: HashSet<String> = HashSet::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut visited: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
                Solution::dfs(&board, (i, j), &trie, &mut visited, &mut answer);
            }
        }
        answer.into_iter().collect()
    }
    fn dfs(
        board: &[Vec<char>],
        pos: (usize, usize),
        trie: &Trie,
        visited: &mut Vec<Vec<bool>>,
        answer: &mut HashSet<String>,
    ) {
        if visited[pos.0][pos.1] {
            return;
        }
        visited[pos.0][pos.1] = true;
        let c = board[pos.0][pos.1];
        if let Some(node) = &trie.children[(c as u8 - b'a') as usize] {
            if let Some(word) = &node.word {
                answer.insert(word.clone());
            }
            if pos.0 > 0 {
                Solution::dfs(board, (pos.0 - 1, pos.1), node.as_ref(), visited, answer);
            }
            if pos.1 > 0 {
                Solution::dfs(board, (pos.0, pos.1 - 1), node.as_ref(), visited, answer);
            }
            if pos.0 < board.len() - 1 {
                Solution::dfs(board, (pos.0 + 1, pos.1), node.as_ref(), visited, answer);
            }
            if pos.1 < board[0].len() - 1 {
                Solution::dfs(board, (pos.0, pos.1 + 1), node.as_ref(), visited, answer);
            }
        }
        visited[pos.0][pos.1] = false;
    }
}

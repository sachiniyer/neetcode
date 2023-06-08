use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
struct Post(i32, i32, i32, i32);

impl Ord for Post {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


struct Twitter {
    follower_map: HashMap<i32, HashSet<i32>>,
    post_map: HashMap<i32, VecDeque<(i32, i32)>>,
    count: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {
        Self {
            follower_map: HashMap::new(),
            post_map: HashMap::new(),
            count: 0
        }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.count += 1;
        (*self.post_map.entry(user_id).or_insert(VecDeque::new())).push_back((self.count, tweet_id));
    }
    
    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        match self.follower_map.get(&user_id) {
            Some(s) => {
                println!("{:?}", s);
                let mut post_heap: BinaryHeap<Post> = BinaryHeap::new();
                for id in s.iter() {
                    match self.post_map.get(id) {
                        Some(posts) => {
                            match posts.back() {
                                Some(post) => post_heap.push(Post(post.0, post.1, (posts.len()-1) as i32, *id)),
                                None => ()
                            }
                        },
                        None => ()
                    }
                }
                let mut res: Vec<i32> = Vec::new();
                while res.len() < 10 && !post_heap.is_empty() {
                    let top = post_heap.pop().unwrap();
                    match self.post_map.get(&top.3){
                        Some(posts) => {
                            match posts.get((top.2-1) as usize) {
                                Some(post) => post_heap.push(Post(post.0, post.1, top.2-1, top.3)),
                                None => ()
                            }
                        }
                        None => ()
                    }
                    res.push(top.1);
                }
                res
            },
            None => {
                self.follower_map.insert(user_id, HashSet::new());
                self.follower_map.get_mut(&user_id).unwrap().insert(user_id);
                self.get_news_feed(user_id)
            }
        }
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        match self.follower_map.get_mut(&follower_id) {
            Some (m) => {m.insert(followee_id);},
            None => {
                self.follower_map.insert(follower_id, HashSet::new());
                self.follower_map.get_mut(&follower_id).unwrap().insert(follower_id);
                self.follow(follower_id, followee_id)
            }
        }
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        match self.follower_map.get_mut(&follower_id) {
            Some (m) => {
                m.remove(&followee_id);
            },
            None => {
                self.follower_map.insert(follower_id, HashSet::new());
                self.follower_map.get_mut(&follower_id).unwrap().insert(follower_id);
            }
        }

    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

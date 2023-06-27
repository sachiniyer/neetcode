// marccarre

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());

        let mut dist = Vec::with_capacity(word2.len() + 1);

        for j in 0..=word2.len() {
            dist.push(j)
        }
        let mut prev_dist = dist.clone();

        for i in 1..=word1.len() {
            for j in 0..=word2.len() {
                if j == 0 {
                    dist[j] += 1;
                } else if word1[i - 1] == word2[j - 1] {
                    dist[j] = prev_dist[j - 1];
                } else {
                    dist[j] = dist[j].min(dist[j - 1]).min(prev_dist[j - 1]) + 1;
                }
            }
            prev_dist.copy_from_slice(&dist);
        }
        dist[word2.len()] as i32
    }
}

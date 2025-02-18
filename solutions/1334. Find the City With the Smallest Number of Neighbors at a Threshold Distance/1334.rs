// Time complexity: O(n^3)
// Space complexity: O(n^2)
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let dist = Self::floyd_warshall(n, &edges, distance_threshold);
        let mut ans = -1;
        let mut min_cities_count = n;

        for i in 0..n {
            let cities_count = (0..n).filter(|&j| dist[i][j] <= distance_threshold).count();
            if cities_count <= min_cities_count {
                ans = i as i32;
                min_cities_count = cities_count;
            }
        }

        ans
    }

    fn floyd_warshall(n: usize, edges: &Vec<Vec<i32>>, distance_threshold: i32) -> Vec<Vec<i32>> {
        let mut dist = vec![vec![distance_threshold + 1; n]; n];

        for i in 0..n {
            dist[i][i] = 0;
        }

        for edge in edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            dist[u][v] = w;
            dist[v][u] = w;
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }

        dist
    }
}

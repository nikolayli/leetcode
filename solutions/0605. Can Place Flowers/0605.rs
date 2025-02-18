// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;

        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0 {
                let empty_left_plot = i == 0 || flowerbed[i - 1] == 0;
                let empty_right_plot = i == flowerbed.len() - 1 || flowerbed[i + 1] == 0;

                if empty_left_plot && empty_right_plot {
                    flowerbed[i] = 1;
                    count += 1;
                }
            }
        }

        count >= n
    }
}

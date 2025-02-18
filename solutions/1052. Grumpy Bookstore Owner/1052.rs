// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let minutes = minutes as usize;

        let satisfied: i32 = customers
            .iter()
            .zip(grumpy.iter())
            .filter(|&(_, &g)| g == 0)
            .map(|(&c, _)| c)
            .sum();

        let mut additional_satisfied = 0;
        let mut max_additional_satisfied = 0;

        for i in 0..customers.len() {
            if grumpy[i] == 1 {
                additional_satisfied += customers[i];
            }
            if i >= minutes && grumpy[i - minutes] == 1 {
                additional_satisfied -= customers[i - minutes];
            }
            max_additional_satisfied = max_additional_satisfied.max(additional_satisfied);
        }

        satisfied + max_additional_satisfied
    }
}

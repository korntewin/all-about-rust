use algorithms::min_in_rotated_array::solve;

fn main() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    println!("Minimum in rotated array: {}", solve(nums));

    let nums = vec![3, 4, 5, 1, 2];
    println!("Minimum in rotated array: {}", solve(nums));
}

// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
    let vec = vec![22, 44, 66]; // Sample vector
    let filled_vec = fill_vec(vec); // Call the function to fill the vector

    // Print the result.
    println!("Filled vector: {:?}", filled_vec); // Use debug formatting to print the vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

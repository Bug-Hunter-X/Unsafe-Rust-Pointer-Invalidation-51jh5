fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, we directly modify the vector.
    v[0] = 4; 
    println!("Modified vector: {:?}", v);
}
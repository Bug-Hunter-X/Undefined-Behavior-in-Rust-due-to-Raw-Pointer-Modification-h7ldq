fn main() {
    let mut v = vec![1, 2, 3];
    // Avoid direct manipulation of vector's internal data
    v[0] = 10;
    println!("{:?}", v);
}
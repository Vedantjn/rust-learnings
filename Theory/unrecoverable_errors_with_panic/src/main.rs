fn main() {
    println!("Hello, world!");

    // Uncomment the line below to cause a panic and terminate the program
    // panic!("This is my panic");

    // Create a vector with three elements
    let v = vec![1, 2, 3];

    // Attempt to access an out-of-bounds index (this will cause a panic)
    v[99];

    // This line will not be executed due to the panic caused by the previous line
    println!("This is end of the program");
}

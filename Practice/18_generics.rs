fn main() {
    let bigger = larger(1, 2);
    let bigger_char = larger('a', 'b');

    println!("{}", bigger);
    println!("{}", bigger_char);
}

fn larger<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    }
    else {
        b
    }
}

// In TypeScript
fn larger<T>(a: T, b: T): T {
    if a > b {
        return a;
    }
    else {
        return b;
    }
}
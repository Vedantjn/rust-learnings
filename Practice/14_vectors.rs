fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec);
}

// -----------------------------------------------

// Function that returns even values
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ans = even_filter(vec);

    println!("{:?}", ans);
}

fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }

    return new_vec;
}

// -----------------------------------------------

// Function that returns even values
// Borrowing
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ans = even_filter(&vec);

    println!("{:?}", ans);
    println!("{:?}", vec);
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val); // dereference
        }
    }

    return new_vec;
}

// -----------------------------------------------

fn main() {
    let numbers = vec![1, 2, 3];
    let numbers: Vec<I32> = vec![1, 2, 3];
    for number in numbers {
        println!("{}", number);
    }
}


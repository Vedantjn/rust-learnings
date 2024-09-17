fn main() {
    let nums = vec![1, 2, 3, 4];

    for num in nums {
        println!("{}", num);
    }
    
    // println!("{:?}", v1);
}

// ---------------------------------------------------------

// Borrowing
// Immutable reference and ownership is not transferred
fn main() {
    let nums = vec![1, 2, 3, 4];
    let iter = nums.iter();

    for num in iter {
        println!("{}", num);
    }
    
    println!("{:?}", v1);
}

// ---------------------------------------------------------

// Borrowing
// Mutable reference, ownership is not transferred
fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter_mut();

    for num in v1_iter {
        *value = *value + 1;
    }

    println!("{:?}", v1);
}

// ---------------------------------------------------------

fn main() {
    let v1 = vec![1, 2, 3, 4];
    let mut v1_iter = v1.iter();

    while let Some(val) = v1_iter.next() {
        println!("{}", val);
    }
}

// ---------------------------------------------------------

// Takes ownership
// If you want to move the variable into the iterator and don't want to use it afterwards
fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.into_iter();

    for value in v1_iter {
        println!("{}", val);
    }

    println!("{}", v1); // Ownership is already taken
}

// ---------------------------------------------------------

fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.into_iter();

    for value in v1_iter {
        println!("{}", val);
    }

    println!("{}", v1); // Ownership is already taken
}

// ---------------------------------------------------------

// Consuming Adaptors -> Methods that call next. Because calling them uses up the iterator

// sum
// ---------------------------------------------------------

// sum() -> is a consuming adapter
fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    
    assert_eq!(total, 10);
    
    let sum2: i32 = v1_iter.sum();
}

// ---------------------------------------------------------

// Iterator Adaptors -> methods defined on the iterator trait that don't consume the iterator. 
// Instead, they produce different iterators by changing some aspect of the original iterator.

// map, filter
// ---------------------------------------------------------

// map
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let iter = v1.iter();
    let iter2 = iter.map(|x| x + 1);

    for value in iter2 {
        println!("{}", value);
    }

    println!("{}", v1);
}
// ---------------------------------------------------------

// filter
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let iter = v1.iter();
    let iter2 = iter.filter(|x| *x % 2 == 0);

    for value in iter2 {
        println!("{}", value);
    }

    println!("{}", v1);
}

// ---------------------------------------------------------

// Q). Write the logic to first filter all odd values then double each value and create a new vector

fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();

    let ans = v1_iter.filter(|x| *x % 2 == 1),map(|x| x*2);

    for value in ans {
        println!("{}", value);
    }
}
// ---------------------------------------------------------

// Q). Write the logic to first filter all odd values then double each value and create a new vector
// collect() : Iterator -> Vector 

fn main() {
    let v1 = vec![1, 2, 3, 4];
    let iter = v1.iter().filter(|x| *x%2 == 0).map(|x| x*2);

    let v2: Vec<i32> = iter.collect();

    println!("{}", v2);
}
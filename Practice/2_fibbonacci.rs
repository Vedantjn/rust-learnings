fn main() {
    let ans = fib(3);
    println!("{}", ans);
}

fn fib(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }

    fib(num - 1) + fib(num - 2)
}

// -------------------------------------------------

fn main() {
    println!("{}", fib(4));
}

fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 0..(num - 1) {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}

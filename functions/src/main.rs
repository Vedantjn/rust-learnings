// fn main() {
//     println!("Hello, world!");
//     let num = 7;
//     my_function(num, true);
// }

// fn my_function(x: i32, y: bool) {
//     println!("Hello from my function {x} {y}");
// }


// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("Value of y is : {y}")
// }





// fn main() {
//     let y = add(5, 6);
//     println!("Value of y is : {y}")
// }

// fn add(x: i32, y: i32) -> i32{
//     // return 10;
//     // 24
//     // x+y

//     let result = x+y;
//     result
// }



fn main() {
    let y = is_even(5);
    println!("Value of y is: {y}");
}

fn is_even(x: i32) -> bool {
    if x%2 == 0 {
        return true;
    }
    println!("The number is not even!");
    false
}

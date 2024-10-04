// Main function
fn main() {
    // Create a vector of integers
    let list = vec![34, 50, 25, 100, 65];
    // Create a vector of floating-point numbers
    let list_2 = vec![1.2,  3.4, 5.6];
    // Print the largest number in the integer list
    println!("The largest number is {}", largest(&list));
    // Print the largest number in the floating-point list
    println!("The largest number is {}", largest(&list_2));
}

// Function to find the largest integer in a list
fn largest(list: &[i32]) -> &i32 {
    let mut result = &list[0];
    for item in list {
        if item > result {
            result = item;
        }
    }
    result
}

// Function to find the largest floating-point number in a list
fn largest_float(list: &[f32]) -> &f32 {
    let mut result = &list[0];
    for item in list {
        if item > result {
            result = item;
        }
    }
    result
}

// --------------------------------------------------------------------

// Main function
fn main() {
    // Create a vector of integers
    let number_list = vec![34, 50, 25, 100, 65];
    // Create a vector of floating-point numbers
    let list_2 = vec![1.2,  3.4, 5.6];
    // Find the largest number in the integer list
    let result = largest(&number_list);
    // Print the result
    println!("The largest number is {}", result);
}

// Generic function to find the largest element in a list
fn largest<A>(list: &[A]) -> &A {
    let mut result = &list[0];
    for item in list {
        if item > result {
            result = item;
        }
    }
    result
}

// --------------------------------------------------------------------

// Define a User struct
struct User {
    email: String,
}

// Main function
fn main() {
    // Create a vector of integers
    let number_list = vec![34, 50, 25, 100, 65];
    // Create a vector of floating-point numbers
    let list_2 = vec![1.2,  3.4, 5.6];
    // Find the largest number in the integer list
    let result = largest(&number_list);
    // Print the result
    println!("The largest number is {}", result);

    // Create a vector of User structs
    let users = vec![
        User {
            email: String::from("someone@example.com"),
        },
        User {
            email: String::from("sometwo@example.com"),
        },
    ];
}

// Generic function to find the largest element in a list
// The type A must implement the PartialOrd trait
fn largest<A: std::cmp::PartialOrd>(list: &[A]) -> &A {
    let mut result = &list[0];
    for item in list {
        if item > result {
            result = item;
        }
    }
    result
}

// --------------------------------------------------------------------

// Define a generic Point struct with one type parameter
struct Point<T> {
    x: T,
    y: T,
}

// Main function
fn main() {
    // Create Point instances with different types
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // This won't work because x and y must be of the same type
    // let wont_work = Point { x: 5, y: 4.0 };
}

// --------------------------------------------------------------------

// Define a generic Point struct with two type parameters
struct Point<T, U> {
    x: T,
    y: U,
}

// Main function
fn main() {
    // Create Point instances with different type combinations
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

// --------------------------------------------------------------------

// Define a generic Point struct with two type parameters
struct Point<T, U> {
    x: T,
    y: U,
}

// Implement methods for the Point struct
impl<T, U> Point<T, U> {
    // Constructor method to create a new Point
    fn new(x: T, y: U) -> Self {
        Self {
            x,
            y,
        }
    }
}

// Main function
fn main() {
    // Create Point instances using the new method
    let point_a = Point::new(1, 2);
    let point_b = Point::new(1.0, 2.0);
    let point_c = Point::new(1, 2.0);
    let point_d = Point::new(1.0, 2);
}

// --------------------------------------------------------------------

// Implement a specific method for Point<f64, f64>
impl Point<f64, f64> {
    // Calculate the distance from the origin (0, 0)
    fn calculate_distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Main function
fn main() {
    // Create a Point with f64 coordinates
    let point_b = Point::new(1.0, 2.0);
    // This won't work because calculate_distance is only implemented for f64 coordinates
    // let wont_work = Point::new(5, 4.0);

    // Calculate and print the distance
    println!("{}", point_b.calculate_distance());
}

// --------------------------------------------------------------------

// Define a generic Point struct with two type parameters
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// Implement methods for the Point struct
impl<T, U> Point<T, U> {
    // Calculate the distance from the origin (0, 0)
    // Note: This implementation is incorrect and won't compile
    fn calculate_distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // Mix up the x and y values of two points
    fn mixup<X, Y>(self, other: Point<X, Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Main function
fn main() {
    // Create two points
    let point_a = Point::new(1.0, 2.0);
    let point_b = Point::new(5, 4.0);

    // Mix up the points
    let point_c = point_a.mixup(point_b);
    // Print the mixed-up point's coordinates
    println!("{} {}", point_c.x, point_c.y);
}
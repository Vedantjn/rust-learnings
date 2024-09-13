// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,   
// }

// struct Color(u8, u8, u8);
// struct Point(u8, u8, u8);

// fn main() {
//     // let mut user_1 = User {
//     //     email: String::from("vedant.jain.indore@gmail.com"),
//     //     username: String::from("VedantJain"),
//     //     active: true,
//     //     sign_in_count: 0,
//     // };
    
//     // let user_2 = User {
//     //     email: String::from("john.indore@gmail.com"),
//     //     username: String::from("John"),
//     //     active: true,
//     //     sign_in_count: 0,
//     // };

//     // println!("The name od user is {}", user_1.username);
//     // user_1.username = String::from("KartikJain");
//     // println!("The name od user is {}", user_1.username);
//     // user_1.username.push_str("SamyakJain");
//     // println!("The name od user is {}", user_1.username);

//     // let s1 = user_1.username;
//     // user_1.username = String::from("This is a value after move");
//     // println!("Value od username is {}", user_1.username);


    
//     // let user_1 = build_user(
//     //     String::from("VedantJain"), 
//     //     String::from("vedant@gmail.com")
//     // );

//     // let user_2 = User {
//     //     email: String::from("ved@gmail.com"),
//     //     active: false,
//     //     ..user_1
//     // };

//     // // println!("Value od username is {}", user_1.username); // Value moved
//     // println!("Value od username is {}", user_2.username);


    
//     let red = (100, 0, 0);
//     set_bg_color(red);

//     let point = Point(30, 40, 90);
//     move_point(point);

// }


// fn build_user(username: String, email: String) -> User {
//     // User {
//     //     username: username,
//     //     email: email,
//     //     active: true,
//     //     sign_in_count: 0,
//     // }

//     User {
//         username,
//         email,
//         active: true,
//         sign_in_count: 0,
//     }
// }

// // RGB
// fn set_bg_color(color: (u8, u8, u8)) {
//     println!("Setting background color R = {}, G = {}, B = {}", color.0, color.1, color.2);
// }

// fn move_point(point: Point) {
//     println!("The cursor was moved Y={} X={} Z={}", point.0, point.1, point.2);
// }



#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let w = 100;
    // let h = 200;

    // let rect = (30, 40);

    let rect = Rectangle {
        width: 32,
        height: 22,
    };

    // let area = calculate_area(rect);

    let area = dbg!(calculate_area(&rect));

    let area = calculate_area(&rect);

    // println!("The area is {}", area);

    // println!("The area of {:?} is {}", rect, area);

    // dbg!(rect);

    // dbg!(&rect);

    println!("The area of {:#?} is {}", rect, area);
}

// fn calculate_area(width: u32, height: u32) -> u32 {
//     width*height
// }

// fn calculate_area(dimensions:(u32, u32)) -> u32 {
//     let (width, height) = dimensions;
//     width*height
// }

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width*rect.height
}
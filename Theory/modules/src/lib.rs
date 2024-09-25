pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {
            // Absolute path
            crate::front_of_house::hosting::seat_at_table();

            // Relative path
            super::hosting::seat_at_table();
        }

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in summer with Wheat toast
    let mut meal = back_of_house::Breakfast::summer("Wheat");

    // Change the toast choice
    meal.toast = String::from("Rye");
    println!("I'd like {} toast, please.", meal.toast);

    // We cannot access seasonal_fruit since it's private
    // println!("The fruit is {}", meal.seasonal_fruit); // This line would cause an error
}

// Similar to abstract class & interfaces in JS and implementing them
// Trait is like a blueprint that other structs may follow
// Struct implements trait

trait Summary {
    fn summarise(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarise(&self) -> String {
        return format!("The name is {} and the age is {}", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("Vedant"),
        age: 22
    };

    println!("{}", user.summarise());
}

// ----------------------------------------------------------------------------

trait Summary {
    // Default Implementation
    fn summarise(&self) -> String {
        return String::from("Hi there");
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    // fn summarise(&self) -> String {
        //     return format!("The name is {} and the age is {}", self.name, self.age);
        // }
}
    
    fn main() {
        let user = User {
        name: String::from("Vedant"),
        age: 22
    };
    
    println!("{}", user.summarise());
}

// ----------------------------------------------------------------------------

// Traits as Parameters

trait Summary {
    // Default Implementation
    fn summarise(&self) -> String {
        return String::from("Hi there");
    }
}

struct User {
    name: String,
    age: u32,
}

struct Fix;

impl Summary for User{}
impl Summary for Fix{}

fn notify(u: impl Summary) {
    println!("{}", u.summarise());
}

    
fn main() {
    let user = User {
        name: String::from("Vedant"),
        age: 22
    };
    notify(user);

    let f = Fix;
    notify(f);
    
    println!("{}", user.summarise());
}


// Trait Bound -> syntactic sugar

pb fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item)
}

// ----------------------------------------------------------------------------

// Multiple Trait Bound -> syntactic sugar

pb fn notify<T: Summary + Fix>(item: &T) {
    println!("Breaking news! {}", item)
}

trait Summary {
    fn summarise(&self) -> String {
        return String::from("Hi there");
    }
}

trait Fix {
    fn fix(&self) -> String {
        return String::from("Hi there from fix");
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary as User ()
impl Fix as User ()

fn main() {
    let user = User {
        name: String::from("Vedant");
        age: 22
    };

    notify(user);
}

fn notify<T: Summary + Fix>(u: T) {
    println!("{}", u.summarise());
}
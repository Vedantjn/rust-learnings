fn main() {
    let my_string = String::from("My name is vedant");
    let length = get_string_length(&my_string);
    println!("{}", length);

}

fn get_string_length(s: &str) -> usize {
    s.chars().count()
}
use std::collections::HashMap;

fn main() {
    // Create a new, empty HashMap to store scores
    let mut scores = HashMap::new();
    
    // Create team names as String types
    let blue_team = String::from("Blue");
    let yellow_team = String::from("Yellow");

    // Insert scores for Blue and Yellow teams
    scores.insert(blue_team, 10);
    scores.insert(yellow_team, 50);

    // Insert a new team (Green) with a default score of 0 if it doesn't exist
    scores.entry(String::from("Green")).or_insert(0);

    // Iterate over the HashMap and print each team's score
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Get the score for the Blue team, or 0 if it doesn't exist
    // let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    let score = scores.get(&String::from("Blue")).unwrap_or(&0);
    println!("Score = {}", score);

    // Count word occurrences in a string
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // Print the word count map
    println!("{:?}", map);
}

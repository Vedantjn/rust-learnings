fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is ocnfigured to be {max}"),
        _ => (),
    }
}

// ---------------------------------------------------------------------------

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is ocnfigured to be {max}");
    }
}

// ---------------------------------------------------------------------------

fn main() {
    let config_max: Option<u8> = None;
    if let None = config_max {
        println!("There is no configuration");
    }
}

// ---------------------------------------------------------------------------


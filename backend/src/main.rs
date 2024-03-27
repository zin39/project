use std::io;

struct Input {
    location: String,
    quantity: i32,
    floor: i32,
}

impl Input {
    fn new(location: String, quantity: i32, floor: i32) -> Input {
        Input {
            location,
            quantity,
            floor,
        }
    }
}
#[derive(Debug)]
enum LocationCategory {
    IBlock,
    FBlock,
    GBlock,
    HBlock,
}

fn classify_location(location: &str) -> (LocationCategory, Option<i32>) {
    let mut category = LocationCategory::IBlock;
    let mut driver = None;

    let mut chars = location.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_alphabetic() {
            match c {
                'I' => category = LocationCategory::IBlock,
                'F' => category = LocationCategory::FBlock,
                'G' => category = LocationCategory::GBlock,
                'H' => category = LocationCategory::HBlock,
                _ => (),
            }
            // Check if there's a number after the letter
            if let Some(next) = chars.peek() {
                if next.is_numeric() {
                    driver = Some(next.to_digit(10).unwrap() as i32);
                    break;
                }
            }
        }
    }

    (category, driver)
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_int_input(prompt: &str) -> i32 {
    loop {
        let input = get_input(prompt);
        match input.parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid integer."),
        }
    }
}

fn main() {
    let location = get_input("Enter location:");
    let quantity = get_int_input("Enter quantity:");
    let floor = get_int_input("Enter floor:");

    let (category, driver) = classify_location(&location);
    println!("Location Category: {:?}", category);
    if let Some(driver) = driver {
        println!("Driver: {}", driver);
    } else {
        println!("No driver assigned.");
    }

    let input = Input::new(location, quantity, floor);

    println!("Quantity: {}", input.quantity);
    println!("Floor: {}", input.floor);
}


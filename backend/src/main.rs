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
        let c_lower = c.to_ascii_lowercase();
        if c_lower.is_alphabetic() {
            match c_lower {
                'i' => {
                    category = LocationCategory::IBlock;
                    driver = Some(1);
                }
                'f' => {
                    category = LocationCategory::FBlock;
                    driver = Some(2);
                }
                'g' => {
                    category = LocationCategory::GBlock;
                    driver = Some(3);
                }
                'h' => {
                    category = LocationCategory::HBlock;
                    driver = Some(4);
                }
                _ => (),
            }

            // Continue to process remaining data

            while let Some(next) = chars.next() {
                if next.is_numeric() {
                    // assign the numeric character to the same driver determined by the alpahbet
                    driver = Some(driver.unwrap_or(1));
                    break;
                }
            }
            // printing all the values that driver has been assigned.
            // Print details before returning
            println!("Category: {:?}, Driver: {:?}", category, driver);
            // Break the loop once processing is done
            break;
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

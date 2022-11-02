// Errors!

use core::fmt;

// Define my custom error, which is just an empty struct
struct CustomError;

// We should implement Display so we can print the error
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "you got a custom error")
    }
}

// First, I just want to write a function that returns an error on purpose
fn this_should_error() -> Result<(), CustomError>{
    return Err(CustomError)
}

// Then, let's wrap that function with another, and propagate the error
fn handle_error_with_string() -> String {
    match this_should_error() {
        Ok(_) => "it's okay".to_owned(),
        Err(_) => "it's broken".to_owned(),
    }
}


fn main() {
    // we should get an error here
    match this_should_error() {
        Ok(_) => println!("it worked but it shouldnt"),
        Err(e) => println!("{}", e),
    };

    // this should be broken
    let msg = handle_error_with_string();
    println!("{}", msg);
}

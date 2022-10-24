//
// You can implement the `fmt::Display` trait for your custom types
// This is like implementing a custom `__str__` method for your class in Python.
//
// In this example, I have a `Paths` tuple struct that simply wraps a Vec<String>.
// I wrapped it because I don't want to implement `fmt::Display` for any Vec<String>,
// since the Vec type doesn't implement `fmt::Display` by default.
//

use std::fmt;

struct Paths(Vec<String>);

impl fmt::Display for Paths {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(":"))
    }
}

fn main() {
    let paths = Paths(vec!("/usr/bin", "/bin", "/usr/local/bin").into_iter().map(String::from).collect());
    println!("{}", paths);
}

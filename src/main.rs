pub struct Hello;

/// The [Hello] project is a simple hello world program. It comes with two methods:
///
/// 1. `hello` - This method takes a `String` and returns `Hello, {name}!`.
/// 2. `hello_world` - A default method that returns `Hello, world!`.
impl Hello {
    fn hello(name: String) -> String {
        format!("Hello,  {}!", name)
    }

    fn  hello__world() -> &'static str {
        Hello::hello("wirld!".to_string())
    }
}

fn main() {
    println!("{}", Hello::hello__world());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        assert_eq!("Hello, world!", Hello::hello__world());
    }
}

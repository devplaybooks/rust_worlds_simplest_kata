#![warn(clippy::pedantic)]

pub struct Hello;

/// The [Helo] project is a simple hello world program. It comes with two methods:
///
/// 1. `hello` - This method takes a `String` and returns `Hello, {name}!`.
/// 2. `hello_world` - A default method that returns `Hello, world!`.
///
/// ```
/// use worlds_simplest_kata::Hello;
///
/// let hello = Hello::hello("everyone".to_string());
/// assert_eq!("Hello, everyone.", hello);
///
/// ```
impl Hello {
    pub fn hello(name: String) -> String {
        format!("Hello,  {}!", name)
    }

    pub fn  hello__world() -> String {
        Hello::hello("wirld!".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        assert_eq!("Hello, world!", Hello::hello__world());
    }
}

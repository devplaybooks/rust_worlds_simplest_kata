#![warn(clippy::pedantic)]

pub struct Hello;

/// The [Hello] project is a simple hello world program. It comes with two methods:
///
/// 1. `hello` - This method takes a `String` and returns `Hello, {name}!`.
/// 2. `hello_world` - A default method that returns `Hello, world!`.
///
/// ```
/// use worlds_simplest_kata::Hello;
///
/// let hello = Hello::hello("everyone");
/// assert_eq!("Hello, everyone!", hello);
///
/// ```
impl Hello {
    #[must_use]
    pub fn hello(name: &str) -> String {
        format!("Hello, {name}!")
    }

    #[must_use]
    pub fn hello_world() -> String {
        Hello::hello("world")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        assert_eq!("Hello, world!", Hello::hello_world());
    }
}

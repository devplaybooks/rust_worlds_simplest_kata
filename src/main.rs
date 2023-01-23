pub struct Hello;

impl Hello {
    fn  hello__world() -> &'static str {
        "Hello, wirld!"
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

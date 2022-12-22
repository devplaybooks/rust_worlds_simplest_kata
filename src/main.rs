fn hello__world() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("{}", hello__world());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        assert_eq!("Hello, world!", hello__world());
    }
}

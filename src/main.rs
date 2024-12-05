fn main() {
    println!("Hello, world!");
}

fn hello_world() -> String {
    "Hello World".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test()
    {
        let result: String = hello_world();
        assert_eq!("Hello World", result);
    }
}
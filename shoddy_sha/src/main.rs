fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it_works() {
        assert_eq!(2+2, 4);
    }
}

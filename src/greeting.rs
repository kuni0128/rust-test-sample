pub fn greeting(name: &str) -> String {
    // for test failed check
    // String::from("Hello!")

    format!("Hello {}!", name)
}

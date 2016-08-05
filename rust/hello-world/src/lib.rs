pub fn hello(s: Option<&str>) -> String {
    format!("Hello, {}!", s.unwrap_or("World"))
}
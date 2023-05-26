fn main() {
    my_func("Hello, world!");
}

fn my_func(text: &str) {
    println!("{}", text.to_string());
}
fn greet(name: &str) {
    println!("Hello {}!", name);
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn get_coordinates() -> (f64, f64) {
    (34.0, -118.2)
}

fn main() {
    greet("Alice");
    println!("Is 4 even: {}", is_even(4));
    println!("Is 3 even: {}", is_even(3));
    let (x, y) = get_coordinates();
    println!("x: {}, y: {}", x, y);
}

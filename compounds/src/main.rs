fn greet(name: &str) {
    println!("Hello {}!", name);
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn get_coordinates() -> (f64, f64) {
    (34.0, -118.2)
}

fn check_number(num: i32) {
    let mut result = "Zero";
    if num > 0 {
        result = "Positive";
    } else if num < 0 {
        result = "Negative";
    }
    println!("{}", result);
}

fn countdown(n: u32) {
    let mut counter = n;
    while counter > 0 {
        println!("{}", counter);
        counter -= 1;
    }
    println!("Liftoff!");
}

fn sum_of_squares(arr: [i32; 5]) -> i32 {
    let mut ss = 0;
    for element in arr {
        ss += element * element;
    }
    ss
}

fn main() {
    check_number(20);
    check_number(0);
    check_number(-20);

    countdown(5);

    let my_array = [1, 2, 3, 4, 5];
    let ss = sum_of_squares(my_array);
    println!("Sum of square or my array: {}", ss);
}

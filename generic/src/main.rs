struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value: value }
    }

    fn get_value(&self) -> &T {
        &self.value
    }

    fn set_value(&mut self, new_value: T) {
        self.value = new_value
    }
}

fn swap<T>(val1: &mut T, val2: &mut T) {
    std::mem::swap(val1, val2);
}

fn main() {
    let container1 = Container::new(10);
    println!("container1 value: {}", container1.get_value());

    let mut container2 = Container::new("Hello".to_string());
    println!("container2 value: {}.", container2.get_value());
    container2.set_value("World".to_string());
    println!("container2 new value: {}.", container2.get_value());

    let mut a = 10;
    let mut b = 20;
    println!("Before swap:\na: {}, b: {}", a, b);
    swap(&mut a, &mut b);
    println!("After swap:\na: {}, b: {}", a, b);

    let mut a = "This is a";
    let mut b = "This is b";
    println!("Before swap:\na: {}, b: {}", a, b);
    swap(&mut a, &mut b);
    println!("After swap:\na: {}, b: {}", a, b);
}

use chrono::Local;
use std::fs::read_to_string;

struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    println!("{}", is_even(17));
    println!("{}", fibonacci(8));

    let name = String::from("yash");
    let len = get_str_len(name);
    println!("The length of the string is {}", len);

    let user = User {
        first_name: String::from("Yash"),
        last_name: String::from("Thakur"),
        age: 22,
    };

    let my_direction = Direction::South;
    let my_shape = Shape::Rectangle(10.0, 20.0);
    println!("{}", user.first_name);

    let index = find_first_a(String::from("preet"));

    match index {
        Some(value) => println!("Index is {}", value),
        None => println!("a not found"),
    }
    let result = read_to_string("a.txt");

    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("Error while reading the file"),
    }

    let time = Local::now();
    println!("Current time is {}", time);
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

enum Direction {
    North,
    East,
    South,
    West,
}

enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => r * r,
    };
    return area;
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

fn fibonacci(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }

    for _ in 1..(num - 1) {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}

fn get_str_len(str: String) -> usize {
    str.chars().count()
}

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

    println!("{}", user.first_name);
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

use std::{collections::HashMap, iter::Filter};

fn main() {
    //Ve1utors
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    let vec2 = vec![1, 2, 3];

    println!("{:?}", vec2);

    println!("{:?}", vec);
    println!("{:?}", even_filter(&vec));

    //HashMap
    let mut users = HashMap::new();

    users.insert(String::from("Yash"), 22);
    users.insert(String::from("Aditya"), 21);

    let first_user_age = users.get("Aditya");
    match first_user_age {
        Some(age) => println!("Age is {}", age),
        None => println!("User not found  in the database"),
    }

    let input_vec = vec![(String::from("Yash"), 22), (String::from("Aditya"), 21)];
    let hm = group_values_by_key(input_vec);
    println!("{:?}", hm);

    //iterators
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{}", val)
    }

    let v1 = vec![10, 2, 3, 4];
    let v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();
    println!("{}", sum);

    let v1 = vec![1, 2, 3, 4, 5, 6];

    let v1_iter = v1.iter();

    let v1_iter2 = v1_iter.filter(|x| *x % 2 == 0).map(|x| x * 2);

    for val in v1_iter2 {
        println!("{}", val);
    }
}

//Vectors
fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }
    return new_vec;
}

//HashMap
fn group_values_by_key(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();

    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}

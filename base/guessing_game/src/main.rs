use std::{cmp::Ordering, io, string};
use rand::Rng;
struct Color(u8, u8, u8);
struct AlwaysEqual;

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match  coin {
        Coin::Penny => {println!("Lucky Penny!"); 1}
        Coin::Nickle => 5,
        Coin::Dime => 10,
        other => 20
    }
}

fn main() {
    let config_max = Some(0u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max1 = Some(3u8);
    if let Some(max1) = config_max1 {
        println!("The maximum is configured to be {}", max1);
    }

    let coin = Coin::Penny;
    let coin_val = value_in_cents(coin);


    let rect1 = Rectangle {width: 30, height: 51};
    println!("The area of rectangle is {} square pixels.", rect1.area());
    println!("The rectangle has a nonzero width; it is {}", rect1.width());

    let rect2 = Rectangle {width: 16, height: 25};
    let rect3 = Rectangle {width: 30, height: 50};

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    let x = Some(5);
    
    let black = Color(0,0,0);
    return;
    println!("color: {}", black.0);
    let mut user1 = build_user(String::from("cyh123"), String::from("cyh@example.com"));
    println!("user1 is {:?}", user1);
    
    let mut user2 = build_user(dbg!(String::from("cyh123")), String::from("cyh@example.com"));
    dbg!(user2);    

    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word is {word}");
    s.clear();
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.into_iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other:&Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
}
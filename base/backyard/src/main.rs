use crate::garden::vegetable::Asparagus;
pub mod garden;
use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;
use std::fs::File;
use std::io::ErrorKind;
pub mod front_of_house;
use crate::front_of_house::hosting;
use backyard::{self, NewsArticle, Summary, Tweet};

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }        
    }
    largest
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }        
//     }
//     largest
// }

struct Point<T> {
    x:T,
    y:T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl  Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct Point1<T, U> {
    x:T,
    y:U,
}

impl<T, U> Point1<T, U> {
    fn mixup<T1, U1>(self, other: Point1<T1, U1>) -> Point1<T,U1> {
        Point1 { x: self.x, y: other.y, }
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people",),
        reply:false,
        retweet:false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
            hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
    println!("New article available! {}", article.summarize_author());

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer.x is {}", integer.x());
    println!("float distance is {}", float.distance_from_origin());
    let integer_float = Point1 { x: 1, y: 4.0 };
    let string_pt = Point1 { x: "hello", y: 'c' };

    let p = string_pt.mixup(integer_float);
    println!("p3.x = {}, p3.y = {}", p.x, p.y);



    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 6000);
    return;

    let greeting_file_result = File::open("hellof.tzt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match  File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
    // panic!("crash and burn");
    let v = vec![1, 2, 3];

    v[99];
    let text = String::from("hello world wonderful world");
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = &scores[&team_name];
    println!("Blue is {score}");
    let score1 = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue is {score1}");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("string is {s}");
    s += " section.";
    println!("string is {s}");
    
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    let hello = String::from("السلام عليكم");
    let hello1 = String::from("Dobrý den");
    let hello2 = String::from("Hello");
    let hello4 = String::from("שָׁלוֹם");
    for c in hello4.chars() {
        println!("{c}");
    }
    for b in hello4.bytes() {
        println!("{b}");
    }
    let hello3 = String::from("नमस्ते");
    let hello5 = String::from("こんにちは");
    let hello7 = String::from("你好");
    let hello8 = String::from("Olá");
    let hello9 = String::from("Здравствуйте");
    let hello10 = String::from("Hola");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),];
    
    let mut v = Vec::new();
    v.push(3);
    let a = &v[0];
    let mut v1 = vec![1,2,3];

    println!("vector is {}", a);
    println!("vector1 is {}", &v1[0]);
    let second = v1.get(1);
    match second {
        Some(second) => println!("The second element is {second}"),
        None => println!("There is no second element."),
    }

    for i in &v1 {
        println!("{i}");
    }

    for i in &mut v1 {
        *i += 50;
    }

    println!("vector1 is {}", v1[0]);

    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    let hosting = hosting::add_to_wait_list();
    println!("hosting is {:?}!", hosting);
}

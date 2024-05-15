use blog::Post;
use std::f32::consts::FRAC_PI_2;
extern "C" {
    fn abs(input: i32) -> i32;
    fn sin(input: f64) -> f64;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        println!(
            "Sinus value of 1.570796327 according to C: {}",
            sin(FRAC_PI_2.into())
        );
    }
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

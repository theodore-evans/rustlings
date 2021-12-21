// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer = square;
    let real_answer = answer(3);
    println!("The answer is {}", real_answer);
}

fn square(num: i32) -> i32 {
   num * num
}

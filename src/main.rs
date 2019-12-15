extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input you Guess.");
    let _secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is:{}", _secret_number);
    diverges();
    let _x: i32 = diverges();
    let _x: String = diverges();
    let _f: fn(i32) -> i32;
    let _f = plus_one;
    _f(1);
    let _six = _f(5);
    let mut x: i32 = 1;
    x = 7;
    let _x = x; // x is now immutable and is bound to 7
    let _y = 4;
    let _y = "I can also be bound to text"; //`y` is now of a different type
    print_number(5);
    println!(" The number to add one:{}", add_one(34));
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let x = 6;
        let y = 10;
        println!("x and y {} and {}", x, y);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);
        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
fn print_number(x: i32) {
    println!("x is {}", x);
}
fn add_one(x: i32) -> i32 {
    x + 1
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn diverges() -> ! {
    panic!("This function never returns!");
}
fn arrays_show() -> ! {
    let _a = [1, 2, 3]; //a:[i32,3]
    let mut m = [1, 2, 3]; //m:[i32;3]
    let _a = [0; 20]; //There is an shorthand for initializing each elment of an array to the same value
    let _a = [1, 2, 3];
    let _names = ["Graydon", "Brian", "Niko"]; //names:[&str;3]
    println!("The second name is :{}", _names[1]);
    println!("a has {} elments", _a.len());
    panic!("This function never returns!");
}
fn primitive_types() -> ! {
    let _x = true;
    let _y: bool = false;
    let _x = 'x';
    let _two_hearts = '※';
    let _x = 42; // x has type i32
    let _y = 1.0; //y has type `f64`
    panic!("This function never returns!");
}
fn slice_array() {
    //a portion of an array without copying.
    let _a = [1, 2, 3, 4, 5];
    let _complete = &_a[..]; //A slice containing all of the elments in `a`
    let _middle = &_a[1..4]; //A slice of `a`:only the elments `1`,`2`, and `3`.
}
fn tuples() {
    let _x = (1, "elaborate"); // We will elaborate further when we cover Strings and references.
    let _x: (i32, &str) = (1, "parentheses and commas heterogeneous heterogeneous");
}
fn you_can_assign_one_tuple_into_another() {
    let mut _x = (1, 2); //x:(i32,i32)
    let y = (2, 3);
    _x = y;
}
fn functions_also_have_a_type() {
    fn foo(x: i32) -> i32 {
        x
    }
    let x: fn(i32) -> i32 = foo; // in this case ,x is an 'function pointer' to a function that takes an i32  and returns an i32;
}
fn if_fun_grasp_the_nuances() {
    let _x = 5;
    if _x == 5 {
        println!("x is five!");
    }
    let y = if _x == 5 { 10 } else { 15 };
}
fn while_fn(arg: i32) -> i32 {
    let mut x = 5;
    let mut done = false;
    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            done = true;
        }
    }
    x
}
fn slice_syntax(arg: i32) -> i32 {
    let a = [0, 1, 2, 3, 4];
    let _complete = &a[..]; //A slice containing all of the elements in `a`;
    let _middle = &a[1..4]; //A slice of a :only the elments 1,2,3;
    7
}

fn foo(){
    let v= vec![1,2,3];

    let _v2=&v;//Rust ensures that there is exactly one binding to any given resource .
    println!("v[0] is {}",v[0]);
    //Rust ensures that there is exactly one binding to any given resource.
    //Rust ensures that there is exactly one bingding to any given resource.
    //Rust ensures that there is exactly one binding to any given resource.
    //Rust ensures that there is exactly one  binding to any given resource. 
    //Rust ensures that there is exactly one binding to any given resource.
    //Rust ensures that there is exactl on binding to any given resource.
    let a = 5;

    let _y = double(a);
    println!("{}", a);

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let (v1, v2, answer) = foo1(v1, v2);
}
fn foo1(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // Do stuff with `v1` and `v2`.

    // Hand back ownership, and the result of our function.
    (v1, v2, 42)
}
fn double(x: i32) -> i32 {
    x * 2
}
fn vect_test(arg: i32) -> i32 {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

        for i in v {
            println!("Take ownership of the vector and its element {}", i);
        }
    1
}
fn is_alpha_numeric(cc: u8) -> bool {
    match cc {
        97...122 | 65...90 | 48...57 => {
            true
        }
        _ => {
            false
        }
    }
}
// parameters
fn example(x: i32, y: f32) {
    println!("{}, {}", x, y);
}

// scope
fn expressions() {
    let x: i32 = 10;
    {
        let x: i32 = 10 + 1;
        println!("{}", x);
    }
    println!("{}", x);
}

// assigning vars
// return type
fn assigning_vars() -> i32 {
    let y: i32 = 10;
    let x: i32 = { y + 10 };
    println!("{} {}", x, y);
    return x;
}

pub fn learn() {
    example(10, 10.1);
    expressions();
    assigning_vars();
}

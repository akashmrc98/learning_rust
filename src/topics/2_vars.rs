pub fn learn() {
    // shadowing in rust
    // as it able rewrite
    // it can able to change data type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces :{}", spaces);

    // mutation
    // as it tries to modifies
    // it cant able to change data type
    let mut x: i32 = 10;
    x += 1;
    println!("spaces :{}", x);
}

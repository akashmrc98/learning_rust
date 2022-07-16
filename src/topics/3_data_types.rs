pub fn learn() {
    // signed bit ints
    let x1: i8 = 1;
    let x2: i16 = 1;
    let x3: i32 = 1;
    let x4: i64 = 1;
    let x5: i128 = 1;
    let x6: isize = 1;
    println!("signed int {}, {}, {}, {}, {}, {}", x1, x2, x3, x4, x5, x6);

    // unsigned bit ints
    let x1: u8 = 1;
    let x2: u16 = 1;
    let x3: u32 = 1;
    let x4: u64 = 1;
    let x5: u128 = 1;
    let x6: usize = 1;
    println!(
        "unsigned int {}, {}, {}, {}, {}, {}",
        x1, x2, x3, x4, x5, x6
    );

    // floats
    let x1: f32 = 1.09;
    let x2: f64 = 112.20;
    println!("floats {}, {}, ", x1, x2);

    // boolean
    let x1: bool = true;
    println!("bool {}", x1);

    // char
    let heart_eyed_cat: char = '😻';
    println!("chars {}", heart_eyed_cat);

    // compound types like tuples
    let tup: (i32, f64, u8) = (-500, 100.12, 10);
    println!("{:?}", tup);

    // static arrays
    let arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("{:?}", arr);

    let arr_one: [i32; 5] = [3; 5];
    println!("{:?}", arr_one);

    // accessing array elements
    println!("{}", arr[0]);
}

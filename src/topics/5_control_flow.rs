pub fn learn() {
    const X_VALUE: u8 = 8;
    if X_VALUE <= 12 {
        println!("OK")
    } else {
        println!("NOT OK")
    }

    // this code throws error as value should be boolean
    // if X_VALUE {
    //     println!("ERROR");
    // }

    if X_VALUE % 2 == 0 {
        println!("{} value is divisible by 2", { X_VALUE });
    } else if X_VALUE % 4 == 0 {
        println!("{} value is divisible by 4", { X_VALUE });
    } else {
        println!("{} value is not divisible by 2, 4", { X_VALUE });
    }

    println!("----------------------------------------------");

    if X_VALUE % 2 == 0 {
        println!("{} value is divisible by 2", { X_VALUE });
    }
    if X_VALUE % 4 == 0 {
        println!("{} value is divisible by 4", { X_VALUE });
        return;
    }
    println!("{} value is not divisible by 2, 4", { X_VALUE });
}

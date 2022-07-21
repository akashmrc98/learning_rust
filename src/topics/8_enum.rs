enum Coin {
    USDC,
    BUSD,
    USDT,
}

fn get_coin_value_in_inr(coin: &Coin) -> f64 {
    match coin {
        Coin::BUSD => 78.1,
        Coin::USDC => 77.1,
        Coin::USDT => 80.1,
    }
}

pub fn learn() {
    let coin_value = get_coin_value_in_inr(&Coin::USDT);
    println!("the value of usdt is {}", coin_value);
    let coin_value = get_coin_value_in_inr(&Coin::BUSD);
    println!("the value of busd is {}", coin_value);
    let coin_value = get_coin_value_in_inr(&Coin::USDC);
    println!("the value of usdc is {}", coin_value);
    let coin_value = get_coin_value_in_inr(&Coin::USDC);
    println!("the value of usdc is {}", coin_value);

    let config_max = Some(1.2f64);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => println!("Default statement"),
    }

    //short hand for no default statements
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;

    let coin = &Coin::BUSD;

    if let Coin::BUSD = coin {
        println!("State quarter from {:?}!", "BUSD");
    } else {
        count += 1;
    }

    println!("count {}", count);
}

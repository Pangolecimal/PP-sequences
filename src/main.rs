#![allow(unused_imports, dead_code, non_snake_case, non_camel_case_types)]

use pp_task::{
    dammit::A133058,
    forest_fire::A229037,
    inventory::A342585,
    kolakoski::A000002,
    miller_rabin::{is_prime, primality_test},
    prime_bin_rev::A265326,
    remy_sigrist::A279125,
    stern_brocot::A002487,
    wisteria::A063543,
};

struct Plot {
    data: Vec<i32>,
}

fn main() {
    clear();

    let modes: Vec<String> = vec![
        "Primality Test".into(),
        "Inventory".into(),
        "Stern-Brocot".into(),
        "Kolakoski".into(),
        "Fly straight, dammit".into(),
        "Primes minus their binary reversal".into(),
        "Remy Sigrist".into(),
        "Wisteria".into(),
        "Forest Fire".into(),
    ];

    let mode = &*get_mode(modes);

    clear();

    match mode {
        "Primality Test" => primality_test(),
        "Inventory" => A342585(),
        "Stern-Brocot" => A002487(),
        "Kolakoski" => A000002(),
        "Fly straight, dammit" => A133058(),
        "Primes minus their binary reversal" => A265326(),
        "Remy Sigrist" => A279125(),
        "Wisteria" => A063543(),
        "Forest Fire" => A229037(),
        _ => println!("Invaid mode"),
    }
}

fn get_mode(modes: Vec<String>) -> String {
    println!("Modes: ");
    for i in 0..modes.len() {
        println!("- mode {}: {}", i, modes[i]);
    }
    println!();
    print!("Pick mode: ");

    let n: usize = get_input_range(modes.len() as i32) as usize;

    modes[n].clone()
}

/**
 * from 0 up to, but not including, MAX
 */
fn get_input_range(max: i32) -> i32 {
    let n_result: Result<i32, _> = get_input_line().parse();

    let mut n: i32 = match n_result {
        Ok(m) => m,
        Err(_) => -1,
    };

    if n < 0 || n >= max {
        n = -1;
    }

    if n == -1 {
        println!("Error: expected integer in range: [0, {}]", max);
        return get_input_range(max);
    }

    return n;
}

fn get_input_line() -> String {
    let mut input_line = String::new();
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    return input_line.trim().to_string();
}

fn clear() {
    print!("{esc}[2J{esc}[0;0H", esc = 27 as char);
}

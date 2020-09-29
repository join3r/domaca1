use rand::{thread_rng, Rng};
use std::{io, io::Write};

/// Opýta si vstup od užívateľa a predtým vypíše text
///
/// funguje tak ako keby bolo definované napríklad takto:
/// fn get_input_from_user(msg: &str) -> u64
/// alebo fn get_input_from_user(msg: u64) -> String
/// a rôzne kombinácie
pub fn get_input_from_user<T, U>(msg: U) -> T
where
    T: std::str::FromStr,
    U: std::fmt::Display,
{
    loop {
        let mut input = String::new();
        print!("{}", msg);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(res) => return res,
            Err(_) => println!("Invalid input"),
        };
    }
}

/// Vráti náhodné čislo medzi dvoma hodnotami
///
/// funguje tak ako keby bolo definované napríklad takto:
/// fn get_random_number(min: i32, max: i32) -> Result<i32, &str>
/// alebo fn get_random_number(min: u128, max: u128) -> Result<u128, &str>
/// atď
pub fn get_random_number<T>(min: T, max: T) -> Result<T, &'static str>
where
    T: rand::distributions::uniform::SampleUniform + std::cmp::PartialOrd,
{
    if min > max {
        Err("Minimálne číslo je väčšie ako maximálne")
    } else {
        let mut rng = thread_rng();
        Ok(rng.gen_range(min, max))
    }
}

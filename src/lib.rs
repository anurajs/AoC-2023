pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

use std::time::SystemTime;
use std::{env, fs};

use dotenv;
use reqwest::blocking::Client;
use reqwest::Method;

pub fn download_day(year: u16, day: u8) -> String {
    let path = &format!("day{day}.txt")[..];
    if let Ok(_) = fs::metadata(path) {
        return fs::read_to_string(path).unwrap();
    }
    dotenv::dotenv().unwrap();
    let cookie = env::var("AOC_COOKIE").expect("Expected cookie");
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = Client::new();
    let response = client
        .request(Method::GET, url)
        .header("Cookie", format!("session={cookie}"))
        .send()
        .unwrap()
        .text()
        .unwrap();

    fs::write(path, &response[..]).unwrap();

    response
}
pub fn timeit<F: Fn() -> T, T>(f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs_f64());
    result
}
#[cfg(test)]
mod tests {
    use super::download_day;

    #[test]
    fn sample_test() {
        println!("{}", download_day(2023, 1));
    }
}

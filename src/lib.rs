pub mod day_1;

use std::env;

use dotenv;
use reqwest::blocking::Client;
use reqwest::Method;

pub fn download_day(year: u16, day: u8) -> String {
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

    response
}

#[cfg(test)]
mod tests {
    use super::download_day;

    #[test]
    fn sample_test() {
        println!("{}", download_day(2015, 7));
    }
}

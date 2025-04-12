use reqwest::blocking;

fn main() {
    let client = blocking::Client::new();
    let url = "https://api.polar.sh/v1/benefits/1";
    let response = client.get(url).send();
    let body = response.unwrap().text().unwrap();
    println!("{}", body);
}

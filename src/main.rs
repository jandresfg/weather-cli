fn main() {
    let api_token = std::env::var("API_TOKEN").expect("expected there to be an api token");

    let mut arg_iterator = std::env::args();
    arg_iterator.next(); // to skip the binary name,  i.e, weather-cli
    let args: String = arg_iterator.collect();

    dbg!(args);
}

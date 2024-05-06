use dotenv::dotenv;

fn main() {
    dotenv().expect("Could not load the .env file.");
    println!(
        "Hello from main: {:?}",
        std::env::vars().collect::<Vec<(String, String)>>()
    )
}

use axum::{routing::get, Router};
mod ether;
mod init;
mod utils;
#[tokio::main]

async fn main() {
    // let (secret_key, pub_key) = ether::generate_keypair();

    // println!("secret key: {}", &secret_key.to_string());
    // println!("public key: {}", &pub_key.to_string());
    match init::init_wallet("account_config.json").await {
        Ok(account) => {
            println!("Wallet initialized successfully:");
            println!("{:?}", account);
            // You can access account fields here if needed
        }
        Err(e) => eprintln!("Error initializing wallet: {}", e),
    }

    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }));

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

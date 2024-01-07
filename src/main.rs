use clap::Parser;
use mal_cli::auth::get_access_token;
use mal_cli::init::initialize_client;
use mal_cli::list::show_list;

#[derive(Parser, Debug)]
struct Args {
    command: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("Command: {:?}", args.command);

    match args.command.as_str() {
        "init" => {
            initialize_client();
        }
        "login" => {
            get_access_token().await.unwrap();
            println!("Logged in...");
        }
        "list" => {
            show_list().await.unwrap();
        }
        _ => (),
    }
}

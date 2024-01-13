use clap::Parser;
use mal_cli::auth::get_access_token;
use mal_cli::init::initialize_client;
use mal_cli::list::{show_list, ListArgs};
use mal_cli::season::{show_season, SeasonArgs};

#[derive(Parser, Debug)]
struct Args {
    command: String,
    #[clap(long)]
    /// Number of results
    num: Option<i32>,
    #[clap(long)]
    /// Season of the anime
    season: Option<String>,
    #[clap(long)]
    /// Year of the season
    year: Option<i32>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command.as_str() {
        "init" => {
            initialize_client();
        }
        "login" => {
            get_access_token().await.unwrap();
        }
        "list" => {
            // shows a list of the current anime in your list
            show_list(ListArgs::new(args.num)).await.unwrap();
        }
        "season" => {
            // shows the anime in the current season
            show_season(SeasonArgs::new(args.season, args.year)).await.unwrap();
        }
        _ => (),
    }
}

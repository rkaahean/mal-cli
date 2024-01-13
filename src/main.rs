use clap::{Parser, Subcommand};
use mal_cli::init::initialize_client;
use mal_cli::list::{show_list, ListArgs};
use mal_cli::season::{show_season, SeasonArgs};

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Intialize the client Id
    Init {
        /// value of client ID
        value: String,
    },
    List {
        // Numer of anime to show in list
        num: Option<i32>
    },
    Season {
        // Get seasonal anime
        #[clap(long)]
        season: Option<String>,
        #[clap(long)]
        year: Option<i32>
    }

}


#[derive(Parser, Debug)]
struct Init {
    value: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Init { value } => {
            initialize_client(value);
        }
        Commands::List { num } => {
            // shows a list of the current anime in your list
            show_list(ListArgs::new(num)).await.unwrap();
        }
        Commands::Season { season, year } => {
            // shows the anime in the current season
            show_season(SeasonArgs::new(season, year)).await.unwrap();
        },
    }
}

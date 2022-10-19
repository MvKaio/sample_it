use clap::Parser;
use clap::Subcommand;

mod server;
mod database;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Add an item to the collection
    Add,
    /// Remove an item from the collection
    Remove,
    /// List items in the collection
    List,
    /// Generate a sample of the itens, given a set of restrictions
    Sample,
	/// Runs the web server
	Run
}

#[actix_web::main]
async fn main() {
    let args = Args::parse();

    match args.action {
        Action::Add => sample_it::add(),
        Action::Remove => sample_it::remove(),
        Action::List => sample_it::list(),
        Action::Sample => sample_it::sample(),
        Action::Run => server::run().await.unwrap()
    };
}

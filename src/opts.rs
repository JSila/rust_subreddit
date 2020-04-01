use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    #[clap()]
    pub subreddit: String,
}

pub fn get() -> Opts {
    Opts::parse()
}
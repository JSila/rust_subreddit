mod subreddit;
mod opts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = opts::get();

    let url = format!("https://www.reddit.com/r/{}/.json", opts.subreddit);

    let response = reqwest::blocking::get(&url)?
        .text()?;

    let subreddit_data = subreddit::get_data(&response)?;

    for post in subreddit_data.iter() {
        println!("{} (by {})", post.title, post.author);
    }

    Ok(())
}
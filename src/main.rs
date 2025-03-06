use chrono::{Duration, Local};
use clap::Parser;
use reqwest;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 100)]
    min_stocks: u32,
}

#[derive(Deserialize, Debug)]
struct QiitaItem {
    title: String,
    stocks_count: u32,
    url: String,
}

async fn fetch_popular_articles(min_stocks: u32) -> Result<Vec<QiitaItem>, reqwest::Error> {
    let today = Local::now();
    let one_week_ago = today - Duration::days(7);
    let date = one_week_ago.format("%Y-%m-%d").to_string();
    let query = format!("created:>={}+stocks:>={}", date, min_stocks);
    let url = format!("https://qiita.com/api/v2/items?query={}", query);
    let response = reqwest::get(&url).await?;
    let items = response.json::<Vec<QiitaItem>>().await?;
    Ok(items)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match fetch_popular_articles(args.min_stocks).await {
        Ok(mut items) => {
            if items.is_empty() {
                println!("No articles found.");
                return;
            }
            items.sort_by(|a, b| b.stocks_count.cmp(&a.stocks_count));
            println!(
                "Top popular articles (within 1 week, stocks >= {}):",
                args.min_stocks
            );
            for item in items.iter().take(items.len() - 1) {
                println!("Title: {}", item.title);
                println!("Stocks: {}", item.stocks_count);
                println!("URL: {}\n", item.url);
            }
        }
        Err(e) => eprintln!("Error fetching articles: {}", e),
    }
}

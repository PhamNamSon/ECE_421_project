use std::io;
use yahoo_finance_api as yahoo;
use time::{OffsetDateTime, Duration};
use tokio;
use plotters::{prelude::*, style::full_palette::ORANGE};

async fn fetch_stock_data(symbol: &str) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    let provider = yahoo::YahooConnector::new();
    let end = OffsetDateTime::now_utc();
    let start = end - Duration::days(180);
    let response = provider.get_quote_history(symbol, start, end).await?;
    let quotes = response.quotes()?;
    let closing_prices = quotes.iter().map(|quote| {
        let date = quote.timestamp as i64;
        let close = quote.close as f64;
        (date as f64, close)
    }).collect();
    Ok(closing_prices)
}

fn plot_stock_data(symbol: &str, data: &[(f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = format!("{}_chart.png", symbol);
    let root = BitMapBackend::new(&file_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let min_date = data.iter().map(|x| x.0).fold(f64::INFINITY, f64::min);
    let max_date = data.iter().map(|x| x.0).fold(f64::NEG_INFINITY, f64::max);
    let min_price = data.iter().map(|x| x.1).fold(f64::INFINITY, f64::min);
    let max_price = data.iter().map(|x| x.1).fold(f64::NEG_INFINITY, f64::max);
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Closing Prices: {}", symbol), ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_date..max_date, min_price..max_price)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        data.iter().map(|x| (x.0, x.1)),
        &ORANGE,
    ))?;
    Ok(())
}


#[tokio::main]
async fn main() {
    loop {
        println!("Enter a stock symbol to fetch data for or 'quit' to exit: ");
        let mut symbol = String::new();
        io::stdin().read_line(&mut symbol).expect("Failed to read line");

        if symbol.trim().to_lowercase() == "quit" {
            break;
        }

        match fetch_stock_data(&symbol.trim().to_lowercase()).await {
            Ok(data) => {
                if let Err(e) = plot_stock_data(&symbol, &data) {
                    println!("Failed to plot stock data: {}", e);
                } else {
                    println!("Stock data plotted successfully");
                }
            },
            Err(e) => println!("Failed to fetch stock data: {}", e),
        }
    }
}

use std::io;
use yahoo_finance_api as yahoo;
use time::{OffsetDateTime, Duration};
use chrono::{prelude::*, TimeZone};
use tokio;
use plotters::{prelude::*, style::full_palette::ORANGE};

async fn fetch_stock_data(symbol: &str) -> Result<Vec<(f64, f64, f64, f64, f64)>, Box<dyn std::error::Error>> {
    let provider = yahoo::YahooConnector::new();
    let end = OffsetDateTime::now_utc();
    let start = end - Duration::days(180);
    let response = provider.get_quote_history(symbol, start, end).await?;
    let quotes = response.quotes()?;
    let closing_prices = quotes.iter().map(|quote| {
        let date = quote.timestamp as i64;
        let close = quote.close as f64;
        let high = quote.high as f64; 
        let low = quote.low as f64; 
        let percent_change = ((high-low)/close) * 100 as f64;
        (date as f64, close, high, low, percent_change)
    }).collect();
    Ok(closing_prices)
}

fn plot_stock_data(symbol: &str, data: &[(f64, f64, f64, f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = format!("{}_chart.png", symbol);
    let root = BitMapBackend::new(&file_name, (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let min_date = data.iter().map(|x| x.0).fold(f64::INFINITY, f64::min);
    let max_date = data.iter().map(|x| x.0).fold(f64::NEG_INFINITY, f64::max);
    let min_price = data.iter().map(|x| x.1).fold(f64::INFINITY, f64::min);
    let max_price = data.iter().map(|x| x.1).fold(f64::NEG_INFINITY, f64::max);

    let volatile_data: Vec<(f64, f64, f64, f64, f64)> = data
        .iter()
        .filter(|&(_, _, _, _, percent)| percent > &(2 as f64))
        .cloned()
        .collect();

    let to_date = |x: f64| -> String {
        let datetime = Utc.timestamp_opt(x as i64, 0)
            .single()
            .expect("Invalid timestamp");
        datetime.format("%Y-%m-%d").to_string()
    };

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Closing Prices: {}", symbol), ("sans-serif", 50).into_font())
        .margin(25)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_date..max_date, min_price..max_price)?;

    chart.configure_mesh()
        .x_labels(15)
        .x_label_formatter(&|x| to_date(*x))
        .draw()?;
    chart.draw_series(LineSeries::new(
        data.iter().map(|x| (x.0, x.1)),
        &ORANGE,
    ))?;
    chart.draw_series(
        volatile_data.iter().map(|x| {
            PathElement::new(
                vec![(x.0, x.2), (x.0, x.3)],
                &BLUE,
            )
        })
    )?;
    chart.draw_series(
        volatile_data.iter().map(|x| {
            Circle::new(
                (x.0, x.1),
                3,
                &BLUE,
            )
        })
    )?;
    let offset = 100000.0;
    chart.draw_series(
        volatile_data.iter().map(|x| {
            PathElement::new(
                vec![((x.0)-offset, x.2), ((x.0)+offset, x.2)],
                &BLUE,
            )
        })
    )?;
    chart.draw_series(
        volatile_data.iter().map(|x| {
            PathElement::new(
                vec![((x.0)-offset, x.3), ((x.0)+offset, x.3)],
                &BLUE,
            )
        })
    )?;

    println!("\nVolatile Days (more than 2% variation):");
    for (date, _, high, low, percent_change) in volatile_data {
        println!("{}: High: ${:.2}, Low: ${:.2}, {:.2}% change", to_date(date), high, low, percent_change);
    }

    Ok(())
}


#[tokio::main]
async fn main() {
    loop {
        println!("Enter a stock symbol to fetch data for or 'quit' to exit: ");
        let mut symbol = String::new();
        io::stdin().read_line(&mut symbol).expect("Failed to read line");

        let len = symbol.len();
        symbol.truncate(len - 1);
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

1. **Crates**
    - yahoo_finance_api: Retrieving stock quotes from Yahoo Finance
    - time: Determining the date range to retrieve stock data
    - chrono: Converting between a date in milliseconds and a proper date format
    - plotters: Plotting the stock data
    - tokio: Runtime for writing asynchronous code

2. **Financial Analysis Algorithm**
   - If the percent change of a stock price on a given day is greater than 2%, it is considered volatile.
   - percent change = ((high-low)/close) * 100

3. **Charting Setup**
   - Line chart
   - X-axis: Date and time
   - Y-axis: Closing price; will be auto-scaled
   - Error bars used to highlight volatile days
   - The top and bottom caps of the error bars are the intraday high and low values respectively,
     and the circle is the closing value
    
4. **Project Setup**
   - main.rs is located in project_1/src/
   - Cargo.toml is located in project_1/
   - The stock chart will be placed in project_1/ after the program exits

5. **Usage instructions**
   - Run _cargo build_
   - Run _cargo run_
   - Enter a stock ticker when prompted
   - Chart will appear in the project_1 directory named _stock_ticker_.png 
   - Enter another stock ticker or "quit" to exit the program

1. **Crates**
    - yahoo_finance_api: Retrieving stock quotes from Yahoo Finance
    - plotters: Plotting the stock data
    - time: Converting between different time formats as required by different crates

2. **Financial Analysis Algorithm**

3. **Charting Setup**
   - Line chart
   - X-axis: Date and time
   - Y-axis: Closing price; will be auto-scaled
   - Error bars used to highlight volatile days
   - The top and bottom caps of the error bars are the intraday high and low values respectively,
     and the circle is the closing value
    
5. **Project Setup**
   - main.rs is located in project_1/src/
   - Cargo.toml is located in project_1/
   - The stock chart will be placed in project_1/ after the program exits

6. **Usage instructions**
   - Run _cargo build_
   - Run _cargo run_
   - Enter a stock ticker when prompted
   - Chart will appear in the project_1 directory named _stock_ticker_.png 
   - Enter another stock ticker or "quit" to exit the program

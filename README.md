<!-- force Git to detect README change -->
# NYC Yellow Taxi 2025 Data Analysis using Rust & DataFusion

## What the project does
- Analyzes NYC Yellow Taxi trip data for the full year 2025.
- Computes monthly trip statistics and total revenue.
- Examines tip behavior by payment type.
- Demonstrates Rust data processing with the DataFusion framework.

## Dataset source
- NYC TLC Trip Record Data: [https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page](https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page)

## How to download the data
- Download the Parquet files manually from the NYC TLC page (yellow taxi trips for 2025) and place them in the `data/` folder.
- **Do not commit the data folder to GitHub**; only keep it locally.

## How to run the project
1. Clone the repository:  
   ```bash
   git clone https://github.com/Eswar3605/nyc-taxi-datafusion.git

Navigate into the project folder:

cd nyc-taxi-datafusion

Build and run the project using Cargo:

cargo run

The program prints the aggregation summaries in the terminal.

Aggregations
1. Trips and revenue by month

Groups trips by pickup month.

Computes total trips, total revenue, and average fare per month.

2. Tip behavior by payment type

Groups trips by payment type.

Computes total trips, average tip amount, and tip rate.

## Output

Output is printed in the terminal.

Screenshot of output is included in the screenshot folder.
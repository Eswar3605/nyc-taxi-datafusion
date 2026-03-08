<!-- updated screenshot -->

# NYC Taxi Data Analysis using DataFusion

## Project Overview

- Loads NYC Yellow Taxi 2025 Parquet datasets
- Performs analytics using DataFusion DataFrame API
- Executes the same aggregations using DataFusion SQL
- Prints results directly to the terminal

## Dataset Source

NYC TLC Trip Record Data

https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page

## Download Data

Download Yellow Taxi 2025 parquet files and place them in:

data/

Example files:

yellow_tripdata_2025-01.parquet  
yellow_tripdata_2025-02.parquet  
...  
yellow_tripdata_2025-12.parquet

## Run the Project


cargo run


## Aggregations

### Trips and Revenue by Month

Shows monthly taxi activity including:

- Trip count
- Total revenue
- Average fare

### Tip Behavior by Payment Type

Analyzes tipping patterns by payment method including:

- Trip count
- Average tip amount
- Tip rate

## Output

Output is printed in the terminal.

Screenshot of output is included in the screenshot folder.
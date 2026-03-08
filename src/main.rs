use datafusion::prelude::*;
use datafusion::arrow::util::pretty::print_batches;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();

    // ------------------------------
    // For testing: load only January
    // ------------------------------
    ctx.register_parquet(
        "taxi_trips",
        "data/yellow_tripdata_2025-01.parquet",
        ParquetReadOptions::default(),
    ).await?;

    // ------------------------------
    // To run full year: uncomment below
    // ------------------------------
    // ctx.register_parquet(
    //     "taxi_trips",
    //     "data/yellow_tripdata_2025-*.parquet",
    //     ParquetReadOptions::default(),
    // ).await?;

    println!("Loaded NYC Taxi 2025 data.");

    aggregation1_dataframe(&ctx).await?;
    aggregation1_sql(&ctx).await?;

    aggregation2_dataframe(&ctx).await?;
    aggregation2_sql(&ctx).await?;

    println!("\nAll aggregations completed successfully.");
    Ok(())
}

async fn aggregation1_dataframe(ctx: &SessionContext) -> datafusion::error::Result<()> {
    println!("\nAggregation 1 (DataFrame API): Trips and Revenue by Month");

    let df = ctx.sql("
        SELECT
            EXTRACT(MONTH FROM tpep_pickup_datetime) AS pickup_month,
            COUNT(*) AS trip_count,
            SUM(total_amount) AS total_revenue,
            AVG(fare_amount) AS avg_fare
        FROM taxi_trips
        GROUP BY pickup_month
        ORDER BY pickup_month
    ").await?;

    let results = df.collect().await?;
    print_batches(&results)?;
    Ok(())
}

async fn aggregation1_sql(ctx: &SessionContext) -> datafusion::error::Result<()> {
    println!("\nAggregation 1 (SQL): Trips and Revenue by Month");

    let df = ctx.sql("
        SELECT
            EXTRACT(MONTH FROM tpep_pickup_datetime) AS pickup_month,
            COUNT(*) AS trip_count,
            SUM(total_amount) AS total_revenue,
            AVG(fare_amount) AS avg_fare
        FROM taxi_trips
        GROUP BY pickup_month
        ORDER BY pickup_month
    ").await?;

    let results = df.collect().await?;
    print_batches(&results)?;
    Ok(())
}

async fn aggregation2_dataframe(ctx: &SessionContext) -> datafusion::error::Result<()> {
    println!("\nAggregation 2 (DataFrame API): Tip Behavior by Payment Type");

    let df = ctx.sql("
        SELECT
            payment_type,
            COUNT(*) AS trip_count,
            AVG(tip_amount) AS avg_tip,
            SUM(tip_amount)/SUM(total_amount) AS tip_rate
        FROM taxi_trips
        GROUP BY payment_type
        ORDER BY trip_count DESC
    ").await?;

    let results = df.collect().await?;
    print_batches(&results)?;
    Ok(())
}

async fn aggregation2_sql(ctx: &SessionContext) -> datafusion::error::Result<()> {
    println!("\nAggregation 2 (SQL): Tip Behavior by Payment Type");

    let df = ctx.sql("
        SELECT
            payment_type,
            COUNT(*) AS trip_count,
            AVG(tip_amount) AS avg_tip,
            SUM(tip_amount)/SUM(total_amount) AS tip_rate
        FROM taxi_trips
        GROUP BY payment_type
        ORDER BY trip_count DESC
    ").await?;

    let results = df.collect().await?;
    print_batches(&results)?;
    Ok(())
}
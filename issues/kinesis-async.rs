I work in a financial services company, and I want to start using Rust and AWS Kinesis to build a pipeline that can process data from various stock exchanges. Help me build it. Be sure to ask me any question that you might need in order to make your response the best one possible.
Here are the tool that I want you to use:
- AWS SDK for Rust: Official SDK for interacting with AWS services.
- Tokio: For handling asynchronous tasks.
- serde: Parsing and processing JSON records

Assume that the stock exchange data is already being gathered and injected into Kinesis streams, which need to be consumed using the AWS SDK.
Save the results in a simple json from serde.

model A

The response in overall, provides a simple and practical implementation of a pipeline using Rust and AWS Kinses as requested. It breaks the implementation into separate understandable sections with clear headings. 

## Instruction following
- the response misses the oportunity to use rust's concurrency, while the tokio framework is used, it is not used in a way to improve low latency

## Accuracy
- the model failed to deliver using tokios concurrency, its processing data one record at a time

## Efficiency
- processing data one record at a time instead of leveraging batch processing is inefficient for large-scale pipelines
- file based storage introduces bottle necks
- there is no consideration of partition keys in kinesis, this could lead to inefficient stream processing

## Up-to-date
there are several crates with wrong version
- aws-sdk-kinesis = 1.51.0
- aws-config = "1.1.7"
- tokio = 1.41.1
- serde = "1.0.215"
- serde_json = "1.0.133"

model B

## Instruction following
- it was requested for the model to ask questions to better improve the response, this seems to have been skipped
- the response doesn't use concurrency to handle various tasks at a time

## Accuracy
- saving each record in a separate json file is not the way to go, this leads to a really high number of file I/O operations
- tokio was only used to create a new file, we are not using concurrency to handle various tasks at a time

## Efficiency 
The implementation is functional but inefficient
- Writing each record to a separate file slows down the program a lot
- Serializing and deserializing JSON for every record is unnecesary

## Up-to-date
there are several crates with wrong version
- aws-sdk-kinesis = 1.51.0
- aws-config = "1.1.7"
- tokio = 1.41.1
- serde = "1.0.215"
- serde_json = "1.0.133"
Also, the tokio::stream::StreamExt import is outdated for current versions of Tokio. Use futures::stream::StreamExt

tests: 
// Happy Path Test 1: Test with normal data
#[tokio::test]
async fn test_normal_data() {
    let data = StockData {
        symbol: "AAPL".to_string(),
        price: 150.50,
        timestamp: "2023-01-01T00:00:00Z".to_string(),
    };
    let processed_data = process_data(data).await;
    assert_eq!(processed_data["symbol"], "AAPL");
    assert_eq!(processed_data["price"], 150.50);
    assert_eq!(processed_data["timestamp"], "2023-01-01T00:00:00Z");
}

// Happy Path Test 2: Test with another normal data
#[tokio::test]
async fn test_another_normal_data() {
    let data = StockData {
        symbol: "GOOGL".to_string(),
        price: 2800.75,
        timestamp: "2023-01-01T00:00:00Z".to_string(),
    };
    let processed_data = process_data(data).await;
    assert_eq!(processed_data["symbol"], "GOOGL");
    assert_eq!(processed_data["price"], 2800.75);
    assert_eq!(processed_data["timestamp"], "2023-01-01T00:00:00Z");
}
// Edge Case Test 1: Test with empty data
#[tokio::test]
async fn test_empty_data() {
    let data = StockData {
        symbol: "".to_string(),
        price: 0.0,
        timestamp: "".to_string(),
    };
    let processed_data = process_data(data).await;
    assert_eq!(processed_data["symbol"], "");
    assert_eq!(processed_data["price"], 0.0);
    assert_eq!(processed_data["timestamp"], "");
}

// Edge Case Test 2: Test with very large price
#[tokio::test]
async fn test_large_price() {
    let data = StockData {
        symbol: "TEST".to_string(),
        price: f64::MAX,
        timestamp: "2023-01-01T00:00:00Z".to_string(),
    };
    let processed_data = process_data(data).await;
    assert_eq!(processed_data["symbol"], "TEST");
    assert_eq!(processed_data["price"], f64::MAX);
    assert_eq!(processed_data["timestamp"], "2023-01-01T00:00:00Z");
}

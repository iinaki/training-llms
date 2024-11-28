# Development of a Real-Time Environmental Monitoring System using Rust

## Use Case
A meteorological research institute uses this application to gather and analyze environmental data in real-time. The system uses remote sensing devices to regularly capture data such as temperature, humidity, and sunlight intensity. This data is sent back to a central server that processes this information immediately as it is posted. The application provides a comprehensive and immediate understanding of the environmental factors impacting the discipline of study, such as atmospheric science or ecology. Leveraging Rust's system-level capabilities, it ensures fast, concurrent, and safe data handling, essential for managing large volumes of real-time data.

```rust

use tokio::net::TcpListener;
use tokio::io::{BufReader, AsyncReadExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(HashMap::from([
        ("temperature", (0, 0)),
        ("humidity", (0, 0)),
        ("sunlight_intensity", (0, 0)),
    ])));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let data_clone = Arc::clone(&data);
        tokio::spawn(handle_stream(stream, data_clone));
    }
}

async fn handle_stream(mut stream: tokio::net::TcpStream, data: Arc<Mutex<HashMap<&str, (i32, i32)>>>) {
    let mut reader = BufReader::new(&mut stream);
    let mut buffer = [0; 12];

    loop {
        match reader.read_exact(&mut buffer).await {
            Ok(_) => {
                let temperature = i32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let humidity = i32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
                let sunlight_intensity = i32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);

                update_data(data.clone(), "temperature", temperature).await;
                update_data(data.clone(), "humidity", humidity).await;
                update_data(data.clone(), "sunlight_intensity", sunlight_intensity).await;

                println!("Received data: temperature={}, humidity={}, sunlight_intensity={}", temperature, humidity, sunlight_intensity);
                println!("Averages: {:?}", *data.lock().unwrap());
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}

async fn update_data(data: Arc<Mutex<HashMap<&str, (i32, i32)>>>, key: &str, value: i32) {
    let mut data_map = data.lock().unwrap();
    let (sum, count) = data_map.get(key).unwrap();
    data_map.insert(key, (sum + value, count + 1));
}

```

## tests


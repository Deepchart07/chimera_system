use anyhow::Result;
use kdbplus::ipc::*;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    
    let mut qstream = QStream::connect(ConnectionMethod::TCP, "127.0.0.1", 5000, "").await?;

    let url = "wss://stream.binance.com:9443/ws/btcusdt@trade";
    let (mut ws_stream, _) = connect_async(url).await?;

    println!("Project Chimera: Enlace HFT establecido. Escuchando Binance...");

    while let Some(msg) = ws_stream.next().await {
        if let Ok(Message::Text(text)) = msg {
            let v: Value = serde_json::from_str(&text)?;
            
            let sym = v["s"].as_str().unwrap_or("BTCUSDT");
            let price: f64 = v["p"].as_str().unwrap_or("0").parse()?;
            let size: f64 = v["q"].as_str().unwrap_or("0").parse()?;

            let query = format!("`trades insert (.z.n; `{sym}; {price}; {size})");
            
            
            if let Err(e) = qstream.send_async_message(&K::new_string(query, 0)).await {
                eprintln!("IPC Error: {}", e);
            }
        }
    }

    Ok(())
}
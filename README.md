# Synapse 🧠

**Synapse** is a standardized metric system designed to evaluate real-world network quality beyond simple speed tests. It introduces three key metrics:

- 🌀 **Vortex** (Performance): Measures flow efficiency based on speed, latency, jitter, and packet loss.
- 📡 **Radiance** (Signal): Measures the physical quality of the wireless environment (SNR & Channel Width).
- 🏆 **Axon** (Health): The unified score representing the real-world stability of the connection.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
synapse = "0.1.0"
# Optional: Enable Serde support
# synapse = { version = "0.1.0", features = ["serde"] }
```

## Usage
```rust
use synapse::NetworkData;

fn main() {
    let data = NetworkData {
        down_mbps: Some(150.0),
        up_mbps: Some(40.0),
        ping_ms: Some(18.0),
        jitter_ms: Some(2.0),
        packet_loss_percent: Some(0.0),
        rssi_dbm: Some(-60.0),
        noise_dbm: Some(-90.0),
        channel_width_mhz: Some(40.0),
    };

    if let Some(score) = data.calculate_axon() {
        println!("Network Health Score: {:.2}", score);
    }
}
```

## License
MIT License.
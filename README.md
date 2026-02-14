<h1 align="center">Synapse 🧠</h1>
<div align="center">
    <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/synapse-rs">
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/synapse-rs">
    <img alt="Crates.io License" src="https://img.shields.io/crates/l/synapse-rs">
</div>

**Synapse** is a standardized metric system designed to evaluate real-world network quality beyond simple speed tests. It introduces three key metrics:

- 🌀 **Vortex** (Performance): Measures flow efficiency based on speed, latency, jitter, and packet loss.
- 📡 **Radiance** (Signal): Measures the physical quality of the wireless environment (SNR & Channel Width).
- 🏆 **Axon** (Health): The unified score representing the real-world stability of the connection.

## Installation 📦

Add this to your `Cargo.toml`:

```toml
[dependencies]
synapse-rs = "1.0.1"
# Optional: Enable Serde support
# synapse = { version = "1.0.1", features = ["serde"] }
```

## Example 💡
```rust
use synapse_rs::NetworkData;

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

## Contributing 🤝
1. Fork the original repository by clicking the “Fork” button at the top right of the repository page.

2. Clone your fork locally: `git clone https://github.com/FlanZCode/synapse.git`

3. Create a new branch for your changes: `git checkout -b my-new-feature`

4. Make your changes in the code.

5. Commit your changes with a clear message: `git commit -m "Description of my changes"`

6. Push your changes to your fork: `git push origin my-new-feature`

7. Go to your fork’s page on GitHub and click the “New pull request” button.

8. Describe your changes and submit the pull request.

## Credits 🙏
<a href="https://github.com/FlanZCode/synapse/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=FlanZCode/synapse" />
</a>

## License 📕
This project is licensed under the MIT License. See the LICENSE file for details.
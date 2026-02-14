#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents raw network connection data.
///
/// This struct serves as the entry point for Synapse metric calculations.
/// All fields are optional to handle partial cases (e.g., Ethernet without Wi-Fi).
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NetworkData {
    /// Download speed in Mbps.
    pub down_mbps: Option<f64>,
    /// Upload speed in Mbps.
    pub up_mbps: Option<f64>,
    /// Latency (Ping) in milliseconds.
    pub ping_ms: Option<f64>,
    /// Jitter (Ping variation) in milliseconds.
    pub jitter_ms: Option<f64>,
    /// Packet loss percentage (e.g., 1.0 for 1%).
    pub packet_loss_percent: Option<f64>,
    /// Received Signal Strength Indicator in dBm (e.g., -65.0). Wireless only.
    pub rssi_dbm: Option<f64>,
    /// Noise floor in dBm (e.g., -90.0). Wireless only.
    pub noise_dbm: Option<f64>,
    /// Wi-Fi channel width in MHz (e.g., 20, 40, 80, 160).
    pub channel_width_mhz: Option<f64>,
}

impl NetworkData {
    /// Small value to avoid division by zero in friction calculation.
    const EPSILON: f64 = 1e-7;

    /// Calculates the **VORTEX** score (Pure flow performance).
    ///
    /// The Vortex measures pipe efficiency. It combines volume (speeds)
    /// and fluidity (latency/jitter), penalized by integrity (packet loss).
    ///
    /// # Returns
    /// * `Some(score)`: If all performance data is present.
    /// * `None`: If data is missing (ping, speed, etc.).
    pub fn calculate_vortex(&self) -> Option<f64> {
        let down = self.down_mbps?;
        let up = self.up_mbps?;
        let ping = self.ping_ms?;
        let jitter = self.jitter_ms?;
        let lost = self.packet_loss_percent?;

        let down_score = (1.0 + down).log10();
        let up_score = (1.0 + up).log10();
        let volume = down_score * up_score;

        let ping_seconds = ping / 1000.0;
        let jitter_seconds = jitter / 1000.0;
        let friction = ping_seconds + (3.0 * jitter_seconds) + Self::EPSILON;

        let integrity = (1.0 - (lost / 100.0))
            .clamp(0.0, 1.0)
            .powf(10.0);

        Some((volume / friction) * integrity)
    }

    /// Calculates the **RADIANCE** score (Physical signal quality).
    ///
    /// Radiance measures the cleanliness of the radio environment.
    /// Based on SNR (Signal-to-Noise Ratio) and channel width.
    ///
    /// # Returns
    /// * `Some(score)`: If Wi-Fi data is present.
    /// * `None`: If wired (Ethernet) or read error.
    pub fn calculate_radiance(&self) -> Option<f64> {
        let width = self.channel_width_mhz?;
        let rssi = self.rssi_dbm?;
        let noise = self.noise_dbm?;

        let width_factor = width / 20.0;
        let snr = rssi - noise;

        Some((width_factor * snr).max(0.0))
    }

    /// Calculates the final **AXON** score (Unified Metric).
    ///
    /// Axon is the geometric mean of Vortex (Perf) and Radiance (Phys).
    /// It represents the "Real Health" of the connection.
    pub fn calculate_axon(&self) -> Option<f64> {
        let vx = self.calculate_vortex()?;
        let rd = self.calculate_radiance()?;

        Some((vx * rd).sqrt())
    }
}
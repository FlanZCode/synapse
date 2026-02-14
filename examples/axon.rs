use synapse::NetworkData;

fn main() {
    let measure = NetworkData {
        down_mbps: Some(45.0),
        up_mbps: Some(12.0),
        ping_ms: Some(35.0),
        jitter_ms: Some(4.0),
        packet_loss_percent: Some(0.1),
        rssi_dbm: Some(-80.0),
        noise_dbm: Some(-100.0),
        channel_width_mhz: Some(20.0),
        ..Default::default()
    };

    match measure.calculate_vortex() {
        Some(v) => println!("Vortex   : {:.2}", v),
        None => println!("Vortex   : N/A (Insufficient data)"),
    }

    match measure.calculate_radiance() {
        Some(r) => println!("Radiance : {:.2}", r),
        None => println!("Radiance : N/A (Wired Mode or Error)"),
    }

    println!("--------------------");

    match measure.calculate_axon() {
        Some(a) => println!("AXON     : {:.2}", a),
        None => println!("AXON     : Not calculable"),
    }
    #[cfg(feature = "serde")]
    {
        let json = serde_json::to_string_pretty(&measure).unwrap();
        println!("\nStats (JSON):\n{}", json);
    }
}
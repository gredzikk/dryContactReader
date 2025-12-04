// Add to Cargo.toml: serialport = "4.8.1"
use serialport::{SerialPort, DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut port = serialport::new("COM3", 9600)
        .timeout(Duration::from_millis(10))
        .open()?;

    port.write_request_to_send(true)?;
    port.write_data_terminal_ready(true)?;

    println!("Reading relays (Ctrl+C to exit):\n");

    loop {
        let relay1 = if port.read_clear_to_send()? { "CLOSED" } else { "OPEN  " };
        let relay2 = if port.read_data_set_ready()? { "CLOSED" } else { "OPEN  " };

        print!("\rRelay1: {} | Relay2: {}", relay1, relay2);
        io::stdout().flush()?;

        std::thread::sleep(Duration::from_millis(100));
    }
}

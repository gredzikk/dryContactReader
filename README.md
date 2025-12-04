# Dry Contact Reader Examples

⚠️ **DANGER - READ THIS FIRST** ⚠️

**WARNING: This code is provided as-is for educational purposes only. It is a quick, dirty, and potentially dangerous implementation.**

## CRITICAL SAFETY NOTICES:

### ⚠️ DRY CONTACTS ONLY ⚠️
**ONLY CONNECT DRY RELAY CONTACTS TO YOUR SERIAL PORT!**

- ❌ **NEVER** connect any external voltage source to serial port pins
- ❌ **NEVER** connect mains voltage (AC power) to these circuits
- ❌ **NEVER** connect powered circuits or active signals
- ✅ **ONLY** connect isolated, unpowered relay contacts (dry contacts)
- ✅ Use relays with NO external power on the contact side

### ⚠️ DISCLAIMER ⚠️
**THE AUTHOR TAKES NO RESPONSIBILITY FOR:**
- Damaged computers, serial ports, or USB adapters
- Destroyed devices or equipment
- Data loss or system failures
- Personal injury or loss of life
- Any other damages, losses, or consequences

**USE AT YOUR OWN RISK!** You are solely responsible for ensuring safe connections and proper isolation.

### What is a "Dry Contact"?
A dry contact is a relay contact with **NO voltage or power** applied to it. It's simply a mechanical switch that opens and closes. The serial port provides the low voltage (~5-12V) and the relay contact completes the circuit.

---

Examples for reading relay dry contact states using serial port control lines (RTS-CTS and DTR-DSR pairs).

## Hardware Connections
- **Relay 1**: RTS (pin 7) → Dry Contact → CTS (pin 8)
- **Relay 2**: DTR (pin 4) → Dry Contact → DSR (pin 6)

**Note:** Ground (pin 5) is NOT used - each signal pair is self-contained.

## Examples Provided

### C# (`sample.cs`)
```bash
csc sample.cs
sample.exe
```

### Python (`sample.py`)
```bash
pip install pyserial
python sample.py
```

### Rust (`sample.rs`)
```bash
# Add to Cargo.toml: serialport = "4.8.1"
cargo run
```

## Usage Notes

- Change port name (`COM3`, `/dev/ttyUSB0`, etc.) to match your system
- Contact closure is detected when CTS/DSR goes high

## License

This project is licensed under the MIT License with additional safety disclaimers - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This is an educational project demonstrating serial port control line usage
- Always verify your hardware setup before connecting anything
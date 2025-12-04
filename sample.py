#!/usr/bin/env python3
#requires: pip install pyserial

import serial
import time

port = serial.Serial('COM3', 9600, timeout=1)

port.rts = True
port.dtr = True

print("Reading relays (Ctrl+C to exit):\n")

try:
    while True:
        relay1 = "CLOSED" if port.cts else "OPEN  "
        relay2 = "CLOSED" if port.dsr else "OPEN  "
        
        print(f"\rRelay1: {relay1} | Relay2: {relay2}", end='', flush=True)
        time.sleep(0.1)
        
except KeyboardInterrupt:
    print("\n\nExiting...")
finally:
    port.close()

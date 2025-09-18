# Forlenza Industrial Control System v2.1

**Educational Mock Software for Demonstrating Virtualization Benefits**

## Overview

The Forlenza Industrial Control System is a mock industrial control application designed specifically for educational purposes. It demonstrates how legacy software becomes incompatible with newer operating systems and showcases the critical role of virtualization in preserving access to older applications.

> ⚠️ **Educational Purpose Only**: This is simulation software created for teaching virtualization concepts. It does not control real industrial equipment.

## Installation

### Method 1: Pre-compiled Binary
1. Download the latest release from the releases page
2. Extract the ZIP file to your desired location
3. Run `forlenza_industrial.exe`

### Method 2: Build from Source
1. Install [Rust](https://rustup.rs/) (version 1.70 or later)
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/forlenza-industrial-control
   cd forlenza-industrial-control
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Run the executable:
   ```bash
   target/release/forlenza_industrial.exe
   ```

### Features

#### On Incompatible Systems (Windows 8+)
- Clear error messages explaining incompatibility
- Lists specific technical reasons for failure
- Suggests virtualization as a solution

#### On Compatible Systems (Windows 7)
- Real-time sensor monitoring (temperature, pressure, motor speeds)
- Interactive diagnostic system
- Emergency shutdown capabilities
- Professional industrial interface

#### Simulated Industrial Features
- **Temperature Sensors**: 4 sensors with realistic fluctuations
- **Pressure Gauges**: 3 gauges monitoring system pressure
- **Motor Control**: 4 motors with variable speeds and states
- **Safety Systems**: Emergency shutdown and interlock monitoring


## License

This educational software is released under the MIT License. See LICENSE file for details.

## Disclaimer

**Important**: This software is designed for educational purposes only. It simulates industrial control systems but does not connect to or control real equipment. Do not use in production environments.

---

*Created for educational demonstration of virtualization concepts and legacy software preservation.*
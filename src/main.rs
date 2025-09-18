use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;

// Windows API declarations for legacy functions
#[link(name = "kernel32")]
extern "system" {
    fn GetVersion() -> u32;
    fn GetVersionExW(lpVersionInfo: *mut OSVERSIONINFOEXW) -> i32;
}

#[repr(C)]
struct OSVERSIONINFOEXW {
    dwOSVersionInfoSize: u32,
    dwMajorVersion: u32,
    dwMinorVersion: u32,
    dwBuildNumber: u32,
    dwPlatformId: u32,
    szCSDVersion: [u16; 128],
    wServicePackMajor: u16,
    wServicePackMinor: u16,
    wSuiteMask: u16,
    wProductType: u8,
    wReserved: u8,
}

struct IndustrialController {
    system_id: String,
    temperature_sensors: Vec<f32>,
    pressure_gauges: Vec<f32>,
    motor_speeds: Vec<u16>,
    safety_interlocks: bool,
}

impl IndustrialController {
    fn new() -> Result<Self, String> {
        // Check for Windows 7 specifically
        if !Self::is_windows_7() {
            return Err("Forlenza Industrial Control System requires Windows 7 Professional or Ultimate.\nThis software uses legacy Windows APIs that are not supported on newer operating systems.".to_string());
        }

        println!("Forlenza Industrial Control System v2.1");
        println!("Initializing legacy hardware interfaces...");
        
        Ok(IndustrialController {
            system_id: "HIS-CTRL-7001".to_string(),
            temperature_sensors: vec![23.5, 24.1, 22.8, 25.0],
            pressure_gauges: vec![101.3, 98.7, 102.1],
            motor_speeds: vec![1750, 1800, 0, 2200],
            safety_interlocks: true,
        })
    }

    fn is_windows_7() -> bool {
        unsafe {
            let mut version_info = OSVERSIONINFOEXW {
                dwOSVersionInfoSize: std::mem::size_of::<OSVERSIONINFOEXW>() as u32,
                dwMajorVersion: 0,
                dwMinorVersion: 0,
                dwBuildNumber: 0,
                dwPlatformId: 0,
                szCSDVersion: [0; 128],
                wServicePackMajor: 0,
                wServicePackMinor: 0,
                wSuiteMask: 0,
                wProductType: 0,
                wReserved: 0,
            };

            let result = GetVersionExW(&mut version_info);
            
            if result != 0 {
                // Windows 7 is version 6.1
                version_info.dwMajorVersion == 6 && version_info.dwMinorVersion == 1
            } else {
                // Fallback to deprecated GetVersion (removed in Windows 8.1+)
                let version = GetVersion();
                let major = version & 0xFF;
                let minor = (version >> 8) & 0xFF;
                major == 6 && minor == 1
            }
        }
    }

    fn run_diagnostic(&self) -> Result<(), String> {
        println!("\n=== Forlenza INDUSTRIAL DIAGNOSTIC ===");
        println!("System ID: {}", self.system_id);
        
        // Simulate legacy hardware communication
        println!("Communicating with legacy PLC interfaces...");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        
        println!("Temperature Sensors:");
        for (i, temp) in self.temperature_sensors.iter().enumerate() {
            println!("  Sensor {}: {:.1}°C", i + 1, temp);
        }
        
        println!("Pressure Gauges:");
        for (i, pressure) in self.pressure_gauges.iter().enumerate() {
            println!("  Gauge {}: {:.1} kPa", i + 1, pressure);
        }
        
        println!("Motor Status:");
        for (i, speed) in self.motor_speeds.iter().enumerate() {
            let status = if *speed == 0 { "STOPPED" } else { "RUNNING" };
            println!("  Motor {}: {} ({} RPM)", i + 1, status, speed);
        }
        
        println!("Safety Interlocks: {}", if self.safety_interlocks { "ACTIVE" } else { "BYPASSED" });
        
        // Simulate Windows 7-specific registry access
        self.check_legacy_drivers()?;
        
        println!("\nDiagnostic Complete - All Systems Operational");
        Ok(())
    }
    
    fn check_legacy_drivers(&self) -> Result<(), String> {
        println!("\nChecking legacy device drivers...");
        std::thread::sleep(std::time::Duration::from_millis(800));
        
        // Simulate checking for Windows 7-era drivers
        let drivers = vec![
            "Forlenza Serial Interface v3.2",
            "Industrial Ethernet Adapter v2.1", 
            "Legacy PLC Communication Driver v1.8",
            "Safety Interlock Monitor v4.0"
        ];
        
        for driver in drivers {
            println!("  ✓ {}", driver);
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
        
        Ok(())
    }

    fn emergency_shutdown(&mut self) {
        println!("\n!!! EMERGENCY SHUTDOWN INITIATED !!!");
        self.motor_speeds = vec![0, 0, 0, 0];
        self.safety_interlocks = true;
        println!("All motors stopped. Safety systems engaged.");
    }
}

fn main() {
    println!("Forlenza Industrial Control System");
    println!("Checking system compatibility...\n");
    
    match IndustrialController::new() {
        Ok(mut controller) => {
            println!("System compatibility verified!\n");
            
            if let Err(e) = controller.run_diagnostic() {
                eprintln!("Diagnostic error: {}", e);
                return;
            }
            
            println!("\nPress Enter to start emergency shutdown...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            
            controller.emergency_shutdown();
        },
        Err(e) => {
            eprintln!("COMPATIBILITY ERROR:");
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
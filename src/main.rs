use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Windows API declarations for legacy functions
#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
extern "system" {
    fn GetVersion() -> u32;
    fn GetVersionExW(lpVersionInfo: *mut OSVERSIONINFOEXW) -> i32;
}

#[cfg(target_os = "windows")]
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

#[derive(Clone)]
struct SensorData {
    temperatures: Vec<f32>,
    pressures: Vec<f32>,
    motor_speeds: Vec<u16>,
    motor_states: Vec<bool>,
    safety_interlocks: bool,
    last_update: Instant,
}

impl Default for SensorData {
    fn default() -> Self {
        Self {
            temperatures: vec![23.5, 24.1, 22.8, 25.0],
            pressures: vec![101.3, 98.7, 102.1],
            motor_speeds: vec![1750, 1800, 0, 2200],
            motor_states: vec![true, true, false, true],
            safety_interlocks: true,
            last_update: Instant::now(),
        }
    }
}

struct ForlenzaControlApp {
    system_compatible: bool,
    error_message: String,
    sensor_data: Arc<Mutex<SensorData>>,
    diagnostic_running: bool,
    diagnostic_log: Vec<String>,
    emergency_shutdown: bool,
    connection_status: String,
}

impl Default for ForlenzaControlApp {
    fn default() -> Self {
        let (compatible, error) = Self::check_windows_7_compatibility();
        
        let app = Self {
            system_compatible: compatible,
            error_message: error,
            sensor_data: Arc::new(Mutex::new(SensorData::default())),
            diagnostic_running: false,
            diagnostic_log: Vec::new(),
            emergency_shutdown: false,
            connection_status: if compatible { "Connected to Legacy PLCs".to_string() } else { "System Incompatible".to_string() },
        };
        
        if compatible {
            app.start_sensor_simulation();
        }
        
        app
    }
}

impl ForlenzaControlApp {
    #[cfg(target_os = "windows")]
    fn check_windows_7_compatibility() -> (bool, String) {
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
            
            let is_win7 = if result != 0 {
                // Windows 7 is version 6.1
                version_info.dwMajorVersion == 6 && version_info.dwMinorVersion == 1
            } else {
                // Fallback to deprecated GetVersion (removed in Windows 8.1+)
                let version = GetVersion();
                let major = version & 0xFF;
                let minor = (version >> 8) & 0xFF;
                major == 6 && minor == 1
            };
            
            if is_win7 {
                (true, String::new())
            } else {
                (false, "CRITICAL COMPATIBILITY ERROR:\n\nForlenza Industrial Control System requires Windows 7 Professional or Ultimate.\n\nThis software depends on:\n• Legacy Windows APIs removed in Windows 8+\n• DirectX 9.0c for industrial HMI rendering\n• Windows 7 driver model for PLC communication\n• Registry structures changed in newer Windows versions\n\nTo run this software:\n1. Use a Windows 7 virtual machine\n2. Install legacy industrial drivers\n3. Configure compatibility mode (may not work)\n\nContact Forlenza Industrial Systems for upgrade options.".to_string())
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    fn check_windows_7_compatibility() -> (bool, String) {
        (false, "PLATFORM ERROR:\n\nForlenza Industrial Control System is designed exclusively for Windows 7.\n\nThis software requires:\n• Windows 7 Professional/Ultimate\n• Legacy Windows APIs\n• Industrial hardware drivers\n\nPlease run on a Windows 7 system or virtual machine.".to_string())
    }
    
    fn start_sensor_simulation(&self) {
        let sensor_data = Arc::clone(&self.sensor_data);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(1000));
                
                if let Ok(mut data) = sensor_data.lock() {
                    // Simulate temperature fluctuations
                    for temp in &mut data.temperatures {
                        *temp += (rand::random::<f32>() - 0.5) * 0.2;
                        *temp = temp.clamp(20.0, 30.0);
                    }
                    
                    // Simulate pressure changes
                    for pressure in &mut data.pressures {
                        *pressure += (rand::random::<f32>() - 0.5) * 0.5;
                        *pressure = pressure.clamp(95.0, 105.0);
                    }
                    
                    // Update motor speeds for running motors
                    for (i, (speed, running)) in data.motor_speeds.iter_mut().zip(&data.motor_states).enumerate() {
                        if *running {
                            *speed = (1750 + i * 50 + (rand::random::<u16>() % 100)) as u16;
                        }
                    }
                    
                    data.last_update = Instant::now();
                }
            }
        });
    }
    
    fn run_diagnostic(&mut self) {
        if !self.system_compatible || self.diagnostic_running {
            return;
        }
        
        self.diagnostic_running = true;
        self.diagnostic_log.clear();
        self.diagnostic_log.push("=== FORLENZA INDUSTRIAL DIAGNOSTIC ===".to_string());
        self.diagnostic_log.push("System ID: FIS-CTRL-7001".to_string());
        self.diagnostic_log.push("Initializing legacy hardware interfaces...".to_string());
        self.diagnostic_log.push("Checking Windows 7 compatibility... ✓".to_string());
        self.diagnostic_log.push("Loading legacy PLC drivers... ✓".to_string());
        self.diagnostic_log.push("Connecting to industrial network... ✓".to_string());
        self.diagnostic_log.push("Verifying safety interlocks... ✓".to_string());
        self.diagnostic_log.push("Diagnostic Complete - All Systems Operational".to_string());
        
        // Reset diagnostic flag after a delay
        let sensor_data = Arc::clone(&self.sensor_data);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(3000));
            // In a real app, you'd use a channel to communicate back to the main thread
        });
    }
    
    fn emergency_shutdown(&mut self) {
        if let Ok(mut data) = self.sensor_data.lock() {
            for (speed, state) in data.motor_speeds.iter_mut().zip(data.motor_states.iter_mut()) {
                *speed = 0;
                *state = false;
            }
            data.safety_interlocks = true;
        }
        self.emergency_shutdown = true;
        self.connection_status = "EMERGENCY SHUTDOWN ACTIVE".to_string();
    }
}

impl eframe::App for ForlenzaControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaints for real-time updates
        ctx.request_repaint();
        
        if !self.system_compatible {
            // Show compatibility error window
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    
                    ui.heading("Forlenza Industrial Control System v2.1");
                    ui.add_space(20.0);
                    
                    // Show error in a red frame
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(40, 0, 0))
                        .stroke(egui::Stroke::new(2.0, egui::Color32::RED))
                        .inner_margin(egui::style::Margin::same(20.0))
                        .show(ui, |ui| {
                            ui.colored_label(egui::Color32::RED, "⚠ SYSTEM INCOMPATIBLE ⚠");
                            ui.add_space(10.0);
                            ui.label(&self.error_message);
                        });
                    
                    ui.add_space(30.0);
                    ui.label("This demonstrates how legacy industrial software");
                    ui.label("becomes unusable on newer operating systems.");
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::YELLOW, "💡 Solution: Use Windows 7 Virtual Machine");
                });
            });
            return;
        }
        
        // Main application interface
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Forlenza Industrial Control System v2.1");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(
                        if self.emergency_shutdown { egui::Color32::RED } else { egui::Color32::GREEN },
                        &self.connection_status
                    );
                });
            });
        });
        
        egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Run Diagnostic").clicked() {
                    self.run_diagnostic();
                }
                
                ui.separator();
                
                if ui.button("🛑 Emergency Shutdown").clicked() {
                    self.emergency_shutdown();
                }
                
                if ui.button("Reset System").clicked() && self.emergency_shutdown {
                    if let Ok(mut data) = self.sensor_data.lock() {
                        data.motor_states = vec![true, true, false, true];
                        for (i, (speed, state)) in data.motor_speeds.iter_mut().zip(&data.motor_states).enumerate() {
                            if *state {
                                *speed = 1750 + (i * 50) as u16;
                            }
                        }
                    }
                    self.emergency_shutdown = false;
                    self.connection_status = "Connected to Legacy PLCs".to_string();
                }
            });
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                // Left column - Sensor data
                columns[0].group(|ui| {
                    ui.heading("Sensor Monitoring");
                    
                    if let Ok(data) = self.sensor_data.lock() {
                        ui.separator();
                        ui.label("Temperature Sensors:");
                        for (i, temp) in data.temperatures.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("Sensor {}: ", i + 1));
                                ui.colored_label(
                                    if *temp > 26.0 { egui::Color32::RED } else { egui::Color32::GREEN },
                                    format!("{:.1}°C", temp)
                                );
                            });
                        }
                        
                        ui.add_space(10.0);
                        ui.label("Pressure Gauges:");
                        for (i, pressure) in data.pressures.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("Gauge {}: ", i + 1));
                                ui.colored_label(
                                    if *pressure < 98.0 || *pressure > 103.0 { egui::Color32::YELLOW } else { egui::Color32::GREEN },
                                    format!("{:.1} kPa", pressure)
                                );
                            });
                        }
                        
                        ui.add_space(10.0);
                        ui.label("Motor Status:");
                        for (i, (speed, running)) in data.motor_speeds.iter().zip(&data.motor_states).enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("Motor {}: ", i + 1));
                                if *running {
                                    ui.colored_label(egui::Color32::GREEN, format!("RUNNING ({} RPM)", speed));
                                } else {
                                    ui.colored_label(egui::Color32::RED, "STOPPED");
                                }
                            });
                        }
                        
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            ui.label("Safety Interlocks: ");
                            ui.colored_label(
                                if data.safety_interlocks { egui::Color32::GREEN } else { egui::Color32::RED },
                                if data.safety_interlocks { "ACTIVE" } else { "BYPASSED" }
                            );
                        });
                    }
                });
                
                // Right column - Diagnostic log
                columns[1].group(|ui| {
                    ui.heading("System Diagnostic");
                    
                    if self.diagnostic_running {
                        ui.colored_label(egui::Color32::YELLOW, "Running diagnostic...");
                    }
                    
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            for log_entry in &self.diagnostic_log {
                                ui.label(log_entry);
                            }
                        });
                        
                    if self.diagnostic_log.is_empty() {
                        ui.label("Click 'Run Diagnostic' to test system components");
                        ui.add_space(10.0);
                        ui.label("Legacy Components:");
                        ui.label("• Windows 7 API compatibility layer");
                        ui.label("• DirectX 9.0c industrial rendering");
                        ui.label("• Legacy PLC communication drivers");
                        ui.label("• Industrial Ethernet protocols");
                        ui.label("• Safety interlock monitoring");
                    }
                });
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Forlenza Industrial Control System")
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Forlenza Industrial Control System",
        options,
        Box::new(|_cc|         Box::<ForlenzaControlApp>::default()),
    )
}

// Simple random number generation for demo
mod rand {
    use std::sync::atomic::{AtomicU64, Ordering};
    
    static SEED: AtomicU64 = AtomicU64::new(1);
    
    pub fn random<T>() -> T 
    where 
        T: From<u64>
    {
        let prev = SEED.load(Ordering::Relaxed);
        let next = prev.wrapping_mul(1103515245).wrapping_add(12345);
        SEED.store(next, Ordering::Relaxed);
        T::from(next)
    }
}
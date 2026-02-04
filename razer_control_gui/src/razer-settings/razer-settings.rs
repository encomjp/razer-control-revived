use gtk4 as gtk;
use libadwaita as adw;
use gtk::prelude::*;
use adw::prelude::*;
use std::io::ErrorKind;
use std::fs;
use std::time::Duration;

#[path = "../comms.rs"]
mod comms;
mod error_handling;
mod widgets;
mod util;

use service::SupportedDevice;
use error_handling::*;
use widgets::*;
use util::*;

fn send_data(opt: comms::DaemonCommand) -> Option<comms::DaemonResponse> {
    match comms::try_bind() {
        Ok(socket) => comms::send_to_daemon(opt, socket),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            crash_with_msg("Can't connect to the daemon");
        }
        Err(error) => {
            println!("Error opening socket: {error}");
            None
        }
    }
}

// ... Keep helper functions (get_device_name, get_bho, etc) as they are logic, not UI ...
// (Re-pasting them here for completeness)

fn get_device_name() -> Option<String> {
    let response = send_data(comms::DaemonCommand::GetDeviceName)?;
    use comms::DaemonResponse::*;
    match response {
        GetDeviceName { name } => Some(name),
        response => {
            println!("Instead of GetDeviceName got {response:?}");
            None
        }
    }
}

fn get_bho() -> Option<(bool, u8)> {
    let response = send_data(comms::DaemonCommand::GetBatteryHealthOptimizer())?;
    use comms::DaemonResponse::*;
    match response {
        GetBatteryHealthOptimizer { is_on, threshold } => Some((is_on, threshold)),
        response => {
            println!("Instead of GetBatteryHealthOptimizer got {response:?}");
            None
        }
    }
}

fn set_bho(is_on: bool, threshold: u8) -> Option<bool> {
    let response = send_data(comms::DaemonCommand::SetBatteryHealthOptimizer { is_on, threshold })?;
    use comms::DaemonResponse::*;
    match response {
        SetBatteryHealthOptimizer { result } => Some(result),
        response => {
            println!("Instead of SetBatteryHealthOptimizer got {response:?}");
            None
        }
    }
}

fn get_brightness(ac: bool) -> Option<u8> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::GetBrightness{ ac })?;
    use comms::DaemonResponse::*;
    match response {
        GetBrightness { result } => Some(result),
        response => {
            println!("Instead of GetBrightness got {response:?}");
            None
        }
    }
}

fn set_brightness(ac: bool, val: u8) -> Option<bool> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::SetBrightness { ac, val })?;
    use comms::DaemonResponse::*;
    match response {
        SetBrightness { result } => Some(result),
        response => {
            println!("Instead of SetBrightness got {response:?}");
            None
        }
    }
}

fn get_logo(ac: bool) -> Option<u8> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::GetLogoLedState{ ac })?;
    use comms::DaemonResponse::*;
    match response {
        GetLogoLedState { logo_state } => Some(logo_state),
        response => {
            println!("Instead of GetLogoLedState got {response:?}");
            None
        }
    }
}

fn set_logo(ac: bool, logo_state: u8) -> Option<bool> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::SetLogoLedState{ ac , logo_state })?;
    use comms::DaemonResponse::*;
    match response {
        SetLogoLedState { result } => Some(result),
        response => {
            println!("Instead of SetLogoLedState got {response:?}");
            None
        }
    }
}

fn set_effect(name: &str, values: Vec<u8>) -> Option<bool> {
    let response = send_data(comms::DaemonCommand::SetEffect { name: name.into(), params: values })?;
    use comms::DaemonResponse::*;
    match response {
        SetEffect { result } => Some(result),
        response => {
            println!("Instead of SetEffect got {response:?}");
            None
        }
    }
}

fn get_power(ac: bool) -> Option<(u8, u8, u8)> {
    let ac = if ac { 1 } else { 0 };
    let mut result = (0, 0, 0);

    let response = send_data(comms::DaemonCommand::GetPwrLevel{ ac })?;
    use comms::DaemonResponse::*;
    match response {
        GetPwrLevel { pwr } => result.0 = pwr,
        response => {
            println!("Instead of GetPwrLevel got {response:?}");
            return None
        }
    }

    let response = send_data(comms::DaemonCommand::GetCPUBoost { ac })?;
    use comms::DaemonResponse::*;
    match response {
        GetCPUBoost { cpu } => result.1 = cpu,
        response => {
            println!("Instead of GetCPUBoost got {response:?}");
            return None
        }
    }

    let response = send_data(comms::DaemonCommand::GetGPUBoost { ac })?;
    use comms::DaemonResponse::*;
    match response {
        GetGPUBoost { gpu } => result.2 = gpu,
        response => {
            println!("Instead of GetGPUBoost got {response:?}");
            return None
        }
    }
    Some(result)
}

fn set_power(ac: bool, power: (u8, u8, u8)) -> Option<bool> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::SetPowerMode { ac, pwr: power.0, cpu: power.1, gpu: power.2 })?;
    use comms::DaemonResponse::*;
    match response {
        SetPowerMode { result } => Some(result),
        response => {
            println!("Instead of SetPowerMode got {response:?}");
            None
        }
    }
}

fn get_fan_speed(ac: bool) -> Option<i32> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::GetFanSpeed{ ac })?;
    use comms::DaemonResponse::*;
    match response {
        GetFanSpeed { rpm } => Some(rpm),
        response => {
            println!("Instead of GetFanSpeed got {response:?}");
            None
        }
    }
}

fn set_fan_speed(ac: bool, value: i32) -> Option<bool> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::SetFanSpeed{ ac, rpm: value })?;
    use comms::DaemonResponse::*;
    match response {
        SetFanSpeed { result } => Some(result),
        response => {
            println!("Instead of SetFanSpeed got {response:?}");
            None
        }
    }
}

/// Read CPU temperature from hwmon (supports AMD k10temp/zenpower and Intel coretemp)
fn get_cpu_temperature() -> Option<f64> {
    // First, try to find AMD k10temp or zenpower hwmon
    if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let name_path = entry.path().join("name");
            if let Ok(name) = fs::read_to_string(&name_path) {
                let name = name.trim();
                // AMD CPU temperature sensors
                if name == "k10temp" || name == "zenpower" || name == "coretemp" {
                    // Try Tctl (AMD) or Package temp
                    let temp_path = entry.path().join("temp1_input");
                    if let Ok(content) = fs::read_to_string(&temp_path) {
                        if let Ok(temp) = content.trim().parse::<f64>() {
                            return Some(temp / 1000.0);
                        }
                    }
                }
            }
        }
    }
    
    // Fallback to thermal zones
    let paths = [
        "/sys/class/thermal/thermal_zone0/temp",
        "/sys/class/thermal/thermal_zone1/temp",
        "/sys/class/thermal/thermal_zone2/temp",
    ];
    
    for path in paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(temp) = content.trim().parse::<f64>() {
                return Some(temp / 1000.0);
            }
        }
    }
    None
}

/// Read dGPU temperature (NVIDIA RTX 5070 Ti)
fn get_gpu_temperature() -> Option<f64> {
    // Try NVIDIA SMI first (most reliable for NVIDIA GPUs)
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=temperature.gpu", "--format=csv,noheader,nounits"])
        .output()
    {
        if output.status.success() {
            if let Ok(temp_str) = String::from_utf8(output.stdout) {
                if let Ok(temp) = temp_str.trim().parse::<f64>() {
                    return Some(temp);
                }
            }
        }
    }
    
    // Fallback: Try hwmon paths for NVIDIA
    if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let name_path = entry.path().join("name");
            if let Ok(name) = fs::read_to_string(&name_path) {
                if name.trim() == "nvidia" {
                    let temp_path = entry.path().join("temp1_input");
                    if let Ok(content) = fs::read_to_string(&temp_path) {
                        if let Ok(temp) = content.trim().parse::<f64>() {
                            return Some(temp / 1000.0);
                        }
                    }
                }
            }
        }
    }
    None
}

/// Read current fan speed from daemon
fn get_current_fan_speed(ac: bool) -> Option<i32> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::GetFanSpeed { ac })?;
    use comms::DaemonResponse::*;
    match response {
        GetFanSpeed { rpm } => Some(rpm),
        _ => None
    }
}

/// Read system/CPU power consumption from RAPL (supports AMD and Intel)
fn get_system_power() -> Option<f64> {
    // Try AMD RAPL first, then Intel
    let energy_paths = [
        "/sys/class/powercap/amd-rapl:0/energy_uj",
        "/sys/class/powercap/amd_rapl/amd-rapl:0/energy_uj",
        "/sys/class/powercap/intel-rapl:0/energy_uj",
        "/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj",
    ];
    
    for path in &energy_paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(energy) = content.trim().parse::<u64>() {
                static LAST_ENERGY: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                static LAST_TIME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_micros() as u64;
                
                let prev_energy = LAST_ENERGY.swap(energy, std::sync::atomic::Ordering::Relaxed);
                let prev_time = LAST_TIME.swap(now, std::sync::atomic::Ordering::Relaxed);
                
                if prev_energy > 0 && prev_time > 0 && energy > prev_energy {
                    let delta_energy = energy - prev_energy;
                    let delta_time = now - prev_time;
                    if delta_time > 0 {
                        let power = delta_energy as f64 / delta_time as f64;
                        return Some(power);
                    }
                }
                return None; // Found path but need second reading
            }
        }
    }
    None
}

/// Read NVIDIA dGPU power consumption (RTX 5070 Ti)
fn get_dgpu_power() -> Option<f64> {
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=power.draw", "--format=csv,noheader,nounits"])
        .output()
    {
        if output.status.success() {
            if let Ok(power_str) = String::from_utf8(output.stdout) {
                if let Ok(power) = power_str.trim().parse::<f64>() {
                    return Some(power);
                }
            }
        }
    }
    None
}

/// Read NVIDIA dGPU utilization (RTX 5070 Ti)
fn get_dgpu_utilization() -> Option<u32> {
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=utilization.gpu", "--format=csv,noheader,nounits"])
        .output()
    {
        if output.status.success() {
            if let Ok(util_str) = String::from_utf8(output.stdout) {
                if let Ok(util) = util_str.trim().parse::<u32>() {
                    return Some(util);
                }
            }
        }
    }
    None
}

/// Read AMD iGPU (Radeon 870M) power from hwmon
fn get_igpu_power() -> Option<f64> {
    // Look for amdgpu hwmon device (870M iGPU)
    if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let name_path = entry.path().join("name");
            if let Ok(name) = fs::read_to_string(&name_path) {
                if name.trim() == "amdgpu" {
                    // Check if this is the iGPU by looking at power1_average
                    let power_path = entry.path().join("power1_average");
                    if let Ok(content) = fs::read_to_string(&power_path) {
                        if let Ok(power_uw) = content.trim().parse::<f64>() {
                            return Some(power_uw / 1_000_000.0); // Convert microwatts to watts
                        }
                    }
                }
            }
        }
    }
    
    // Fallback: Try Intel RAPL GT domain
    let paths = [
        "/sys/class/powercap/intel-rapl:0:1/energy_uj",
        "/sys/class/powercap/intel-rapl/intel-rapl:0/intel-rapl:0:1/energy_uj",
    ];
    
    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(energy) = content.trim().parse::<u64>() {
                static LAST_IGPU_ENERGY: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                static LAST_IGPU_TIME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_micros() as u64;
                
                let prev_energy = LAST_IGPU_ENERGY.swap(energy, std::sync::atomic::Ordering::Relaxed);
                let prev_time = LAST_IGPU_TIME.swap(now, std::sync::atomic::Ordering::Relaxed);
                
                if prev_energy > 0 && prev_time > 0 && energy > prev_energy {
                    let delta_energy = energy - prev_energy;
                    let delta_time = now - prev_time;
                    if delta_time > 0 {
                        return Some(delta_energy as f64 / delta_time as f64);
                    }
                }
            }
        }
    }
    None
}

/// Read AMD iGPU (Radeon 870M) utilization
fn get_igpu_utilization() -> Option<u32> {
    // Try AMD GPU busy percentage from amdgpu driver
    // Look for the integrated GPU (usually card0 or card1)
    for card in ["card0", "card1", "card2"] {
        let busy_path = format!("/sys/class/drm/{}/device/gpu_busy_percent", card);
        if let Ok(content) = fs::read_to_string(&busy_path) {
            if let Ok(util) = content.trim().parse::<u32>() {
                // Verify this is the iGPU by checking if it's amdgpu
                let driver_path = format!("/sys/class/drm/{}/device/driver", card);
                if let Ok(driver_link) = fs::read_link(&driver_path) {
                    if driver_link.to_string_lossy().contains("amdgpu") {
                        return Some(util);
                    }
                }
            }
        }
    }
    
    // Fallback: Try frequency-based estimation for Intel
    let paths = [
        "/sys/class/drm/card0/gt/gt0/rps_act_freq_mhz",
        "/sys/class/drm/card1/gt/gt0/rps_act_freq_mhz",
    ];
    let max_paths = [
        "/sys/class/drm/card0/gt/gt0/rps_max_freq_mhz",
        "/sys/class/drm/card1/gt/gt0/rps_max_freq_mhz",
    ];
    
    for (i, path) in paths.iter().enumerate() {
        if let Ok(act_content) = fs::read_to_string(path) {
            if let Ok(max_content) = fs::read_to_string(&max_paths[i]) {
                if let (Ok(act), Ok(max)) = (
                    act_content.trim().parse::<f64>(),
                    max_content.trim().parse::<f64>()
                ) {
                    if max > 0.0 {
                        return Some(((act / max) * 100.0) as u32);
                    }
                }
            }
        }
    }
    None
}

/// Read AMD iGPU (Radeon 870M) temperature
fn get_igpu_temperature() -> Option<f64> {
    // Look for amdgpu hwmon device
    if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let name_path = entry.path().join("name");
            if let Ok(name) = fs::read_to_string(&name_path) {
                if name.trim() == "amdgpu" {
                    // Junction/edge temperature
                    for temp_file in ["temp1_input", "temp2_input"] {
                        let temp_path = entry.path().join(temp_file);
                        if let Ok(content) = fs::read_to_string(&temp_path) {
                            if let Ok(temp) = content.trim().parse::<f64>() {
                                return Some(temp / 1000.0);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Create status bar with system info
fn create_status_bar() -> gtk::Box {
    // Main vertical container for two rows
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
    main_box.set_margin_start(16);
    main_box.set_margin_end(16);
    main_box.set_margin_top(8);
    main_box.set_margin_bottom(8);
    main_box.add_css_class("toolbar");
    
    // Top row: Temps and Fan
    let top_row = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    
    // CPU Temperature
    let cpu_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let cpu_icon = gtk::Image::from_icon_name("cpu-symbolic");
    cpu_icon.set_pixel_size(16);
    let cpu_label = gtk::Label::new(Some("CPU: --°C"));
    cpu_label.add_css_class("caption");
    cpu_box.append(&cpu_icon);
    cpu_box.append(&cpu_label);
    
    // iGPU Temperature (AMD Radeon 870M)
    let igpu_temp_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let igpu_temp_label = gtk::Label::new(Some("iGPU: --°C"));
    igpu_temp_label.add_css_class("caption");
    igpu_temp_box.append(&igpu_temp_label);
    
    // dGPU Temperature (NVIDIA RTX 5070 Ti)
    let gpu_temp_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let gpu_temp_icon = gtk::Image::from_icon_name("video-display-symbolic");
    gpu_temp_icon.set_pixel_size(16);
    let gpu_temp_label = gtk::Label::new(Some("dGPU: --°C"));
    gpu_temp_label.add_css_class("caption");
    gpu_temp_box.append(&gpu_temp_icon);
    gpu_temp_box.append(&gpu_temp_label);
    
    // Fan Speed
    let fan_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let fan_icon = gtk::Image::from_icon_name("weather-windy-symbolic");
    fan_icon.set_pixel_size(16);
    let fan_label = gtk::Label::new(Some("Fan: -- RPM"));
    fan_label.add_css_class("caption");
    fan_box.append(&fan_icon);
    fan_box.append(&fan_label);
    
    // Spacer for top row
    let spacer1 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer1.set_hexpand(true);
    
    // Power status
    let power_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let power_icon = gtk::Image::from_icon_name("battery-full-charging-symbolic");
    power_icon.set_pixel_size(16);
    let power_label = gtk::Label::new(Some("--"));
    power_label.add_css_class("caption");
    power_box.append(&power_icon);
    power_box.append(&power_label);
    
    top_row.append(&cpu_box);
    top_row.append(&igpu_temp_box);
    top_row.append(&gpu_temp_box);
    top_row.append(&fan_box);
    top_row.append(&spacer1);
    top_row.append(&power_box);
    
    // Bottom row: GPU stats
    let bottom_row = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    
    // iGPU Power
    let igpu_power_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let igpu_power_label = gtk::Label::new(Some("iGPU: --W"));
    igpu_power_label.add_css_class("caption");
    igpu_power_box.append(&igpu_power_label);
    
    // iGPU Utilization
    let igpu_util_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let igpu_util_label = gtk::Label::new(Some("iGPU: --%"));
    igpu_util_label.add_css_class("caption");
    igpu_util_box.append(&igpu_util_label);
    
    // dGPU Power
    let dgpu_power_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let dgpu_power_label = gtk::Label::new(Some("dGPU: --W"));
    dgpu_power_label.add_css_class("caption");
    dgpu_power_box.append(&dgpu_power_label);
    
    // dGPU Utilization
    let dgpu_util_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let dgpu_util_label = gtk::Label::new(Some("dGPU: --%"));
    dgpu_util_label.add_css_class("caption");
    dgpu_util_box.append(&dgpu_util_label);
    
    // Spacer for bottom row
    let spacer2 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer2.set_hexpand(true);
    
    bottom_row.append(&igpu_power_box);
    bottom_row.append(&igpu_util_box);
    bottom_row.append(&dgpu_power_box);
    bottom_row.append(&dgpu_util_box);
    bottom_row.append(&spacer2);
    
    main_box.append(&top_row);
    main_box.append(&bottom_row);
    
    // Update status periodically
    let cpu_label_ref = cpu_label.clone();
    let igpu_temp_label_ref = igpu_temp_label.clone();
    let gpu_temp_label_ref = gpu_temp_label.clone();
    let fan_label_ref = fan_label.clone();
    let power_label_ref = power_label.clone();
    let power_icon_ref = power_icon.clone();
    let igpu_power_label_ref = igpu_power_label.clone();
    let igpu_util_label_ref = igpu_util_label.clone();
    let dgpu_power_label_ref = dgpu_power_label.clone();
    let dgpu_util_label_ref = dgpu_util_label.clone();
    
    glib::timeout_add_local(Duration::from_secs(2), move || {
        // Update CPU temp
        match get_cpu_temperature() {
            Some(temp) => cpu_label_ref.set_text(&format!("CPU: {:.0}°C", temp)),
            None => cpu_label_ref.set_text("CPU: N/A"),
        }
        
        // Update iGPU temp (AMD Radeon 870M)
        match get_igpu_temperature() {
            Some(temp) => igpu_temp_label_ref.set_text(&format!("iGPU: {:.0}°C", temp)),
            None => igpu_temp_label_ref.set_text("iGPU: N/A"),
        }
        
        // Update dGPU temp (NVIDIA RTX 5070 Ti)
        match get_gpu_temperature() {
            Some(temp) => gpu_temp_label_ref.set_text(&format!("dGPU: {:.0}°C", temp)),
            None => gpu_temp_label_ref.set_text("dGPU: Off"),
        }
        
        // Update fan speed
        let on_ac = check_if_running_on_ac_power().unwrap_or(true);
        match get_current_fan_speed(on_ac) {
            Some(0) => fan_label_ref.set_text("Fan: Auto"),
            Some(rpm) => fan_label_ref.set_text(&format!("Fan: {} RPM", rpm)),
            None => fan_label_ref.set_text("Fan: N/A"),
        }
        
        // Update iGPU power
        match get_igpu_power() {
            Some(power) => igpu_power_label_ref.set_text(&format!("iGPU: {:.1}W", power)),
            None => igpu_power_label_ref.set_text("iGPU: N/A"),
        }
        
        // Update iGPU utilization
        match get_igpu_utilization() {
            Some(util) => igpu_util_label_ref.set_text(&format!("iGPU: {}%", util)),
            None => igpu_util_label_ref.set_text("iGPU: N/A"),
        }
        
        // Update dGPU power
        match get_dgpu_power() {
            Some(power) => dgpu_power_label_ref.set_text(&format!("dGPU: {:.1}W", power)),
            None => dgpu_power_label_ref.set_text("dGPU: Off"),
        }
        
        // Update dGPU utilization
        match get_dgpu_utilization() {
            Some(util) => dgpu_util_label_ref.set_text(&format!("dGPU: {}%", util)),
            None => dgpu_util_label_ref.set_text("dGPU: Off"),
        }
        
        // Update power status
        match check_if_running_on_ac_power() {
            Some(true) => {
                power_label_ref.set_text("AC Power");
                power_icon_ref.set_icon_name(Some("battery-full-charging-symbolic"));
            }
            Some(false) => {
                power_label_ref.set_text("Battery");
                power_icon_ref.set_icon_name(Some("battery-symbolic"));
            }
            None => {
                power_label_ref.set_text("Unknown");
            }
        }
        
        glib::ControlFlow::Continue
    });
    
    main_box
}


fn main() {
    setup_panic_hook();
    
    // Adwaita setup
    let app = adw::Application::builder()
        .application_id("com.encomjp.razer-settings")
        .build();

    app.connect_startup(|_| {
        adw::init().ok();
    });

    app.connect_activate(move |app| {
        let device_file = std::fs::read_to_string(service::DEVICE_FILE).unwrap_or("[]".into());
        let devices: Vec<SupportedDevice> = serde_json::from_str(&device_file)
            .expect("Failed to parse device file");

        let device_name = get_device_name()
            .expect("Failed to get device name");
        
        // Find device
        let device = devices.iter().find(|d| d.name == device_name)
            .expect("Failed to get device info").clone();

        // Create window with modern styling
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Razer Control")
            .default_width(850)
            .default_height(700)
            .build();

        // Main content with proper spacing
        let content_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        
        // Modern Header Bar with subtitle
        let header_bar = adw::HeaderBar::new();
        header_bar.set_show_end_title_buttons(true);
        
        // View Switcher for tabs
        let view_switcher = adw::ViewSwitcher::new();
        view_switcher.set_policy(adw::ViewSwitcherPolicy::Wide);
        
        // Stack for pages
        let view_stack = adw::ViewStack::new();
        view_switcher.set_stack(Some(&view_stack));
        header_bar.set_title_widget(Some(&view_switcher));
        
        // Clamp for better responsive design
        let clamp = adw::Clamp::new();
        clamp.set_maximum_size(900);
        clamp.set_tightening_threshold(600);
        
        let scrolled_window = gtk::ScrolledWindow::new();
        scrolled_window.set_vexpand(true);
        scrolled_window.set_hscrollbar_policy(gtk::PolicyType::Never);
        scrolled_window.set_child(Some(&clamp));
        clamp.set_child(Some(&view_stack));
        
        content_box.append(&header_bar);
        content_box.append(&scrolled_window);

        // Create pages with icons
        let ac_page = make_page(true, device.clone());
        let page = view_stack.add_titled(&ac_page.page, Some("AC"), "AC Power");
        page.set_icon_name(Some("battery-full-charging-symbolic"));
        
        let battery_page = make_page(false, device.clone());
        let page = view_stack.add_titled(&battery_page.page, Some("Battery"), "Battery");
        page.set_icon_name(Some("battery-symbolic"));
        
        let general_page = make_general_page();
        let page = view_stack.add_titled(&general_page.page, Some("General"), "Settings");
        page.set_icon_name(Some("preferences-system-symbolic"));
        
        let about_page = make_about_page(device.clone());
        let page = view_stack.add_titled(&about_page.page, Some("About"), "About");
        page.set_icon_name(Some("help-about-symbolic"));

        // Check power state to set default page
        match check_if_running_on_ac_power() {
            Some(false) => view_stack.set_visible_child_name("Battery"),
            _ => {}
        }
        
        // Status bar at bottom
        let status_bar = create_status_bar();
        content_box.append(&status_bar);
        
        window.set_content(Some(&content_box));
        window.present();
    });

    app.run();
}

fn make_page(ac: bool, device: SupportedDevice) -> SettingsPage {
    let fan_speed = get_fan_speed(ac).unwrap_or(0);
    let brightness = get_brightness(ac).unwrap_or(100);
    let power = get_power(ac);

    let min_fan_speed = *device.fan.get(0).unwrap_or(&0) as f64;
    let max_fan_speed = *device.fan.get(1).unwrap_or(&5000) as f64;

    let settings_page = SettingsPage::new();

    // Logo Control Section
    if device.has_logo() {
        let logo = get_logo(ac).unwrap_or(1);
        let section = settings_page.add_section(Some("Logo Control"));
        
        let combo = ComboRow::new(
            "Logo Mode",
            "Control your Razer logo lighting",
            &["Off", "On", "Breathing"],
            logo as u32
        );
        combo.combo.connect_changed(move |c| {
            let logo = c.active().unwrap_or(0) as u8;
            set_logo(ac, logo);
        });
        section.add_row(&combo.row);
    }

    // Power Management Section
    if let Some(power) = power {
        let section = settings_page.add_section(Some("Performance Profile"));
        
        let power_combo = ComboRow::new(
            "Power Profile",
            "Select your performance mode",
            &["Balanced", "Gaming", "Creator", "Silent", "Custom"],
            power.0 as u32
        );
        section.add_row(&power_combo.row);

        // CPU Boost Control
        let cpu_options = if device.can_boost() {
            vec!["Low", "Medium", "High", "Boost"]
        } else {
            vec!["Low", "Medium", "High"]
        };
        let cpu_combo = ComboRow::new(
            "CPU Performance",
            "Adjust processor performance level",
            &cpu_options.iter().map(|s| *s).collect::<Vec<_>>(),
            power.1 as u32
        );
        section.add_row(&cpu_combo.row);

        // GPU Boost Control
        let gpu_combo = ComboRow::new(
            "GPU Performance",
            "Adjust graphics performance level",
            &["Low", "Medium", "High"],
            power.2 as u32
        );
        section.add_row(&gpu_combo.row);

        // Set visibility based on custom mode
        let show_boost = power.0 == 4;
        cpu_combo.row.set_visible(show_boost);
        gpu_combo.row.set_visible(show_boost);

        // Connect power profile changes
        let cpu_row_ref = cpu_combo.row.clone();
        let gpu_row_ref = gpu_combo.row.clone();
        let cpu_combo_ref = cpu_combo.combo.clone();
        let gpu_combo_ref = gpu_combo.combo.clone();
        
        power_combo.combo.connect_changed(glib::clone!(
            @weak cpu_combo_ref, @weak gpu_combo_ref, @weak cpu_row_ref, @weak gpu_row_ref => move |pp| {
                let profile = pp.active().unwrap_or(0) as u8;
                let cpu = cpu_combo_ref.active().unwrap_or(0) as u8;
                let gpu = gpu_combo_ref.active().unwrap_or(0) as u8;
                set_power(ac, (profile, cpu, gpu));
                let show = profile == 4;
                cpu_row_ref.set_visible(show);
                gpu_row_ref.set_visible(show);
            }
        ));
        
        let power_combo_ref = power_combo.combo.clone();
        let gpu_combo_ref2 = gpu_combo.combo.clone();
        cpu_combo.combo.connect_changed(glib::clone!(
            @weak power_combo_ref, @weak gpu_combo_ref2 => move |cb| {
                let profile = power_combo_ref.active().unwrap_or(0) as u8;
                let cpu = cb.active().unwrap_or(0) as u8;
                let gpu = gpu_combo_ref2.active().unwrap_or(0) as u8;
                set_power(ac, (profile, cpu, gpu));
            }
        ));
        
        let power_combo_ref2 = power_combo.combo.clone();
        let cpu_combo_ref2 = cpu_combo.combo.clone();
        gpu_combo.combo.connect_changed(glib::clone!(
            @weak power_combo_ref2, @weak cpu_combo_ref2 => move |gb| {
                let profile = power_combo_ref2.active().unwrap_or(0) as u8;
                let cpu = cpu_combo_ref2.active().unwrap_or(0) as u8;
                let gpu = gb.active().unwrap_or(0) as u8;
                set_power(ac, (profile, cpu, gpu));
            }
        ));
    }

    // Fan Control Section
    let section = settings_page.add_section(Some("Cooling Control"));
    
    let auto = fan_speed == 0;
    let fan_switch = SwitchRow::new(
        "Automatic Fan Control",
        "Let the system manage fan speed",
        auto
    );
    section.add_row(&fan_switch.row);

    // Fan speed slider
    let fan_slider = SliderRow::new(
        "Fan Speed (RPM)",
        "Manually control cooling performance",
        min_fan_speed, max_fan_speed, 100.0,
        if auto { min_fan_speed } else { fan_speed as f64 }
    );
    fan_slider.add_mark(min_fan_speed, Some("Min"));
    fan_slider.add_mark(max_fan_speed, Some("Max"));
    fan_slider.scale.set_sensitive(!auto);
    section.add_row(&fan_slider.container);

    // Fan control logic
    let switch_ref = fan_switch.switch.clone();
    fan_slider.scale.connect_value_changed(glib::clone!(@weak switch_ref => move |sc| {
        let value = sc.value();
        set_fan_speed(ac, value as i32);
        switch_ref.set_active(false);
    }));
    
    let scale_ref = fan_slider.scale.clone();
    fan_switch.switch.connect_state_set(glib::clone!(@weak scale_ref => @default-return glib::Propagation::Proceed, move |_sw, state| {
        if state {
            set_fan_speed(ac, 0);
        } else {
            set_fan_speed(ac, min_fan_speed as i32);
            scale_ref.set_value(min_fan_speed);
        }
        scale_ref.set_sensitive(!state);
        glib::Propagation::Proceed
    }));

    // Keyboard Brightness Section
    let section = settings_page.add_section(Some("Keyboard Lighting"));
    
    let brightness_slider = SliderRow::new(
        "Brightness Level",
        "Adjust keyboard backlight intensity",
        0.0, 100.0, 1.0,
        brightness as f64
    );
    brightness_slider.add_mark(0.0, Some("Off"));
    brightness_slider.add_mark(50.0, Some("50%"));
    brightness_slider.add_mark(100.0, Some("100%"));
    section.add_row(&brightness_slider.container);

    brightness_slider.scale.connect_value_changed(move |sc| {
        set_brightness(ac, sc.value() as u8);
    });

    settings_page
}

fn make_general_page() -> SettingsPage {
    let bho = get_bho();
    let page = SettingsPage::new();

    // Keyboard Effects Section
    let section = page.add_section(Some("Keyboard Effects"));
    
    let effect_combo = ComboRow::new(
        "Effect Type",
        "Choose your keyboard lighting effect",
        &["Static", "Static Gradient", "Wave Gradient", "Breathing"],
        0
    );
    section.add_row(&effect_combo.row);

    let color1 = ColorRow::new("Primary Color", "Select the main color");
    section.add_row(&color1.row);

    let color2 = ColorRow::new("Secondary Color", "For gradient effects");
    section.add_row(&color2.row);

    // Apply button in its own box for better layout
    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    button_box.set_margin_top(12);
    button_box.set_margin_bottom(12);
    button_box.set_margin_start(12);
    button_box.set_margin_end(12);
    button_box.set_halign(gtk::Align::End);
    
    let button = gtk::Button::with_label("Apply Effect");
    button.add_css_class("suggested-action");
    button.set_margin_start(8);
    button_box.append(&button);
    section.add_row(&button_box);
    
    let effect_ref = effect_combo.combo.clone();
    let color1_ref = color1.button.clone();
    let color2_ref = color2.button.clone();
    button.connect_clicked(move |_| {
        let color = color1_ref.rgba();
        let red = (color.red() * 255.0) as u8;
        let green = (color.green() * 255.0) as u8;
        let blue = (color.blue() * 255.0) as u8;

        let c2 = color2_ref.rgba();
        let red2 = (c2.red() * 255.0) as u8;
        let green2 = (c2.green() * 255.0) as u8;
        let blue2 = (c2.blue() * 255.0) as u8;
        
        let effect = effect_ref.active().unwrap_or(0);
        match effect {
            0 => { set_effect("static", vec![red, green, blue]); },
            1 => { set_effect("static_gradient", vec![red, green, blue, red2, green2, blue2]); },
            2 => { set_effect("wave_gradient", vec![red, green, blue, red2, green2, blue2]); },
            3 => { set_effect("breathing_single", vec![red, green, blue, 10]); },
            _ => {}
        }
    });

    // Battery Health Optimizer Section
    if let Some(bho) = bho {
        let section = page.add_section(Some("Battery Health Optimizer"));
        
        let bho_switch = SwitchRow::new(
            "Enable Battery Health Mode",
            "Limits charging to extend battery lifespan",
            bho.0
        );
        section.add_row(&bho_switch.row);

        let bho_slider = SliderRow::new(
            "Charge Limit",
            "Maximum battery charge level (%)",
            50.0, 80.0, 5.0,
            bho.1 as f64
        );
        bho_slider.add_mark(50.0, Some("50%"));
        bho_slider.add_mark(65.0, Some("65%"));
        bho_slider.add_mark(80.0, Some("80%"));
        bho_slider.scale.set_sensitive(bho.0);
        section.add_row(&bho_slider.container);

        // BHO Logic
        let switch_ref = bho_switch.switch.clone();
        bho_slider.scale.connect_value_changed(glib::clone!(@weak switch_ref => move |sc| {
            let is_on = switch_ref.is_active();
            let threshold = sc.value() as u8;
            set_bho(is_on, threshold);
        }));
        
        let scale_ref = bho_slider.scale.clone();
        bho_switch.switch.connect_state_set(glib::clone!(@weak scale_ref => @default-return glib::Propagation::Proceed, move |_sw, state| {
            let threshold = scale_ref.value() as u8;
            set_bho(state, threshold);
            scale_ref.set_sensitive(state);
            glib::Propagation::Proceed
        }));
    }

    page
}

fn make_about_page(device: SupportedDevice) -> SettingsPage {
    let page = SettingsPage::new();
    
    // Application Info Section
    let section = page.add_section(Some("Application"));
    
    let app_name = gtk::Label::new(Some("Razer Control (Revived)"));
    app_name.add_css_class("title-2");
    let row = SettingsRow::new("Name", &app_name);
    section.add_row(&row.row);
    
    let version_label = gtk::Label::new(Some("v0.2.0"));
    let row = SettingsRow::new("Version", &version_label);
    section.add_row(&row.row);
    
    let url = gtk::LinkButton::with_label(
        "https://github.com/encomjp/razer-control-revived", 
        "View on GitHub"
    );
    let row = SettingsRow::new("Repository", &url);
    row.set_subtitle("Report issues and contribute");
    section.add_row(&row.row);

    // Device Information Section
    let section = page.add_section(Some("Device Information"));
    
    let name_label = gtk::Label::new(Some(&device.name));
    name_label.set_wrap(true);
    let row = SettingsRow::new("Model", &name_label);
    section.add_row(&row.row);
    
    let features = device.features.join(", ");
    let features_label = gtk::Label::new(Some(&features));
    features_label.set_wrap(true);
    let row = SettingsRow::new("Features", &features_label);
    section.add_row(&row.row);
    
    let fan_min = device.fan.get(0).unwrap_or(&0);
    let fan_max = device.fan.get(1).unwrap_or(&5000);
    let fan_range = format!("{} - {} RPM", fan_min, fan_max);
    let fan_label = gtk::Label::new(Some(&fan_range));
    let row = SettingsRow::new("Fan Range", &fan_label);
    section.add_row(&row.row);

    // About Section
    let section = page.add_section(Some("About"));
    
    let desc_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
    desc_box.set_margin_top(12);
    desc_box.set_margin_bottom(12);
    desc_box.set_margin_start(12);
    desc_box.set_margin_end(12);
    
    let description = gtk::Label::new(Some(
        "Open-source control center for Razer laptops on Linux.\n\
        Manage power profiles, fan speeds, keyboard lighting, and more."
    ));
    description.set_wrap(true);
    description.set_justify(gtk::Justification::Center);
    description.add_css_class("dim-label");
    desc_box.append(&description);
    section.add_row(&desc_box);
    page
}
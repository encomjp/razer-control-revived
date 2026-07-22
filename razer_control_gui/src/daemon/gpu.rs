use std::fs;
use std::path::Path;
use std::process::Command;

use super::comms::GpuInfo;

/// Known GPU vendor IDs
const VENDOR_NVIDIA: &str = "0x10de";
const VENDOR_AMD: &str = "0x1002";
const VENDOR_INTEL: &str = "0x8086";

/// Scan PCI devices and return all detected GPUs
pub fn discover_gpus() -> Vec<GpuInfo> {
    let mut gpus = Vec::new();
    let pci_dir = Path::new("/sys/bus/pci/devices");

    let entries = match fs::read_dir(pci_dir) {
        Ok(e) => e,
        Err(_) => return gpus,
    };

    for entry in entries.flatten() {
        let dev_path = entry.path();
        let class = read_sysfs_trimmed(&dev_path.join("class"));

        // Check if this is a GPU (VGA controller or 3D controller)
        let is_gpu = match class.as_deref() {
            Some(c) => c.starts_with("0x0300") || c.starts_with("0x0302"),
            None => false,
        };
        if !is_gpu {
            continue;
        }

        let vendor = read_sysfs_trimmed(&dev_path.join("vendor"));
        let device_id = read_sysfs_trimmed(&dev_path.join("device"));
        let pci_slot = entry.file_name().to_string_lossy().to_string();

        // Determine driver from symlink
        let driver = match fs::read_link(dev_path.join("driver")) {
            Ok(link) => link
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default(),
            Err(_) => String::new(),
        };

        // Determine GPU type based on vendor
        let gpu_type = match vendor.as_deref() {
            Some(VENDOR_NVIDIA) => "dgpu".to_string(),
            Some(VENDOR_AMD) => {
                if driver == "amdgpu" {
                    // On hybrid laptops, AMD is typically the iGPU
                    "igpu".to_string()
                } else {
                    "dgpu".to_string()
                }
            }
            Some(VENDOR_INTEL) => "igpu".to_string(),
            _ => "unknown".to_string(),
        };

        // Get runtime PM status
        let runtime_status = read_sysfs_trimmed(&dev_path.join("power/runtime_status"))
            .unwrap_or_else(|| "unsupported".to_string());

        // Build a human-readable name
        let name = resolve_gpu_name(vendor.as_deref(), device_id.as_deref(), &driver);

        gpus.push(GpuInfo {
            name,
            pci_slot,
            driver,
            gpu_type,
            runtime_status,
        });
    }

    // Sort: iGPU first, then dGPU
    gpus.sort_by(|a, b| a.gpu_type.cmp(&b.gpu_type));
    gpus
}

/// Find the first dGPU PCI slot
fn find_dgpu_path() -> Option<std::path::PathBuf> {
    let gpus = discover_gpus();
    gpus.into_iter()
        .find(|g| g.gpu_type == "dgpu")
        .map(|g| Path::new("/sys/bus/pci/devices").join(&g.pci_slot))
}

/// Check if dGPU runtime PM is set to "auto" (power saving enabled)
pub fn get_dgpu_runtime_pm() -> bool {
    if let Some(dgpu_path) = find_dgpu_path() {
        let control = read_sysfs_trimmed(&dgpu_path.join("power/control"));
        matches!(control.as_deref(), Some("auto"))
    } else {
        false
    }
}

/// Set dGPU runtime PM: true = "auto" (allow suspend), false = "on" (always active)
pub fn set_dgpu_runtime_pm(enabled: bool) -> bool {
    if let Some(dgpu_path) = find_dgpu_path() {
        let value = if enabled { "auto" } else { "on" };
        let control_path = dgpu_path.join("power/control");
        match fs::write(&control_path, value) {
            Ok(_) => {
                println!("Set dGPU runtime PM to '{}'", value);
                true
            }
            Err(e) => {
                eprintln!("Failed to write to {:?}: {}", control_path, e);
                false
            }
        }
    } else {
        eprintln!("No dGPU found for runtime PM control");
        false
    }
}

/// Check if envycontrol is installed
pub fn envycontrol_available() -> bool {
    Command::new("which")
        .arg("envycontrol")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Query current envycontrol GPU mode
pub fn get_envycontrol_mode() -> String {
    match Command::new("envycontrol").arg("--query").output() {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            // envycontrol --query outputs something like "Current mode: hybrid"
            if let Some(mode) = stdout.to_lowercase().split("mode:").nth(1) {
                mode.trim().to_string()
            } else {
                stdout.trim().to_lowercase()
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("envycontrol --query failed: {}", stderr);
            "unknown".to_string()
        }
        Err(e) => {
            eprintln!("Failed to run envycontrol: {}", e);
            "unknown".to_string()
        }
    }
}

/// Set GPU mode via envycontrol. Returns (success, message).
pub fn set_envycontrol_mode(mode: &str) -> (bool, String) {
    let valid_modes = ["integrated", "hybrid", "nvidia"];
    if !valid_modes.contains(&mode) {
        return (
            false,
            format!(
                "Invalid mode '{}'. Use: integrated, hybrid, or nvidia",
                mode
            ),
        );
    }

    match Command::new("envycontrol").args(["-s", mode]).output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            if output.status.success() {
                let msg = format!(
                    "GPU mode set to '{}'. Logout required to take effect.",
                    mode
                );
                println!("{}", msg);
                (true, msg)
            } else {
                let msg = format!("envycontrol failed: {}{}", stdout, stderr);
                eprintln!("{}", msg);
                (false, msg)
            }
        }
        Err(e) => {
            let msg = format!("Failed to run envycontrol: {}", e);
            eprintln!("{}", msg);
            (false, msg)
        }
    }
}

/// Read a sysfs file, returning trimmed content
fn read_sysfs_trimmed(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

/// Resolve a human-readable GPU name from vendor/device IDs and driver
fn resolve_gpu_name(vendor: Option<&str>, device_id: Option<&str>, driver: &str) -> String {
    // Try nvidia-smi for NVIDIA GPUs
    if vendor == Some(VENDOR_NVIDIA)
        && let Ok(output) = Command::new("nvidia-smi")
            .args(["--query-gpu=name", "--format=csv,noheader,nounits"])
            .output()
        && output.status.success()
    {
        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !name.is_empty() {
            return name;
        }
    }

    // Try lspci for a name
    if let Some(dev_id) = device_id {
        // Strip 0x prefix for lspci lookup
        let vid = vendor.unwrap_or("").trim_start_matches("0x");
        let did = dev_id.trim_start_matches("0x");
        if let Ok(output) = Command::new("lspci")
            .args(["-d", &format!("{}:{}", vid, did), "-mm"])
            .output()
            && output.status.success()
        {
            let line = String::from_utf8_lossy(&output.stdout);
            // lspci -mm format: Slot "Class" "Vendor" "Device" ...
            // Extract the device name (4th quoted field)
            let fields: Vec<&str> = line.split('"').collect();
            if fields.len() >= 8 {
                let vendor_name = fields[3];
                let device_name = fields[5];
                return format!("{} {}", vendor_name, device_name);
            }
        }
    }

    // Fallback: vendor + driver
    let vendor_name = match vendor {
        Some(VENDOR_NVIDIA) => "NVIDIA",
        Some(VENDOR_AMD) => "AMD",
        Some(VENDOR_INTEL) => "Intel",
        _ => "Unknown",
    };
    if driver.is_empty() {
        format!("{} GPU", vendor_name)
    } else {
        format!("{} GPU ({})", vendor_name, driver)
    }
}

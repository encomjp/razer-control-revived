use std::fs;
use std::path::{Path, PathBuf};

const NVIDIA_VENDOR_ID: &str = "0x10de";

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NvidiaTelemetry {
    pub temperature: Option<f64>,
    pub power: Option<f64>,
    pub utilization: Option<u32>,
}

/// Read NVIDIA telemetry without invoking vendor tools.
///
/// Running `nvidia-smi` can load the NVIDIA kernel module or wake a runtime-
/// suspended dGPU. The GUI polls this function regularly, so only sysfs files
/// belonging to a device that is explicitly reported as active are read.
pub fn read_nvidia_telemetry() -> NvidiaTelemetry {
    read_nvidia_telemetry_from(Path::new("/sys"))
}

fn read_nvidia_telemetry_from(sysfs: &Path) -> NvidiaTelemetry {
    active_nvidia_devices(sysfs)
        .into_iter()
        .find_map(|device| read_device_telemetry(&device))
        .unwrap_or_default()
}

fn active_nvidia_devices(sysfs: &Path) -> Vec<PathBuf> {
    if !sysfs.join("module/nvidia").is_dir() {
        return Vec::new();
    }

    let Ok(devices) = fs::read_dir(sysfs.join("bus/pci/devices")) else {
        return Vec::new();
    };

    let mut devices = devices
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let vendor_is_nvidia = fs::read_to_string(path.join("vendor"))
                .is_ok_and(|vendor| vendor.trim().eq_ignore_ascii_case(NVIDIA_VENDOR_ID));
            let driver_is_nvidia = fs::read_link(path.join("driver"))
                .ok()
                .and_then(|driver| driver.file_name().map(|name| name == "nvidia"))
                .unwrap_or(false);
            let is_active = fs::read_to_string(path.join("power/runtime_status"))
                .is_ok_and(|status| status.trim() == "active");

            (vendor_is_nvidia && driver_is_nvidia && is_active).then_some(path)
        })
        .collect::<Vec<_>>();
    devices.sort();
    devices
}

fn read_device_telemetry(device: &Path) -> Option<NvidiaTelemetry> {
    let mut hwmon = fs::read_dir(device.join("hwmon"))
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    hwmon.sort();

    let temperature = hwmon
        .iter()
        .find_map(|path| read_number::<f64>(&path.join("temp1_input")))
        .map(|value| value / 1_000.0);
    let power = hwmon
        .iter()
        .flat_map(|path| {
            ["power1_average", "power1_input"]
                .map(|name| path.join(name))
                .into_iter()
        })
        .find_map(|path| read_number::<f64>(&path))
        .map(|value| value / 1_000_000.0);
    let utilization =
        read_number::<u32>(&device.join("gpu_busy_percent")).filter(|value| *value <= 100);

    let telemetry = NvidiaTelemetry {
        temperature,
        power,
        utilization,
    };
    (telemetry != NvidiaTelemetry::default()).then_some(telemetry)
}

fn read_number<T: std::str::FromStr>(path: &Path) -> Option<T> {
    fs::read_to_string(path).ok()?.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    struct TestSysfs(PathBuf);

    impl TestSysfs {
        fn new() -> Self {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "razer-control-gpu-monitor-{}-{id}",
                std::process::id()
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn add_nvidia_gpu(
            &self,
            slot: &str,
            runtime_status: Option<&str>,
            bound: bool,
            temperature_millicelsius: Option<u32>,
            power_microwatts: Option<u64>,
            utilization: Option<u32>,
        ) {
            let module = self.0.join("module/nvidia");
            let device = self.0.join("bus/pci/devices").join(slot);
            fs::create_dir_all(&module).unwrap();
            fs::create_dir_all(device.join("power")).unwrap();
            fs::write(device.join("vendor"), "0x10de\n").unwrap();
            if let Some(status) = runtime_status {
                fs::write(device.join("power/runtime_status"), status).unwrap();
            }
            if bound {
                symlink(&module, device.join("driver")).unwrap();
            }

            if temperature_millicelsius.is_some() || power_microwatts.is_some() {
                let hwmon = device.join("hwmon/hwmon0");
                fs::create_dir_all(&hwmon).unwrap();
                if let Some(value) = temperature_millicelsius {
                    fs::write(hwmon.join("temp1_input"), value.to_string()).unwrap();
                }
                if let Some(value) = power_microwatts {
                    fs::write(hwmon.join("power1_average"), value.to_string()).unwrap();
                }
            }
            if let Some(value) = utilization {
                fs::write(device.join("gpu_busy_percent"), value.to_string()).unwrap();
            }
        }
    }

    impl Drop for TestSysfs {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn returns_no_telemetry_when_kernel_module_is_unloaded() {
        let sysfs = TestSysfs::new();
        assert_eq!(
            read_nvidia_telemetry_from(sysfs.path()),
            NvidiaTelemetry::default()
        );
    }

    #[test]
    fn fails_closed_for_unbound_or_indeterminate_devices() {
        let sysfs = TestSysfs::new();
        sysfs.add_nvidia_gpu(
            "0000:01:00.0",
            Some("active"),
            false,
            Some(50_000),
            None,
            None,
        );
        sysfs.add_nvidia_gpu("0000:02:00.0", None, true, Some(51_000), None, None);

        assert_eq!(
            read_nvidia_telemetry_from(sysfs.path()),
            NvidiaTelemetry::default()
        );
    }

    #[test]
    fn ignores_suspended_and_transitional_devices() {
        let sysfs = TestSysfs::new();
        for (index, status) in ["suspended", "suspending", "resuming"].iter().enumerate() {
            sysfs.add_nvidia_gpu(
                &format!("0000:0{}:00.0", index + 1),
                Some(status),
                true,
                Some(50_000),
                None,
                None,
            );
        }

        assert_eq!(
            read_nvidia_telemetry_from(sysfs.path()),
            NvidiaTelemetry::default()
        );
    }

    #[test]
    fn reads_sysfs_metrics_from_an_active_nvidia_gpu() {
        let sysfs = TestSysfs::new();
        sysfs.add_nvidia_gpu(
            "0000:01:00.0",
            Some("active"),
            true,
            Some(51_000),
            Some(12_750_000),
            Some(34),
        );

        assert_eq!(
            read_nvidia_telemetry_from(sysfs.path()),
            NvidiaTelemetry {
                temperature: Some(51.0),
                power: Some(12.75),
                utilization: Some(34),
            }
        );
    }

    #[test]
    fn skips_a_suspended_gpu_and_reads_an_active_one() {
        let sysfs = TestSysfs::new();
        sysfs.add_nvidia_gpu(
            "0000:01:00.0",
            Some("suspended"),
            true,
            Some(90_000),
            None,
            None,
        );
        sysfs.add_nvidia_gpu(
            "0000:02:00.0",
            Some("active"),
            true,
            Some(48_000),
            None,
            None,
        );

        assert_eq!(
            read_nvidia_telemetry_from(sysfs.path()).temperature,
            Some(48.0)
        );
    }

    #[test]
    fn rejects_out_of_range_utilization() {
        let sysfs = TestSysfs::new();
        sysfs.add_nvidia_gpu("0000:01:00.0", Some("active"), true, None, None, Some(101));

        assert_eq!(
            read_nvidia_telemetry_from(sysfs.path()),
            NvidiaTelemetry::default()
        );
    }

    #[test]
    fn reads_utilization_without_hwmon() {
        let sysfs = TestSysfs::new();
        sysfs.add_nvidia_gpu("0000:01:00.0", Some("active"), true, None, None, Some(42));

        assert_eq!(
            read_nvidia_telemetry_from(sysfs.path()),
            NvidiaTelemetry {
                temperature: None,
                power: None,
                utilization: Some(42),
            }
        );
    }

    #[test]
    fn selects_the_lowest_active_pci_slot_deterministically() {
        let sysfs = TestSysfs::new();
        sysfs.add_nvidia_gpu(
            "0000:02:00.0",
            Some("active"),
            true,
            Some(60_000),
            None,
            None,
        );
        sysfs.add_nvidia_gpu(
            "0000:01:00.0",
            Some("active"),
            true,
            Some(45_000),
            None,
            None,
        );

        assert_eq!(
            read_nvidia_telemetry_from(sysfs.path()).temperature,
            Some(45.0)
        );
    }
}

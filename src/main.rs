use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

enum FileList {
    LedFile,
    AsusThermalPolicy,
    AsusFanPolicy,
    FstsThermalPolicy,
    FstsFanPolicy,
}

impl FileList {
    fn path(&self) -> &str {
        match self {
            FileList::LedFile => "/sys/class/leds/asus::kbd_backlight/brightness",
            FileList::AsusThermalPolicy => {
                "/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy"
            }
            FileList::AsusFanPolicy => "/sys/devices/platform/asus-nb-wmi/fan_boost_mode",
            FileList::FstsThermalPolicy => "/sys/devices/platform/faustus/throttle_thermal_policy",
            FileList::FstsFanPolicy => "/sys/devices/platform/faustus/fan_boost_mode",
        }
    }
}

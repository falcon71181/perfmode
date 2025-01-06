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

#[derive(Debug)]
enum Operation {
    Off,
    Min,
    Med,
    Max,
    Silent,
    Balanced,
    Turbo,
    Overboost,
    Default,
    Get,
}

#[derive(Debug)]
enum Operator {
    Led,
    Fan,
    Thermal,
    Help,
}

#[derive(Debug)]
enum Error {
    InvalidArgv,
    BadFp,
    NoPermission,
    InvalidArgFun,
    Unknown,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidArgv => write!(f, "Invalid arguments\nView help with: perfmode -help"),
            Error::BadFp => write!(f, "Bad file pointer"),
            Error::NoPermission => write!(f, "No permission"),
            Error::InvalidArgFun => write!(f, "Invalid Argument to function"),
            Error::Unknown => write!(f, "Invalid Error Reported!"),
        }
    }
}

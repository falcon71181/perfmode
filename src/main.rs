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
    FileWriteError,
    Unknown,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidArgv => write!(f, "Invalid arguments\nView help with: perfmode -help"),
            Error::BadFp => write!(f, "Bad file pointer"),
            Error::NoPermission => write!(f, "No permission"),
            Error::InvalidArgFun => write!(f, "Invalid Argument to function"),
            Error::FileWriteError => write!(f, "Unable to write to file"),
            Error::Unknown => write!(f, "Invalid Error Reported!"),
        }
    }
}

fn print_help() {
    println!(
        "\x1b[32mPerfmode - Manage performance mode of your asus laptop\x1b[m\n\n\
        \x1b[1;4;31mUsage\x1b[m\n\
        \tperfmode -option arg\n\n\
        \x1b[1;4;31mOptions\x1b[m\n\n\
        \x1b[1;4;31mFan Control\x1b[m\n\
        \x1b[31m\t-fan turbo\x1b[m          Turbo Mode\n\
        \x1b[31m\t-fan balanced\x1b[m       Balanced Mode\n\
        \x1b[31m\t-fan silent\x1b[m         Silent Mode\n\n\
        \x1b[1;4;31mThermal Policy\x1b[m\n\
        \x1b[31m\t-thermal overboost\x1b[m  Overboost Mode\n\
        \x1b[31m\t-thermal default\x1b[m    Default Mode\n\
        \x1b[31m\t-thermal silent\x1b[m     Silent Mode\n\n\
        \x1b[1;4;31mKeyboard Backlight\x1b[m\n\
        \x1b[31m\t-led off\x1b[m            Turn off Backlight\n\
        \x1b[31m\t-led min\x1b[m            Minimum Backlight\n\
        \x1b[31m\t-led med\x1b[m            Medium  Backlight\n\
        \x1b[31m\t-led max\x1b[m            Maximum Backlight\n\n\
        \x1b[1;4;31mCommon option for all kinds of operations\x1b[m\n\
        \x1b[31m\tget\x1b[m                 get the current thermal, led, fan mode\n\
        \x1b[1;4;31mHelp\x1b[m\n\
        \x1b[31m\t-help\x1b[m               Display help\n"
    );
}

fn parse_args(args: &[String]) -> Result<(Operator, Operation), Error> {
    if args.len() < 2 {
        return Ok((Operator::Help, Operation::Get));
    }

    match args[1].as_str() {
        "-help" | "-h" => Ok((Operator::Help, Operation::Get)),
        "-fan" | "-f" => parse_fan_args(&args[2]),
        "-thermal" | "-t" => parse_thermal_args(&args[2]),
        "-led" | "-l" => parse_led_args(&args[2]),
        _ => Err(Error::InvalidArgv),
    }
}

fn parse_fan_args(arg: &str) -> Result<(Operator, Operation), Error> {
    match arg {
        "silent" | "s" => Ok((Operator::Fan, Operation::Silent)),
        "balanced" | "b" => Ok((Operator::Fan, Operation::Balanced)),
        "turbo" | "t" => Ok((Operator::Fan, Operation::Turbo)),
        "get" | "g" => Ok((Operator::Fan, Operation::Get)),
        _ => Err(Error::InvalidArgv),
    }
}

fn parse_thermal_args(arg: &str) -> Result<(Operator, Operation), Error> {
    match arg {
        "silent" | "s" => Ok((Operator::Thermal, Operation::Silent)),
        "default" | "df" => Ok((Operator::Thermal, Operation::Default)),
        "overboost" | "ob" => Ok((Operator::Thermal, Operation::Overboost)),
        "get" | "g" => Ok((Operator::Thermal, Operation::Get)),
        _ => Err(Error::InvalidArgv),
    }
}

fn parse_led_args(arg: &str) -> Result<(Operator, Operation), Error> {
    match arg {
        "off" => Ok((Operator::Led, Operation::Off)),
        "min" => Ok((Operator::Led, Operation::Min)),
        "med" => Ok((Operator::Led, Operation::Med)),
        "max" => Ok((Operator::Led, Operation::Max)),
        "get" | "g" => Ok((Operator::Led, Operation::Get)),
        _ => Err(Error::InvalidArgv),
    }
}

fn identify_file(operator: &Operator, operation: &Operation) -> Result<FileList, Error> {
    match operator {
        Operator::Led => check_file_access(FileList::LedFile, operation),
        Operator::Fan => {
            if file_exists(&FileList::AsusFanPolicy, operation) {
                Ok(FileList::AsusFanPolicy)
            } else if file_exists(&FileList::FstsFanPolicy, operation) {
                Ok(FileList::FstsFanPolicy)
            } else {
                Err(Error::NoPermission)
            }
        }
        Operator::Thermal => {
            if file_exists(&FileList::AsusThermalPolicy, operation) {
                Ok(FileList::AsusThermalPolicy)
            } else if file_exists(&FileList::FstsThermalPolicy, operation) {
                Ok(FileList::FstsThermalPolicy)
            } else {
                Err(Error::NoPermission)
            }
        }
        Operator::Help => Err(Error::InvalidArgFun),
    }
}

fn file_exists(file: &FileList, operation: &Operation) -> bool {
    let path = std::path::Path::new(file.path());
    if matches!(operation, Operation::Get) {
        path.exists()
            && path
                .metadata()
                .map(|m| m.permissions().readonly())
                .unwrap_or(true)
    } else {
        path.exists()
            && path
                .metadata()
                .map(|m| !m.permissions().readonly())
                .unwrap_or(false)
    }
}

fn check_file_access(file: FileList, operation: &Operation) -> Result<FileList, Error> {
    if file_exists(&file, operation) {
        Ok(file)
    } else {
        Err(Error::NoPermission)
    }
}

fn write_to_file(
    file_path: &str,
    value: char,
    operator: &Operator,
    operation: &Operation,
) -> Result<(), Error> {
    let mut file = File::create(file_path).map_err(|_| Error::BadFp)?;
    file.write_all(&[value as u8])
        .map_err(|_| Error::FileWriteError)?;

    let msg = format!(
        "Set {} policy to {:?}",
        match operator {
            Operator::Led => "led",
            Operator::Fan => "fan",
            Operator::Thermal => "thermal",
            Operator::Help => "help",
        },
        operation
    );
    println!("Perfmode: {}", msg);
    Ok(())
}

fn read_from_file(file_path: &str, operator: &Operator) -> Result<(), Error> {
    let mut file = File::open(file_path).map_err(|_| Error::BadFp)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| Error::BadFp)?;

    let value = match operator {
        Operator::Led => match contents.trim() {
            "0" => "off",
            "1" => "min",
            "2" => "med",
            "3" => "max",
            _ => return Err(Error::Unknown),
        },
        Operator::Fan => match contents.trim() {
            "0" => "balanced",
            "1" => "turbo",
            "2" => "silent",
            _ => return Err(Error::Unknown),
        },
        Operator::Thermal => match contents.trim() {
            "0" => "default",
            "1" => "overboost",
            "2" => "silent",
            _ => return Err(Error::Unknown),
        },
        Operator::Help => return Err(Error::InvalidArgFun),
    };

    println!("{}", value);
    Ok(())
}

fn perform_operation(
    file_list: FileList,
    operator: &Operator,
    operation: &Operation,
) -> Result<(), Error> {
    match operation {
        Operation::Get => read_from_file(file_list.path(), operator),
        _ => {
            let value =
                match (operator, operation) {
                    (Operator::Led, Operation::Off) => '0',
                    (Operator::Led, Operation::Min) => '1',
                    (Operator::Led, Operation::Med) => '2',
                    (Operator::Led, Operation::Max) => '3',
                    (Operator::Fan | Operator::Thermal, Operation::Silent) => '2',
                    (Operator::Fan, Operation::Balanced)
                    | (Operator::Thermal, Operation::Default) => '0',
                    (Operator::Fan, Operation::Turbo)
                    | (Operator::Thermal, Operation::Overboost) => '1',
                    _ => return Err(Error::InvalidArgFun),
                };
            write_to_file(file_list.path(), value, operator, operation)
        }
    }
}

//TODO: use logger & write logs to dot folder date wise
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match parse_args(&args) {
        Ok((Operator::Help, _)) => {
            print_help();
            exit(1);
        }
        Ok((operator, operation)) => match identify_file(&operator, &operation) {
            Ok(file) => {
                if let Err(e) = perform_operation(file, &operator, &operation) {
                    eprintln!("Perfmode: {}", e);
                    exit(1);
                }
            }
            Err(e) => {
                eprintln!("Perfmode: {}", e);
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("Perfmode: {}", e);
            exit(1);
        }
    }
}

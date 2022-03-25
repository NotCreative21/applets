use sysinfo::{System, SystemExt, ProcessorExt};
use std::process::Command;
use chrono::{DateTime, Local};

#[cfg(feature = "battery")]
mod battery {
    use std::fs;

    const BATTERY: &str = "/sys/class/power_supply/BAT0/";
    #[inline(always)]
    pub fn charge() -> String {
        let current: usize = match fs::read_to_string(format!("{BATTERY}charge_now")) {
            Ok(v) => v.trim().parse().unwrap_or(1),
            Err(_) => 1
        };
        let total: usize = match fs::read_to_string(format!("{BATTERY}charge_full")) {
            Ok(v) => v.trim().parse().unwrap_or(1),
            Err(_) => 1 
        };
        format!("{:.2}%", (current as f32 / total as f32) * 100.0)
    }
}

#[inline(always)]
fn ram(sys: &System) -> (u64, u64) {
    (sys.used_memory(), sys.total_memory())
}

#[inline(always)]
fn cpu(sys: &System, cores: usize) -> (f32, bool) {
    let mut usage = 0.0;
    sys.processors().into_iter().for_each(|x| usage += x.cpu_usage());
    let usage  = usage / cores as f32;
    (usage, usage < 10.0)
}

#[inline(always)]
fn time() -> String {
   (&DateTime::to_rfc2822(&Local::now())[..22]).to_string() 
}

fn bar() {
    let mut sys = System::new_all();
    let core_count = sys.processors().into_iter().count();
    sys.refresh_cpu();
    sys.refresh_memory();
    let (u, t) = ram(&sys);
    let ram = format!("{}M/{:.2}%", u / 1024, (u as f32 / t as f32) * 100.0);
    let vol = {
        match Command::new("pamixer")
            .arg("--get-volume-human")
            .output() {
                Ok(v) => {
                    let output = String::from_utf8(v.stdout).unwrap();
                    match output == String::from("muted\n") {
                        true => "^ muted".to_string(),
                        false => format!("^ {}%", output.trim())
                    }
                },
                Err(_) => "*".into()
            }
    };
    // so that we always have the same length output
    let cpu = {
        let (percent, add_space) = cpu(&sys, core_count);
        match add_space {
            true => format!("0{:.2}%", percent),
            false => format!("{:.2}%", percent)
        }
    };
    #[cfg(feature = "battery")]
    println!("⌨ {cpu} | {ram} | {vol} | {} | {}", battery::charge(), time());
    #[cfg(not(feature = "battery"))]
    println!("⌨ {cpu} | {ram} | {vol} | {}", time());
}

fn main() {
    bar();
}

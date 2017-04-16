extern crate regex;
extern crate toml;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::env;
use std::path::Path;
use std::fs;
use std::collections::BTreeMap;
use regex::Regex;
use std::time::{self, SystemTime};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::process::Command;

#[derive(Deserialize)]
struct Subject {
    directory: String,
    results_path: String,
    name: String,
    colour: String
}
#[derive(Deserialize)]
struct Config {
    subjects: Vec<Subject>
}
static GNUPLOT_SCRIPT: &'static str = include_str!("./script.gnuplot");

type MarkData = BTreeMap<SystemTime, (String, f32)>;
fn analyse_dir(re: &Regex, path: &Path) -> io::Result<MarkData> {
    let mut map = BTreeMap::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if !meta.is_dir() {
            let name = entry.file_name().into_string()
                .unwrap();
            let ctime = meta.modified()?;
            if let Some(caps) = re.captures(&name) {
                if let Some(perc) = caps.name("percent") {
                    if let Ok(perc) = perc.as_str().parse::<f32>() {
                        map.insert(ctime, (name.clone(), perc));
                    }
                }
            }
        }
    }
    Ok(map)
}
fn get_mean(map: &MarkData) -> f32 {
    let mut tot = 0.0;
    let mut j = 1.0;
    for (i, (_, &(_, perc))) in map.iter().enumerate() {
        j = (i as f32) + 1.0;
        tot += perc;
    }
    tot / j
}
fn write_results<W: io::Write>(map: &MarkData, file: &mut W) {
    let mean = get_mean(&map);
    for (i, (ctime, &(ref name, perc))) in map.iter().enumerate() {
        let dur_since_epoch = ctime.duration_since(time::UNIX_EPOCH).unwrap();
        writeln!(file, "{},{},\"{}\",{},{}", i, dur_since_epoch.as_secs(), name.replace("\"", "'"), perc, mean)
            .unwrap();
    }
}
fn main() {
    println!("The Markifier, an eta project");
    let re = Regex::new(r".*\[(?P<percent>.+)%.*\].*").unwrap();
    println!("[+] Reading arguments");
    let mut args = env::args();
    let path = args.nth(1)
        .expect("Please provide a path to the config file as the first argument.");
    println!("[+] Reading configuration file {}...", path);
    let mut config = File::open(&path).unwrap();
    let mut contents = String::new();
    config.read_to_string(&mut contents).unwrap();
    let conf: Config = toml::from_str(&contents).unwrap();
    for subj in conf.subjects {
        println!("[+] Reading directory {}...", subj.directory);
        let data = match analyse_dir(&re, Path::new(&subj.directory)) {
            Ok(d) => d,
            Err(e) => {
                println!("[-] Error: {}", e);
                continue;
            }
        };
        println!("[+] Writing results to {}...", subj.results_path);
        let mut file = match File::create(&subj.results_path) {
            Ok(f) => f,
            Err(e) => {
                println!("[-] Error creating file: {}", e);
                continue;
            }
        };
        write_results(&data, &mut file);
        println!("[+] Writing gnuplot script...");
        let conf = GNUPLOT_SCRIPT
            .replace("OUTPUTFILEPATH", &subj.results_path.replace(".csv", ".png"))
            .replace("INPUTFILEPATH", &subj.results_path)
            .replace("COLOUR", &subj.colour)
            .replace("SUBJNAME", &subj.name)
            .replace("MEAN", &get_mean(&data).to_string());
        let mut gfile = match File::create(&subj.results_path.replace(".csv", ".gnuplot")) {
            Ok(f) => f,
            Err(e) => {
                println!("[-] Error creating file: {}", e);
                continue;
            }
        };
        gfile.write_all(conf.as_bytes()).unwrap();
        println!("[+] Running gnuplot...");
        let code = Command::new("gnuplot")
            .arg("-c")
            .arg(&subj.results_path.replace(".csv", ".gnuplot"))
            .status();
        match code {
            Ok(s) => println!("[+] Exited with: {:?}", s),
            Err(e) => println!("[-] Error running: {}", e)
        }
    }
    println!("[+] Done!");
}

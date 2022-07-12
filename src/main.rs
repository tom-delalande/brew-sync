use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::str;
use toml::value::Table;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: String = if args.len() > 1 {
        args[1].to_string()
    } else {
        "brew.toml".to_string()
    };
    let mut file = File::open(filename).expect("Error opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error opening file");

    let config: Config = toml::from_str(&contents).unwrap();
    let desired_formulae: Vec<String> = config
        .formulae
        .to_owned()
        .iter()
        .map(|e| e.to_owned().0.to_owned())
        .collect();

    let formulae = sync(desired_formulae, "formulae".to_string());
    sync_formulae(formulae.to_install, formulae.to_delete);

    let desired_casks: Vec<String> = config
        .casks
        .to_owned()
        .iter()
        .map(|e| e.to_owned().0.to_owned())
        .collect();
    let casks = sync(desired_casks, "cask".to_string());
    sync_casks(casks.to_install, casks.to_delete);
}

fn sync_formulae(to_install: Vec<String>, to_delete: Vec<String>) {
    Command::new("brew").arg("install").args(to_install.clone()).output().expect("Error installing formulae");
    Command::new("brew").arg("uninstall").args(to_delete.clone()).output().expect("Error uninstalling formulae");
    println!("Installing formulae: {:?}", to_install);
    println!("Deleting formulae: {:?}", to_delete);
}

fn sync_casks(to_install: Vec<String>, to_delete: Vec<String>) {
    Command::new("brew").arg("cask").arg("install").args(to_install.clone()).output().expect("Error installing casks");
    Command::new("brew").arg("cask").arg("uninstall").args(to_delete.clone()).output().expect("Error installing casks");
    println!("Installing casks: {:?}", to_install);
    println!("Deleting casks: {:?}", to_delete);
}

struct SyncResult {
    to_install: Vec<String>,
    to_delete: Vec<String>,
}

fn sync(desired: Vec<String>, filter: String) -> SyncResult {
    let result = Command::new("brew")
        .arg("list")
        .arg(format!("--{}", filter))
        .output()
        .unwrap()
        .to_owned();

    let installed: Vec<String> = str::from_utf8(&result.stdout)
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();

    let mut desired_deps: Vec<String> = desired
        .iter()
        .map(|f| {
            let deps = Command::new("brew")
                .arg("deps")
                .arg(f)
                .output()
                .unwrap()
                .to_owned();
            let str = str::from_utf8(&deps.stdout).unwrap();
            let iter: Vec<String> = str.split_whitespace().map(|s| s.to_owned()).collect();
            iter
        })
        .flatten()
        .collect();

    let mut desired_with_deps = desired.to_vec();
    desired_with_deps.append(&mut desired_deps);
    let to_install: Vec<String> = desired
        .iter()
        .filter(|f| !installed.contains(f))
        .map(|f| f.to_owned())
        .collect();
    let to_delete: Vec<String> = installed
        .iter()
        .filter(|f| !desired_with_deps.contains(f))
        .map(|f| f.to_owned())
        .collect();

    return SyncResult {
        to_install,
        to_delete,
    }
}

#[derive(Deserialize)]
struct Config {
    formulae: Table,
    casks: Table,
}

use std::{env, fs, path::PathBuf, process::{exit, Command}};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args[1] != "build" {
        eprintln!("usage: gdrust build <level.rs> [-o out.gmd]");
        exit(1);
    }
    let script = fs::canonicalize(&args[2]).expect("script not found");
    let out = args.iter().position(|a| a == "-o").map(|i| args[i + 1].clone()).unwrap_or("out.gmd".into());

    // scripts are normal rust, so we wrap them in a throwaway cargo project
    let dir: PathBuf = env::temp_dir().join("gdrust_build");
    fs::create_dir_all(dir.join("src")).unwrap();
    let dep = match env::var("GDRUST_PATH") {
        Ok(p) => format!("gd_rust_core = {{ path = \"{p}\" }}"),
        Err(_) => "gd_rust_core = \"0.1\"".into(),
    };
    fs::write(dir.join("Cargo.toml"), format!(
        "[package]\nname=\"lvl\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[dependencies]\n{dep}\n[[bin]]\nname=\"lvl\"\npath=\"{}\"\n",
        script.display()
    )).unwrap();

    let ok = Command::new("cargo").args(["run", "--release", "-q"]).current_dir(&dir)
        .env("GDRUST_OUT", &out).status().map(|s| s.success()).unwrap_or(false);
    if !ok { exit(1); }
    println!("wrote {out}");
}

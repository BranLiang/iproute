use std::env;
use std::process::Command;
use std::io;
use std::io::Write;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let output = Command::new("route")
            .args(args)
            .output()
            .expect("failed to run route command");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

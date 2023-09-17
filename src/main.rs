use std::env;
use std::fs::File;
use std::io::Write;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <PID>", args[0]);
        exit(1);
    }

    let pid = &args[1];

    let exe_path_buf = env::current_exe().expect("Can't get exe path");

    println!("{}", exe_path_buf.display());

    let output = Command::new("ps")
        .arg("-p")
        .arg(pid)
        .arg("-o")
        .arg("comm=")
        .output()
        .expect("Error cannot exec ps command");

    if let Ok(app_name) = String::from_utf8(output.stdout) {
        let app_name = app_name.trim();
        let (_, name) = app_name.rsplit_once('/').unwrap();
        let log_filename = format!("{}.log", name);
        let exe_directory = exe_path_buf.parent().expect("Can't get exe path");
        let log_path = exe_directory.join(&log_filename);
        let mut log_file = File::create(&log_path).expect("Can't create log file");

        let lsof_output = Command::new("lsof")
            .arg("-p")
            .arg(pid)
            .arg("-Fn")
            .output()
            .expect("Error cannot exec lsof command");

        if let Ok(lsof_output_str) = String::from_utf8(lsof_output.stdout) {
            let lines: Vec<&str> = lsof_output_str.lines().collect();

            for line in lines {
                if line.starts_with('n') {
                    let library_name = &line[1..];
                    writeln!(log_file, "{}", library_name).expect("Error can't write in log file");
                }
            }

            println!("Save path : {}", log_path.display());
        } else {
            eprintln!("Can't read lsof command");
        }
    } else {
        eprintln!("Can't read ps command");
    }
}

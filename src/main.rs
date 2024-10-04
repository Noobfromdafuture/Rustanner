use std::net::SocketAddr;
use tokio::net::TcpStream; // Using Tokio's async TCP connection
use tokio::time::{timeout, Duration}; // For setting timeout on connections
use std::env; // To parse command line arguments

/// Asynchronously scans a single port
async fn scan_port(ip: String, port: u16) -> bool {
    let addr = format!("{}:{}", ip, port); // Constructing the ip:port format
    let socket_addr: SocketAddr = addr.parse().expect("Unable to parse socket address");

    // Timeout of 3 seconds for connection attempt
    if timeout(Duration::from_secs(3), TcpStream::connect(&socket_addr)).await.is_ok() {
        println!("Port {} is open on {}", port, socket_addr); // Port is open
        true
    } else {
        false
    }
}

/// The main function will handle IP and port range input and start scanning
#[tokio::main]
async fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("This is RUSTANNER, your port scanner written in Rust.");
        println!("You will be able to see opened ports on the IP you specified.");
        println!("Here is how to use it:");
        println!("Usage: {} <IP> <start_port> <end_port>", args[0]);
        println!("Example: {} 192.168.1.1 80 100", args[0]);
        println!("Note: I am not responsible for any illegal use of this program. RUSTANNER was written just for educational purposes.");
        eprintln!("Something went wrong. Please try again. Usage: {} <IP> <start_port> <end_port>", args[0]);
        return;
    }

    // Clone the IP address to avoid borrowing issues
    let ip = args[1].clone();
    let start_port: u16 = args[2].parse().expect("Invalid start port.");
    let end_port: u16 = args[3].parse().expect("Invalid end port.");

    println!("Scanning IP: {} on ports: {}-{}", ip, start_port, end_port);

    // Scan ports concurrently using Tokio's task management
    let mut tasks: Vec<_> = vec![];

    for port in start_port..=end_port {
        // Clone the IP address for each task to avoid borrowing issues
        let ip_clone = ip.clone();
        // Spawn a task for each port scan
        tasks.push(tokio::spawn(async move {
            scan_port(ip_clone, port).await;
        }));
    }

    // Await all tasks to complete
    for task in tasks {
        let _ = task.await;
    }

    println!("Port scan completed.");
}

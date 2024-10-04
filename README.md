# Rustanner

Rustanner is a port scanner written in Rust, using the Tokio library to perform asynchronous network operations. It scans a range of ports on a given IP address and checks which ports are open.

## Prerequisites

Before running the program, ensure you have **Rust** installed on your system. If you haven't installed Rust yet, follow the installation instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Compilation

To compile the project, follow these steps:

1. Clone the repository (if applicable):
   ```bash
   git clone <repository-url>
   ```

2. Navigate into the project directory:
   ```bash
   cd rustanner
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

   This will create an executable in the `target/release` directory.

## Usage

After compiling the program, you can run the executable. The usage is as follows:

```bash
./target/release/rustanner <IP> <start_port> <end_port>
```

For example, to scan ports 20 to 80 on the IP `192.168.1.1`:

```bash
./target/release/rustanner 192.168.1.1 20 80
```

When you run the program without arguments or with incorrect arguments, the following help message will be displayed:

```
This is RUSTANNER, your port scanner written in Rust.
You will be able to see opened ports on the IP you specified.
Here is how to use it:
./rustanner <IP> <start_port> <end_port>
Example: ./rustanner 192.168.1.1 20 80

(I am not responsible for any illegal use of this program; it was written for educational purposes only.)
```

## License

This project is licensed under the MIT License.

---

use std::net::{TcpStream};
use std::str;
use std::io::{BufRead, BufReader, Write};
use std::time::Duration;

#[cfg(target_os = "wasi")]
use std::os::wasi::io::{RawFd, AsRawFd, FromRawFd};

#[no_mangle]
#[cfg(target_os = "wasi")]
fn do_start(fd: RawFd) {
  let stream: TcpStream;
  unsafe {
    stream = TcpStream::from_raw_fd(fd);
  }

  disp(stream);
}

#[no_mangle]
fn disp(mut stream: TcpStream) {
  stream.write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
  stream.flush().unwrap();

  let mut reader = BufReader::new(&stream);
  let mut buffer = Vec::new();
  reader.read_until(b'\n', &mut buffer).expect("failed to read from socket");
  print!("{}", str::from_utf8(&buffer).expect("failed to convert to String"));
}

#[cfg(target_os = "wasi")]
fn main() {
    let stream = TcpStream::connect("www.google.com:80").unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();

    let fd = stream.as_raw_fd();

    do_start(fd);
}

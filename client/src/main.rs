use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    //建立tcp连接
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10 {
        let mut input = String::new();
        // 键盘输入
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        // 输入流
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");
        
        // 读取echo返回缓冲区
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        println!("{}", 
            str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("");
    }
    Ok(())
}
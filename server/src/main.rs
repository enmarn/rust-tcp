use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::str;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512];
    loop {
        //读取缓冲区
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        //转换为utf-8字符串
        let msg = str::from_utf8(&buf).expect("Could not write buffer as string");
        let first = &msg[0..4];
        
        println!("接收消息-- {}", msg);
        match first.as_ref() {
            //模式匹配 exit 结束服务
            "exit" => {
                println!("结束服务");
                break
            },
            //模式匹配其他,echo返回
            _ => {
                stream.write(&buf[..bytes_read])?;
                thread::sleep(time::Duration::from_secs(1 as u64));
            }
        }

    }
    return Ok(());
}

fn main() -> std::io::Result<()> {
    //绑定地址端口
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Tcp Server Start ...");
    println!("Listening 127.0.0.1:8080");
    //线程池
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    //监听Tcp
    for stream in listener.incoming() {
        let stream = stream.expect("failed!");
        //启动新线程
        let handle = thread::spawn(move || {
            handle_client(stream)
		.unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        thread_vec.push(handle);
    }
    //关闭线程池
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
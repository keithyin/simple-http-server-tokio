use std::fs;
use tokio::{self, net};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;


async fn process(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        if n == 0 {
            break;
        }
        let s = String::from_utf8_lossy(&buf[0..n]).to_string();
        print!("{}", s);
        if s.ends_with("\r\n\r\n") {
            println!("request received");
            break;
        }
    }
    let response_header = "HTTP/1.1 200 OK";
    // http 响应的 status line 和 entity body 之间要有 \r\n\r\n 。少了的话，浏览器不认
    let response_content = fs::read_to_string("index.html").unwrap();
    let response_str = format!("{}\n\n{}", response_header, response_content);

    println!("response:\n{}", response_str);
    // browser doesn't receive the response that this line of code send.
    stream.write(response_str.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
    println!("response DONE");

}

#[tokio::main]
async fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:9998").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move{
            process(stream).await;
        });
    }
}

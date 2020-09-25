use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listner = TcpListener::bind("127.0.0.1:37564").await?;

    loop {
        let (mut socket, _) = listner.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    // Socket closed
                    Ok(n) if n < 2 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                eprintln!("read buffer size; n = {:?}", n);

                // Command analyze 1st
                let commandx: &[u8] = &[buf[0]];
                let command = std::str::from_utf8(commandx).unwrap();

                eprintln!("command id is ; id = {:?}", command);

                // Command analyze 2nd
                let commandx2: &[u8] = &[buf[1]];
                let command2 = std::str::from_utf8(commandx2).unwrap();

                eprintln!("command2 id is ; id = {:?}", command2);

                // Default response values set (echo values)
                let mut data: &[u8] = &buf;
                let mut size: usize = n;

                // Command exec
                if command == String::from('0') {
                    let mut response: &str = "0-hello world!\r\n";

                    if command2 == String::from('0') {
                        response = "00orega gundam da!\r\n";
                    }

                    size = response.chars().count();
                    data = response.as_bytes();
                }

                // Write the data back
                if let Err(e) = socket.write_all(&data[0..size]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

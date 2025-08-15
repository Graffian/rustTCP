use std::{io::{self, Write}};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};


#[tokio::main]
async fn main(){
    let stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();
    let mut user_input = String::new();
    let (mut reader , mut writer) = stream.into_split();
    tokio::spawn(async move{
        loop{
            let mut buf = vec![0;512];
            let n = reader.read(&mut buf).await.unwrap();
            if n==0{
                println!("SERVER CLOSED CONNECTION");
            }
            println!("BROADCAST MESSAGE FROM SERVER: {}" , String::from_utf8_lossy(&buf[..n]));
        }

    });
    loop{
        user_input.clear(); 
        print!(" >>>>>>>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        if user_input.trim() == "QUIT"{
            break;
        }else{
            writer.write_all(user_input.trim().as_bytes()).await.unwrap();
        }
    }
}
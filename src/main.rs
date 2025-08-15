use std::{collections::HashMap, sync::Arc};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{tcp::{OwnedWriteHalf}, TcpListener}, sync::{mpsc::{self}, Mutex}};
#[tokio::main]
async fn main() {
    let (tx , mut rx) = mpsc::channel::<(String , String)>(100);
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    println!("SERVER RUNNING ON 127.0.0.1:9000");
    let clients: Arc<Mutex<HashMap<String , OwnedWriteHalf>>> = Arc::new(Mutex::new(HashMap::new()));
    let clients_clone = clients.clone();

    tokio::spawn(async move{
        while let Some((sender_id , msg)) = rx.recv().await{
            let mut map = clients_clone.lock().await;
            let full_msg = format!("[{}]:{}\n" , sender_id , msg);
            let mut disconnected = vec![];
            for (id,writer) in map.iter_mut(){
                if writer.write_all(full_msg.as_bytes()).await.is_err(){
                    disconnected.push(id.clone());
                }
            }
            for id in disconnected{
                map.remove(&id);
                println!("CLIENT {} DISCONNECTED" , id);
            }
        }
    });
    let mut client_count = 0;
    loop{
        let (socket , _addr) = listener.accept().await.unwrap();
        client_count+=1;
        let client_id = format!("CLIENT{}" , client_count);
        let tx_clone = tx.clone();
        let mut writers_lock = clients.lock().await;
        let (reader,writer) = socket.into_split();
        writers_lock.insert(client_id.clone(), writer);
        drop(writers_lock);

        tokio::spawn(async move{
            let mut reader = reader;
            let mut buf = vec![0;1024];
            loop{
                let n = reader.read(&mut buf).await.unwrap();
                if n==0{
                    println!("NO MESSAGE");
                }else{
                    let msg = String::from_utf8_lossy(&buf[..n]).to_string();
                    if tx_clone.send((client_id.clone() , msg)).await.is_err(){
                        break
                    }
                }
            }
        });
    }
}

use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct API {
    // logger
    // handler: Arc<dyn HandlerFn + Sync>,
    addr: String,
}

trait HandlerFn: Send + Sync {
    fn handle(&self, input: Vec<u8>) -> Vec<u8>;
}

impl API {
    pub fn new(host: String) -> API {
        API {
            addr: format!("0.0.0.0:{}", host),
            // handler: Arc::new(handler),
        }
    }
    pub async fn run<'a, F>(self, op: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Vec<u8>) -> Vec<u8> + Sync + Send + 'static,
    {
        let listener = TcpListener::bind(&self.addr).await?;
        // use logger here
        println!("Listening on: {}", self.addr);
        let op = Arc::new(op);

        loop {
            let op = op.clone();
            let (mut socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                let mut input = vec![0; 1024];
                loop {
                    let n = socket.read(&mut input).await.expect("unhandled: read");

                    if n == 0 {
                        return;
                    }

                    let result_buff = op(input.clone());

                    socket
                        .write_all(result_buff.as_slice())
                        .await
                        .expect("unhandled: failed to write data to socket");
                }
            });
        }
    }
}

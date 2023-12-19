use std::io::ErrorKind;
use tokio::net::unix::pipe;
use tokio::sync::mpsc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        log::error!("Usage: program_name fifo_file_in fifo_file_out");

        return Err(io::Error::new(
            ErrorKind::Other,
            format!("Wrong program parameters: {}, expected 2", args.len() - 1)
        ));
    }

    let input_fifo = args[1].clone();
    let output_fifo = args[2].clone();

    let rx_test = pipe::OpenOptions::new()
        .read_write(true)
        .open_receiver(input_fifo.clone());

    let tx_test = pipe::OpenOptions::new()
        .read_write(true)
        .open_sender(output_fifo.clone());

    if let (Ok(mut rx), Ok(mut tx)) = (rx_test, tx_test) {
        let (channel_tx, mut channel_rx) = mpsc::channel::<Vec<u8>>(1);

        tokio::spawn(async move {
            println!("Spawn Read Thread");

            loop {
                match channel_rx.recv().await {
                    Some(msg) => {
                        let _ = tx.write(&msg).await;
                    },
                    None => println!("the sender dropped"),
                }
            }
        });

        let read_task = tokio::spawn(async move {
            loop {
                let mut msg = vec![0; 256];

                if let Ok(readed) = rx.read(&mut msg).await {
                    let msg_readed_vec = msg[..readed].to_vec();
                    let _ = channel_tx.try_send(msg_readed_vec);
                }
            }
        });

        loop {
            if let Ok(exists) = tokio::fs::try_exists(input_fifo.clone()).await {
                if !exists {
                    read_task.abort();
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    } else {
        log::error!("Error opening read or tunnel file");

        return Err(io::Error::new(
            ErrorKind::Other,
            format!("Can't open {input_fifo} or {output_fifo}")
        ));
    }
}
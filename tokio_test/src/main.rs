use tokio::sync::mpsc;
use tokio::task;

#[derive(Debug)]
enum EventId {
    GetAddress,
    GetStatus,
}

#[derive(Debug)]
struct EventStruct {
    eid: EventId,
}

async fn send_data(tx: mpsc::Sender<EventStruct>, data: EventStruct) {
    match tx.send(data).await {
        Ok(_) => println!("Successfully sent"),
        Err(e) => eprintln!("Failed to send: {}", e),
    }
}

async fn receive_data(mut rx: mpsc::Receiver<EventStruct>) {
    while let Some(i) = rx.recv().await {
        println!("received EventID: {:?}", i);
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);

    for i in 0..5 {
        let tx = tx.clone();
        let event = EventStruct {
            eid: if i & 2 == 0 { EventId::GetStatus } else { EventId::GetAddress },
        };
        task::spawn(send_data(tx, event));
    }

    match task::spawn(receive_data(rx)).await {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to spawn task: {}", e),
    }
}

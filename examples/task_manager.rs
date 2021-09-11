use std::time::Duration;

use bytes::Bytes;
use mini_redis::client;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

/// Multiple different commands are multiplexed over a single channel.
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Vec<u8>,
        resp: Responder<()>,
    },
}

/// Provided by the requester and used by the manager task to send the command
/// response back to the requester.
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

fn create_channel<T>(buffer: usize) -> (mpsc::Sender<T>, mpsc::Sender<T>, mpsc::Receiver<T>) {
    let (tx, rx) = mpsc::channel(buffer);
    (tx.clone(), tx.clone(), rx)
}

fn create_record() -> (String, String) {
    let key = uuid::Uuid::new_v4().to_string();
    let value = uuid::Uuid::new_v4().to_string();
    (key, value)
}

fn spawn_manager(mut rx: mpsc::Receiver<Command>) -> JoinHandle<()> {
    tokio::spawn(async move {
        // Open a connection to the mini-redis address.
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val.into()).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
            }
        }
    })
}

fn spawn_get_task(record: (String, String), tx: mpsc::Sender<Command>) -> JoinHandle<()> {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(rand::random::<u64>() % 100)).await;
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: record.0,
            resp: resp_tx,
        };

        // Send the GET request
        if tx.send(cmd).await.is_err() {
            println!("connection task shutdown");
            return;
        }

        // Await the response
        let res = resp_rx.await;
        println!("GOT (Get) = {:?}", res);
    })
}

fn spawn_set_task(record: (String, String), tx: mpsc::Sender<Command>) -> JoinHandle<()> {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(rand::random::<u64>() % 100)).await;
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: record.0,
            val: record.1.into_bytes(),
            resp: resp_tx,
        };

        // Send the SET request
        if tx.send(cmd).await.is_err() {
            println!("connection task shutdown");
            return;
        }

        // Await the response
        let res = resp_rx.await;
        println!("GOT (Set) = {:?}", res);
    })
}

async fn join_task(task: JoinHandle<()>, id: &str) {
    task.await.unwrap();
    println!("finished task: {:?}", id);
}

#[tokio::main]
async fn main() {
    let (get_tx, set_tx, manager_rx) = create_channel(32);
    let manager_task = spawn_manager(manager_rx);

    let mut tasks = vec![];
    for _ in 1..=100 {
        let (key, value) = create_record();
        // Spawn two tasks, one setting a value and other querying for key that was set.
        tasks.push((
            spawn_get_task((key.clone(), value.clone()), get_tx.clone()),
            "get_task",
        ));
        tasks.push((
            spawn_set_task((key.clone(), value.clone()), get_tx.clone()),
            "set_task",
        ));
    }

    // If we don't drop these senders, manager task receiver
    // will continuosly sleep and retry to read a value from the channel.
    // The channel only closes when either close() is called or all senders
    // were dropped.
    drop(get_tx);
    drop(set_tx);

    // Await for all takio spawned tasks.
    // Tokio task spawns return a handlers that can be used
    // to pass values/handle errors.
    // In this case, handlers only hold the () value,
    // as we are not really intrested in handling the value.
    for (jh, id) in tasks {
        join_task(jh, id).await
    }
    join_task(manager_task, "manager_task").await;
}

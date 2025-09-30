use std::collections::HashMap;
use tokio::sync::mpsc::{Sender as MPSCSender, Receiver as MPSCReceiver};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use jmphash::jump_hash;
use tokio::sync::mpsc;
use std::sync::Arc;
// use tokio::sync::oneshot::{Sender, Receiver, channel};
use oneshot::{Sender, Receiver, channel};

enum Commands {
    SetIsActive {
        client_id: String,
        is_active: bool,
    },
    GetIsActive {
        client_id: String,
        sender: Sender<bool>
    },
    AddClient {
        client_id: String,
    }
}

struct Client {
    is_active: bool,
}

struct Gateway {
    clients: HashMap<String, Client>
}

struct GatewayService {
    clients: Vec<MPSCSender<Commands>>
}

impl GatewayService {

    fn get_bucket(&self, client_id: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        client_id.hash(&mut hasher);
        let final_hash = hasher.finish();

        jump_hash(final_hash, self.clients.len() as i64) as usize
    }

    async fn add_client(&self, client_id: Arc<str>) {
        let client = self.get_bucket(&client_id);
        println!("client: {client}");
        let x = self.clients[client].clone();
        if let Err(e) = x.send(Commands::AddClient { client_id: client_id.to_string() }).await {
            println!("SendError: {e}");
        }
    }

    async fn set_is_active(&self, client_id: Arc<str>, is_active: bool) {
        let client = self.get_bucket(&client_id);
        println!("client: {client}");
        let x = self.clients[client].clone();
        if let Err(e) = x.send(Commands::SetIsActive { client_id: client_id.to_string(), is_active }).await {
            println!("SendError: {e}");
        }
    }

    async fn get_is_active(&self, client_id: Arc<str>) -> bool {
        let (sender, receiver) = channel::<bool>();

        let client = self.get_bucket(&client_id);
        let client_sender = self.clients[client].clone();
        match client_sender.send(Commands::GetIsActive { client_id: client_id.to_string(), sender }).await {
            Ok(_) => {
                match receiver.await {
                    Ok(r) => r,
                    Err(e) => {
                        println!("TryRecvError: {:?}", e);
                        false
                    },
                }
            },
            Err(e) => {
                println!("Send Error: {:?}", e);
                false
            },
        }
    }

}

impl Gateway {
    fn new() -> Self {
        Self { clients: HashMap::default() }
    }

    fn add_client(&mut self, client_id: String) {
        self.clients.insert(client_id, Client { is_active: false });
    }

    fn add_is_active(&mut self, client_id: String, is_active: bool) {
        println!("Add is active, {is_active}");
        if let Some(client) = self.clients.get_mut(&client_id) {
            client.is_active = is_active;
        }
    }

    fn get_is_active(&mut self, client_id: String, sender: Sender<bool>) {
        if let Some(client) = self.clients.get_mut(&client_id) {
            if let Err(e) = sender.send(client.is_active) {
                println!("Error");
            }
        } else {
            if let Err(e) = sender.send(false) {
                println!("Error");
                
            }
        }
    }
}

async fn event_loop (mut gateway: Gateway, mut rx: MPSCReceiver<Commands>) {
    loop {
        if let Some(command) = rx.recv().await {
            match command {
                Commands::SetIsActive { client_id, is_active } => gateway.add_is_active(client_id, is_active),
                Commands::GetIsActive { client_id, sender } => {
                    gateway.get_is_active(client_id, sender);
                },
                Commands::AddClient { client_id } => {
                    gateway.add_client(client_id);
                }
            }
        }

    }
}

pub async fn run() {
    let num_buckets = 4;
    let mut service = GatewayService { clients: Vec::with_capacity(num_buckets) };


    for _ in 0..num_buckets {
    let (tx, rx) = mpsc::channel::<Commands>(1024); // bounded channel for backpressure
        service.clients.push(tx);
        tokio::spawn(event_loop(Gateway::new(), rx));
    }


    let cid: Arc<str> = Arc::from("client123");
    service.add_client(cid.clone()).await;
    service.set_is_active(cid.clone(), true).await;
    let is_active = service.get_is_active(cid).await;
    println!("is_active: {is_active}");
}
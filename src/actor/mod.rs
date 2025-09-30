use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc, thread, time::Duration,
};
use tokio::sync::mpsc::{self, Sender, Receiver, channel};
use tokio::sync::Mutex;
use std::collections::hash_map::DefaultHasher;

enum Commands {
    SetIsActive {
        client_id: Arc<str>,
        is_active: bool,
    },
    GetIsActive {
        client_id: Arc<str>,
        sender: Sender<bool>
    },
    AddClient {
        client_id: Arc<str>,
    }
}

struct Client {
    is_active: bool,
}

struct Gateway {
    clients: HashMap<String, Client>
}

struct GatewayService {
    clients: Vec<Sender<Commands>>
}

impl GatewayService {

    fn get_bucket(&self, client_id: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        client_id.hash(&mut hasher);
        let final_hash = hasher.finish();

        jump_hash(final_hash, self.clients.len() as i64) as usize
    }

    async fn send_command(&self, client_id: Arc<str>, command: Commands) {
        let bucket = self.get_bucket(&client_id);
        if let Some(sender) = self.clients.get(bucket) {
            if let Err(e) = sender.send(command).await {
                eprintln!("SendError: {e}");
            }
        } else {
            eprintln!("Client bucket not found for id: {client_id}");
        }
    }

    async fn add_client(&self, client_id: Arc<str>) {
        let client = self.get_bucket(&client_id);
        println!("add_client_id {client_id}");
        let x = self.clients[client].clone();
        if let Err(e) = x.send(Commands::AddClient { client_id }).await {
            println!("SendError: {e}");
        }
    }

    async fn set_is_active(&self, client_id: Arc<str>, is_active: bool) {
        let client = self.get_bucket(&client_id);
        println!("set_is_active for client_id: {client_id}, value: {is_active}.");
        let x = self.clients[client].clone();
        if let Err(e) = x.send(Commands::SetIsActive { client_id, is_active }).await {
            println!("SendError: {e}");
        }
    }

    async fn get_is_active(&self, client_id: Arc<str>) -> bool {
        let (sender, mut receiver) = channel::<bool>(1);
        self.send_command(
            client_id.clone(),
            Commands::GetIsActive { client_id, sender },
        )
        .await;

        match receiver.recv().await {
            Some(status) => {
                println!("Status: {status}");
                status
            },
            None => {
                eprintln!("Failed to receive is_active status");
                false
            }
        }
    }

}

impl Gateway {
    fn new() -> Self {
        Self { clients: HashMap::default() }
    }

    fn add_client(&mut self, client_id: Arc<str>) {
        self.clients.insert(client_id.to_string(), Client { is_active: false });
    }

    fn set_is_active(&mut self, client_id: Arc<str>, is_active: bool) {
        if let Some(client) = self.clients.get_mut(&client_id.to_string()) {
            client.is_active = is_active;
        }
    }

    fn get_is_active(&mut self, client_id: Arc<str>) -> bool {
        let client_id = &client_id.to_string();

        if let Some(client) = self.clients.get_mut(client_id) {
            return client.is_active
        } else {
            return false
        }
    }
}

async fn event_loop (gateway: Arc<Mutex<Gateway>>, mut rx: Receiver<Commands>) {
     while let Some(command) = rx.recv().await {
        let gateway = gateway.clone();
        tokio::task::spawn(async move {
            let mut g = gateway.lock().await;
            match command {
                Commands::AddClient { client_id } => g.add_client(client_id),
                Commands::SetIsActive { client_id, is_active } => g.set_is_active(client_id, is_active),
                Commands::GetIsActive { client_id, sender } => {
                    let v = g.get_is_active(client_id);
                    sender.send(v).await.unwrap();
                },
            }
        });
    }
}

pub async fn run() {
    let gat = Arc::new(Mutex::new(Gateway::new()));
    let num_buckets = 1;
    let mut service = GatewayService { clients: Vec::with_capacity(num_buckets) };


    for _ in 0..num_buckets {
    let (tx, rx) = mpsc::channel::<Commands>(1024); // bounded channel for backpressure
        service.clients.push(tx);
        tokio::spawn(event_loop(gat.clone(), rx));
    }

    let cid: Arc<str> = Arc::from("client123");
    service.add_client(cid.clone()).await;
    thread::sleep(Duration::from_millis(150));
    println!("Add Client");

    service.set_is_active(cid.clone(), true).await;
    thread::sleep(Duration::from_millis(150));
    println!("Set IsActive");

    let is_active = service.get_is_active(cid).await;
    println!("is_active: {is_active}");
}

// --------------------- Jump Hash Placeholder ---------------------
fn jump_hash(hash: u64, buckets: i64) -> i64 {
    // Replace with your actual jump hash implementation
    (hash % buckets as u64) as i64
}
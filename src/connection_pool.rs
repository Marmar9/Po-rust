use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

#[derive(Clone)]
pub struct RedisConnectionPool {
    connections: Arc<Mutex<Vec<redis::aio::Connection>>>,
    waiting_clients: Arc<Mutex<Vec<oneshot::Sender<()>>>>,
}

impl RedisConnectionPool {
    pub async fn new(num_connections: usize) -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let connections = Arc::new(Mutex::new(Vec::new()));
        let waiting_clients = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..num_connections {
            let connection = client.get_tokio_connection().await.expect("Failed to create connection");
            connections.lock().unwrap().push(connection);
        }

        RedisConnectionPool {
            connections,
            waiting_clients,
        }
    }

    pub async fn get_connection(&self) -> redis::aio::Connection {
        // Try to get a connection from the pool
        if let Some(connection) = self.connections.lock().unwrap().pop() {
            return connection;
        }

        // No available connections, so wait for one
        let (tx, rx) = oneshot::channel();
        self.waiting_clients.lock().unwrap().push(tx);

        rx.await.unwrap();

        // Placeholder return statement, adjust as needed
        redis::Client::open("redis://127.0.0.1/").unwrap().get_tokio_connection().await.unwrap()
    }

    pub async fn return_connection(&self, connection: redis::aio::Connection) {
        // Return the connection to the pool
        self.connections.lock().unwrap().push(connection);

        // If there are waiting clients, signal them that a connection is available
        if let Some(tx) = self.waiting_clients.lock().unwrap().pop() {
            tx.send(()).unwrap();
        }
    }
}

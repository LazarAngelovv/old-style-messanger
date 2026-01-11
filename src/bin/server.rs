use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}
};

use serde::{Serialize, Deserialize};
use chrono::Local;
use std::error:Error;

#[derive(Debug, Clone, Serialize, Deserialize)]

struct ChatMessage{
    username: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType{
    UserMessage,
    SystemNotification,
}
//What is Tokio?
//Async runtime for the rust language
//Async programming allows you to program
//to do things while waiting for one task to finish

#[tokio::main]

async fn main()->Result<(),Box<dyn Error>{
    let listener = TcpListener::bind("127.0.0.1:8082").await?;

    // Display server startup message with formatting
    println!("╔════════════════════════════════════════╗");
    println!("║        RETRO CHAT SERVER ACTIVE        ║");
    println!("║        Port: 8082  Host: 127.0.0.1     ║");
    println!("║        Press Ctrl+C to shutdown        ║");
    println!("╚════════════════════════════════════════╝");

    //Create a boradcast channel for message distribution
    let(tx: Sender<String>, _) = broadcast::channel::<String>(100);

    // Main server loop to handle incoming connections
    loop {
           let (socket, addr) = listener.accept().await?; // Accept a new connection

           // Display connection information
           println!("┌─[{}] New connection", Local::now().format("%H:%M:%S"));
           println!("└─ Address: {}", addr);

           let tx: Sender<String> = tx.clone());
           let rx: Receiver<String> = tx.subscribe();

           tokio::spawn(feature: async move)
           //tokio::spawn(async move{
           // //handle_connection
           // })
    }
    //Function to handle the client connection

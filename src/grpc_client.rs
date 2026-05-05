pub mod services {
    tonic::include_proto!("services");
}

use services::{
    ChatMessage, PaymentRequest, TransactionRequest, chat_service_client::ChatServiceClient,
    payment_service_client::PaymentServiceClient,
    transaction_service_client::TransactionServiceClient,
};

use tokio::{
    io::{self, AsyncBufReadExt},
    sync::mpsc,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, transport::Channel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "http://[::1]:50051";

    let mut payment_client = PaymentServiceClient::connect(address).await?;
    let request = Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });

    let response = payment_client.process_payment(request).await?;
    println!("RESPONSE={:?}", response.into_inner());

    let mut transaction_client = TransactionServiceClient::connect(address).await?;
    let request = Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });

    let mut stream = transaction_client
        .get_transaction_history(request)
        .await?
        .into_inner();
    while let Some(transaction) = stream.message().await? {
        println!("Transaction {:?}", transaction);
    }

    let channel = Channel::from_static(address).connect().await?;
    let mut client = ChatServiceClient::new(channel);
    let (tx, rx) = mpsc::channel::<ChatMessage>(32);

    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            if line.trim().is_empty() {
                continue;
            }
            let message = ChatMessage {
                user_id: "user_123".to_string(),
                message: line,
            };

            if tx.send(message).await.is_err() {
                eprintln!("Failed to send message to server.");
                break;
            }
        }
    });

    let request = Request::new(ReceiverStream::new(rx));
    let mut response_stream = client.chat(request).await?.into_inner();

    while let Some(response) = response_stream.message().await? {
        println!("Server says: {:?}", response);
    }

    Ok(())
}

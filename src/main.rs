use borsh::{BorshDeserialize, BorshSerialize};
use futures_util::stream::StreamExt;
use lapin::{
    options::*,
    types::FieldTable,
    Connection,
    ConnectionProperties,
};
use std::{thread, time};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() {
    let conn = Connection::connect(
        "amqp://guest:guest@127.0.0.1:5672/%2f",
        ConnectionProperties::default(),
    )
    .await
    .expect("Failed to connect");

    let channel = conn.create_channel().await.expect("Create channel failed");

    channel
        .queue_declare(
            "user_created",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Queue declare failed");

    let mut consumer = channel
        .basic_consume(
            "user_created",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Basic consume failed");

    println!("Subscriber berjalan. Menunggu message...");

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let message =
                UserCreatedEventMessage::try_from_slice(&delivery.data)
                    .expect("Failed deserialize");

            let ten_millis = time::Duration::from_millis(1000);

            // Uncomment nanti untuk slow subscriber:
            thread::sleep(ten_millis);

            println!(
                "In Tirta's Computer [2406355621]. Message received: {:?}",
                message
            );

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Ack failed");
        }
    }
}
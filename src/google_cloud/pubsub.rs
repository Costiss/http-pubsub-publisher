use std::collections::HashMap;

use google_cloud_default::WithAuthExt;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::{Client, ClientConfig};

use tracing::{error, info};

pub async fn enqueue(
    topic: String,
    payload: serde_json::Value,
    attributes: HashMap<String, String>,
) {
    let config = ClientConfig::default().with_auth().await.unwrap();
    let client = Client::new(config).await.unwrap();

    let topic = client.topic(topic.as_str());

    let publisher = topic.new_publisher(None);

    let message = PubsubMessage {
        data: payload.to_string().into(),
        attributes,
        ..Default::default()
    };

    info!("Publishing message: {:?}", payload.to_string());

    let awaiter = publisher.publish(message).await;
    let result = awaiter.get().await;
    match result {
        Ok(ok) => info!("Enqueued: {:?}", ok),
        Err(err) => error!("Error: {:?}", err.to_string()),
    }
}

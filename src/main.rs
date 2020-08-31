pub mod protos;

use grpcio::{ChannelBuilder, EnvBuilder, ClientSStreamReceiver};
use std::sync::{Arc, Mutex};
use protobuf::{SingularPtrField, RepeatedField};
use futures::{StreamExt};
use std::io::{Write};
use futures::executor::{block_on};

#[tokio::main]
async fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect(format!("localhost:50051").as_str());

    let client
        = Arc::new(Mutex::new(protos::network_api_grpc::NetworkServiceClient::new(channel)));

    let pub_client = Arc::clone(&client);
    loop {
        let client = pub_client.lock().unwrap();
        let reader = std::io::stdin();
        let mut input = String::new();

        print!("Enter a number: ");
        std::io::stdout().flush().ok().expect("Could not flush output");
        let _input_result = reader.read_line(&mut input);
        input = input.trim().to_string();
        let input_as_int: i64 = input.parse().unwrap_or(0);
        block_on(async {
            publish_signals(&client, &input_as_int).await;
            subscribe_to_signals(&client).await;
        });
    }
}

// Create a subscribing stream from set of signalIDs to signal-server
async fn subscribe_to_signals(client: &protos::network_api_grpc::NetworkServiceClient) {
    let mut client_id = protos::common::ClientId::new();
    client_id.id = "rusty_client_sub".to_string();

    let mut subscriber_config = protos::network_api::SubscriberConfig::new();
    subscriber_config.clientId = SingularPtrField::some(client_id);
    subscriber_config.signals = generate_signal_ids();

    let mut sub_info: ClientSStreamReceiver<protos::network_api::Signals> =
        client.subscribe_to_signals(&subscriber_config).unwrap();

    match sub_info.next().await {
        Some(resp) => {
            println!("{:?}", resp.unwrap().signal.as_ref());
        }
        None => {}
    }
}

// Publish signals to signal-broker over gRPC
async fn publish_signals(client: &protos::network_api_grpc::NetworkServiceClient, payload: &i64) {
    let mut client_id = protos::common::ClientId::new();
    client_id.id = "rusty_client_pub".to_string();

    let mut publisher_config = protos::network_api::PublisherConfig::new();
    publisher_config.clientId = SingularPtrField::some(client_id);
    publisher_config.signals = generate_signals(payload);
    let _result = client.publish_signals_async(&publisher_config).unwrap().await;
}

// Generating set of SignalsIDs, for subscriptions
fn generate_signal_ids() -> SingularPtrField<protos::network_api::SignalIds> {
    let mut namespace = protos::common::NameSpace::new();
    namespace.name = "VirtualInterface".to_string();

    let mut signal_id = protos::common::SignalId::new();
    signal_id.name = "virtual_signal".to_string();
    signal_id.namespace = SingularPtrField::some(namespace);

    let signal_ids_vector = vec![signal_id];

    let mut signal_ids = protos::network_api::SignalIds::new();
    signal_ids.signalId = RepeatedField::from_vec(signal_ids_vector);

    SingularPtrField::some(signal_ids)
}

// Generating set of Signals, for publishing
fn generate_signals(payload: &i64) -> SingularPtrField<protos::network_api::Signals> {
    let mut namespace = protos::common::NameSpace::new();
    namespace.name = "VirtualInterface".to_string();

    let mut signal_id = protos::common::SignalId::new();
    signal_id.name = "virtual_signal".to_string();
    signal_id.namespace = SingularPtrField::some(namespace);

    let signal_payload = Option::from(protos::network_api::Signal_oneof_payload::integer(*payload));

    let mut signal = protos::network_api::Signal::new();
    signal.id = SingularPtrField::some(signal_id);
    signal.payload = signal_payload;

    let signals_vector = vec![signal];

    let mut signals = protos::network_api::Signals::new();
    signals.signal = RepeatedField::from_vec(signals_vector);

    SingularPtrField::some(signals)
}


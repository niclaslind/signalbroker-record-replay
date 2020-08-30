pub mod protos;

use grpcio::{ChannelBuilder, EnvBuilder, ClientSStreamReceiver};
use std::sync::{Arc, Mutex};
use protobuf::{SingularPtrField, RepeatedField};
use std::thread;
use std::time::Duration;
use futures::Stream;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:50051").as_str());
    let client =
        Arc::new(Mutex::new(protos::network_api_grpc::NetworkServiceClient::new(ch)));

    let sub_client = Arc::clone(&client);
    let subscribe = thread::spawn(move || {
        let client = sub_client.lock().unwrap();
        subscribe_to_signals(&client);
    });

    let pub_client = Arc::clone(&client);
    let publish = thread::spawn(move || loop {
        let client = pub_client.lock().unwrap();
        publish_signals(&client);
        thread::sleep(Duration::from_millis(500));
    });

    subscribe.join().expect("The subscribe thread has panicked");
    publish.join().expect("The publisher thread has panicked");
}

// Create a subscribing stream from set of signalIDs to signal-server
fn subscribe_to_signals(client: &protos::network_api_grpc::NetworkServiceClient) {
    let mut client_id = protos::common::ClientId::new();
    client_id.id = "rusty_client_sub".to_string();

    let mut subscriber_config = protos::network_api::SubscriberConfig::new();
    subscriber_config.clientId = SingularPtrField::some(client_id);
    subscriber_config.signals = generate_signal_ids();

    /* TODO Need to understand how to handle a ClientSStreamReceiver,
             for now it doesn't work to use poll_next function
    */
    let sub_info: ClientSStreamReceiver<protos::network_api::Signals> = client.subscribe_to_signals(&subscriber_config).unwrap();
    match sub_info.poll_next() {}
}

// Publish signals to signal-broker over gRPC
fn publish_signals(client: &protos::network_api_grpc::NetworkServiceClient) {
    let mut client_id = protos::common::ClientId::new();
    client_id.id = "rusty_client_pub".to_string();

    let mut publisher_config = protos::network_api::PublisherConfig::new();
    publisher_config.clientId = SingularPtrField::some(client_id);
    publisher_config.signals = generate_signals();
    let _result = client.publish_signals(&publisher_config).unwrap();
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
fn generate_signals() -> SingularPtrField<protos::network_api::Signals> {
    let mut namespace = protos::common::NameSpace::new();
    namespace.name = "VirtualInterface".to_string();

    let mut signal_id = protos::common::SignalId::new();
    signal_id.name = "virtual_signal".to_string();
    signal_id.namespace = SingularPtrField::some(namespace);

    // return the time_now - UNIX_EPOCH
    let payload = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Something went wrong")
        .as_secs() as i64;

    let signal_payload = Option::from(protos::network_api::Signal_oneof_payload::integer(payload));

    let mut signal = protos::network_api::Signal::new();
    signal.id = SingularPtrField::some(signal_id);
    signal.payload = signal_payload;

    let signals_vector = vec![signal];

    let mut signals = protos::network_api::Signals::new();
    signals.signal = RepeatedField::from_vec(signals_vector);

    SingularPtrField::some(signals)
}


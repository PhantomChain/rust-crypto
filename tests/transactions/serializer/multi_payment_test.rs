use phantomchain_crypto::configuration::network;
use phantomchain_crypto::enums::Network;
use phantomchain_crypto::transactions::{deserializer, serializer};
use *;

#[test]
#[ignore]
fn test_signed_with_a_passphrase() {
    network::set(Network::Devnet);

    let fixture = json_transaction("multi_payment", "passphrase");
    let serialized = fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(&serialized);

    let actual = serializer::serialize(&transaction);
    assert_eq!(actual, serialized);
}

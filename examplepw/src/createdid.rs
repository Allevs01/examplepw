// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;

use identity_storage::Storage;
use identity_stronghold::StrongholdStorage;
use examplepw::utils::stronghold_path;
use examplepw::utils::random_stronghold_path;
use examplepw::utils::get_address_with_funds;
use examplepw::utils::MemStorage;
use identity_iota::core::ToJson;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::NetworkName;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::MethodScope;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;
use iota_sdk::types::block::output::AliasOutput;

/// Demonstrates how to create a DID Document and publish it in a new Alias Output.
///
/// In this example we connect to a locally running private network, but it can be adapted
/// to run on any IOTA node by setting the network and faucet endpoints.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // The API endpoint of an IOTA node, e.g. Hornet.

  // The faucet endpoint allows requesting funds for testing purposes.
  let faucet_endpoint: &str = "http://localhost/faucet/api/enqueue";

  // Stronghold snapshot path.
  let base_directory = Path::new("/home/sallevi/Scrivania/examplepw/stronghold_path");
  let path = stronghold_path(base_directory);

  // Stronghold password.
  let password = Password::from("secure_password".to_owned());

  println!("{}", path.to_str().unwrap());


  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node("http://localhost", None)?
    .finish()
    .await?;

  // Create a new secret manager backed by a Stronghold.
  let stronghold = StrongholdSecretManager::builder()
        .password(password.clone())
        .build(path.clone())?;

  let stronghold_storage = StrongholdStorage::new(stronghold);

  // Get an address with funds for testing.
  let address: Address = get_address_with_funds(&client, stronghold_storage.as_secret_manager(), faucet_endpoint).await?;

  // Get the Bech32 human-readable part (HRP) of the network.
  let network_name: NetworkName = client.network_name().await?;

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  let mut document: IotaDocument = IotaDocument::new(&network_name);

  // Insert a new Ed25519 verification method in the DID document.
  let storage = Storage::new(stronghold_storage.clone(), stronghold_storage.clone());
  
  //it's necessary to keep the fragment in order to use it when you have to sign credentials
  let fragment =document
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await?;
  println!("{}", fragment);

  // Construct an Alias Output containing the DID document, with the wallet address
  // set as both the state controller and governor.
  let alias_output: AliasOutput = client.new_did_output(address, document, None).await?;

  // Publish the Alias Output and get the published DID document.
  let document: IotaDocument = client.publish_did_output(stronghold_storage.as_secret_manager(), alias_output).await?;
  println!("Published DID document: {document:#}");

  Ok(())
}

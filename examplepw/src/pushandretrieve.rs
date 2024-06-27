// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to create a Verifiable Credential and validate it.
//! In this example, alice takes the role of the subject, while we also have an issuer.
//! The issuer signs a UniversityDegreeCredential type verifiable credential with Alice's name and DID.
//! This Verifiable Credential can be verified by anyone, allowing Alice to take control of it and share it with
//! whomever they please.
//!
//! cargo run --release --example 5_create_vc

use examplepw::identitylib::create_vc_example;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::Object;

use identity_iota::credential::DecodedJwtCredential;
use identity_iota::credential::Jwt;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::JwtCredentialValidator;
use identity_iota::credential::JwtCredentialValidatorUtils;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaIdentityClientExt;
use iota_sdk::client::Client;

use examplepw::utils::API_ENDPOINT;
use examplepw::identitylib::{create_vc, get_issuer_storage};
use identity_iota::credential::FailFast;
use identity_iota::iota::IotaDocument;
use iota_sdk::types::block::payload::Payload;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new client to interact with the IOTA ledger.
    let client: Client = Client::builder()
        .with_primary_node(API_ENDPOINT, None)?
        .finish()
        .await?;

    //change these dids with other ones you generate with the first example
    let did_string_issuer = "did:iota:snd:0xdd69166eea24b073a7f6a849e9533fe16d48500edcd4b3f17f9cfef1249bd625";
    let did_string_holder = "did:iota:snd:0x8d5b36023bc20e9bc26bb3353a5d50bf31e29b75aff01fd962419ff69e77ad2c";

    let did_holder: IotaDID = IotaDID::parse(did_string_holder).unwrap();
    let did_issuer: IotaDID = IotaDID::parse(did_string_issuer).unwrap();

    let fragment = "_deh8uNhAlbLgt7OBWVzCRXzHox6-4-dQVtJx4JM6kI";

    //recover issuer storage from stronghold to issue the credential
    let password = "secure_password";

    let stronghold_path = "/home/sallevi/Scrivania/examplepw/stronghold_path/test_strongholds/bYK0jFH2W2j8cKYZvvjkBC5aPbPp3Zqi.stronghold";

    let storage = get_issuer_storage(&stronghold_path, &password).await?;

    //recover DID documents
    let holder_document: IotaDocument = client.resolve_did(&did_holder).await?;
    let issuer_document: IotaDocument = client.resolve_did(&did_issuer).await?;

    //issue vc
    let credential_jwt = create_vc_example(&storage,&fragment,&issuer_document,&holder_document).await?;
    println!("credential jwt : {:#?}", credential_jwt);
    
    //verify credential
    let decoded_credential: DecodedJwtCredential<Object> =
          JwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default())
            .validate::<_, Object>(
              &credential_jwt,
              &issuer_document,
              &JwtCredentialValidationOptions::default(),
              FailFast::FirstError,
            )
            .unwrap();

    println!("VC successfully validated");
    println!("Credential JSON > {:#}", decoded_credential.credential);

    //set the tag and the payload of the block
    let tag = std::env::args().nth(1).unwrap_or_else(|| "My first tag".to_string());
    let data = std::env::args().nth(2).unwrap_or_else(|| credential_jwt.as_str().to_string());

    // Create and send the block with tag and data, this method returns the block pushed
    let block = client
        .build_block()
        .with_tag(tag.as_bytes().to_vec())
        .with_data(data.as_bytes().to_vec())
        .finish()
        .await?;

    println!("{block:#?}\n");

    println!(
        "Block with tag and data sent: {}/block/{}",
        "https://explorer.shimmer.network/testnet",
        block.id()
    );

    //for testing purpose we will use the block id directly to get the block (even if we already have it)
    
    let block1 = client.get_block(&block.id()).await?;

    let mut jwt_string:String = "".to_string();

    //we recover the payload (credential jwt) from the block
    if let Some(Payload::TaggedData(payload)) = block1.payload() {
        jwt_string = String::from_utf8(payload.data().to_vec()).expect("found invalid UTF-8");
    }

    //the recovered jwt
    let jwt = Jwt::from(jwt_string.to_string());

    println!("{:#?}", jwt);

    //Now, to be sure, we verify again the credential jwt by extracting the issuer from it
    let issuer_did: IotaDID = JwtCredentialValidatorUtils::extract_issuer_from_jwt(&jwt)?;
    let issuer1_document: IotaDocument = client.resolve_did(&issuer_did).await?;

    let decoded_credential1: DecodedJwtCredential<Object> =
    JwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default())
    .validate::<_, Object>(
        &jwt,
        &issuer1_document,
        &JwtCredentialValidationOptions::default(),
        FailFast::FirstError,
    )
    .unwrap();

    println!("VC successfully validated");

    println!("Credential JSON > {:#}", decoded_credential1.credential);

  Ok(())
}
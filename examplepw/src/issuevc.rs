// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to create a Verifiable Credential and validate it.
//! In this example, alice takes the role of the subject, while we also have an issuer.
//! The issuer signs a UniversityDegreeCredential type verifiable credential with Alice's name and DID.
//! This Verifiable Credential can be verified by anyone, allowing Alice to take control of it and share it with
//! whomever they please.
//!
//! cargo run --release --example 5_create_vc

use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::Object;

use identity_iota::credential::DecodedJwtCredential;
use identity_iota::credential::Jwt;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::JwtCredentialValidator;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaIdentityClientExt;
use iota_sdk::client::Client;

use examplepw::utils::API_ENDPOINT;
use examplepw::identitylib::{create_vc, get_issuer_storage};
use identity_iota::credential::FailFast;
use identity_iota::iota::IotaDocument;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new client to interact with the IOTA ledger.
    let client: Client = Client::builder()
        .with_primary_node(API_ENDPOINT, None)?
        .finish()
        .await?;

    //change this did with another one you generate
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
    let credential_jwt = create_vc(&storage,&fragment,&issuer_document,&holder_document).await?;
    
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


  Ok(())
}
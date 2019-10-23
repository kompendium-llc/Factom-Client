use super::*;

impl Factom {
/**
This method, compose-chain, will return the appropriate API calls to create a 
chain in factom. You must first call the commit-chain, then the reveal-chain 
API calls. To be safe, wait a few seconds after calling commit.

Note: The firstentry fields are automatically hex encoded for the server to 
process.
# Example
```
use factom::*;

let ec_address = "EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx";
let extids = vec!("Cargo Test", "test harness");
let content = "Here be the content";
let factom = Factom::new();
let query = factom
              .compose_chain(extids, content, ec_address)
              .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn compose_chain(self, extids: Vec<&str>, content: &str, ecpub: &str)
                        -> impl Future<Item=Response, Error=FetchError>{
    
    let mut params = HashMap::new();
    let hex_content = str_to_hex(content);
    let mut hex_extids = Vec::new();
    for extid in extids{
      hex_extids.push(str_to_hex(extid));
    }
    let chain = json!({
      "firstentry": {
        "extids": hex_extids,
        "content": hex_content
      }
    });
    params.insert("chain".to_string(), chain);
    params.insert("ecpub".to_string(), json!(ecpub));
    dbg!(&params);
    self.walletd_call("compose-chain", params)
  }

/**
This method, compose-entry, will return the appropriate API calls to create an 
entry in factom. You must first call the commit-entry, then the reveal-entry 
API calls. To be safe, wait a few seconds after calling commit.

Note: The entry fields are automatically hex encoded for the server to process.
# Example
```
use factom::*;

let chainid = "9dec48601fba6ddb4bcea12066ba0f2b2467f89c788c5a243eb253c3de0f815b";
let ec_address = "EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx";
let extids = vec!("Cargo Test", "test harness");
let content = "Here be the content";
let factom = Factom::new();
let query = factom
      .compose_entry(chainid, extids, content, ec_address)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success()); 
```
*/
  pub fn compose_entry(self, chainid: &str, extids: Vec<&str>, content: &str, ecpub: &str)
                        -> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    let mut hex_extids = Vec::new();
    for extid in extids {
      hex_extids.push(str_to_hex(extid));
    }
    let entry = json!({
    "chainid": chainid,
    "extids": hex_extids,
    "content": str_to_hex(content)
    });
    params.insert("entry".to_string(), entry);
    params.insert("ecpub".to_string(), json!(ecpub));
    self.walletd_call("compose-entry", params)
  }

/**
Compose transaction marshals the transaction into a hex encoded string. The 
string can be inputted into the factomd API factoid-submit to be sent to 
the network.
# Example
```
use factom::*;

let tx_name = "my-tx";
let factom = Factom::new();
let query = factom
      .compose_transaction(tx_name)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success()); 
```
*/
  pub fn compose_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("tx-name".to_string(), json!(tx_name));
    self.walletd_call("compose-transaction", params)
  }

/**
This request allows one identity to state an attribute about another identity 
and publish that entry to any existing chain. An attribute is a set of generic 
key:value pairs that can be assigned to an identity and is flexible enough to 
accommodate many different use-cases. In the example request, an identity is 
giving itself an attribute describing its current email address, and writing 
that entry to its own identity chain. Each attribute must be in the format of 
{"key":KEY, "value":VALUE} where KEY and VALUE can be of any valid JSON type. 
Each attribute you wish to assign must be put into an array, even if it is 
just a single key/value pair. 
For example: [{"key":"worksAt", "value":"Factom Inc."}, {"key":"isNice", 
"value":true}] would be a valid attributes array to use as a parameter. If the 
wallet is encrypted, it must be unlocked prior to using this command.

The entry will be constructed based on the information you included in the request.

receiver-chainid - the Chain ID for the identity being assigned the attribute

destination-chainid - the Chain ID where the attribute entry will be written. 
Could be any existing chain, dream big.

attributes - the array of attributes that you are assigning to the receiver

signerkey - the public identity key being used to sign the entry. Must be 
stored in the wallet already and should be a currently valid key for the 
signer identity.

signer-chainid - the Identity Chain of the signing party (who is giving the 
attribute)

The response you receive is similar to the compose-entry response. You must 
first call the commit-entry, then the reveal-entry API calls. To be safe, 
wait a few seconds after calling commit.

#Example
```
use factom::*;
use std::collections::Hashmap;

let rx_chain = "3b69dabe22c014af9a9bc9dfa7917ce4602a03579597ddf184d8de56702512ae";
let dst_chain = "3b69dabe22c014af9a9bc9dfa7917ce4602a03579597ddf184d8de56702512ae";
let mut attributes = Hashmap::new();
attributes.insert("key".to_string(), "email");
attributes.insert("value".to_string(), "hello@factom.com");
attributes = vec!(attributes);
let signerkey = "idpub2cw4NS4JZowXTwhGeo2tTGNvnjc5n2QvHBURdvVFCKRDuLEnBh";
let signer_chain = "3b69dabe22c014af9a9bc9dfa7917ce4602a03579597ddf184d8de56702512ae";
let ecpub = "EC2ZFTmTv5Fs7UyKZzxY8km4jF635VkhR5KKBMzNP4BK4fPKaVw4";
let force = false;

let factom = Factom::new();
let query = factom
            .compose_id_attribute(
              rx_chain,
              dst_chain,
              attributes,
              signerkey,
              signer_chain,
              ecpub,
              force
            )
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```
 */
  pub fn compose_id_attribute<T>(
    self, 
    receiver_chain: &str,
    destination_chain: &str,
    attributes: Vec<(T, T)>,
    signer_key: &str,
    signer_chainid: &str,
    ecpub: &str,
    force: bool
  ) -> impl Future<Item=Response, Error=FetchError> where 
    T: Serialize,
  {
    let mut params = HashMap::new();
    params.insert("receiver-chainid".to_string(), json!(receiver_chain));
    params.insert("destination-chainid".to_string(), json!(destination_chain));
    params.insert("signerkey".to_string(), json!(signer_key));
    params.insert("signer-chainid".to_string(), json!(signer_chainid));
    params.insert("ecpub".to_string(), json!(ecpub));
    params.insert("force".to_string(), json!(force));
    let mut attr_list = vec!(HashMap::new());
    for attr in attributes {
      let mut map = HashMap::new();
      map.insert("key".to_string(), attr.0);
      map.insert("value".to_string(), attr.1);
      attr_list.push(map);
    }
    params.insert("attributes".to_string(), json!(attr_list));
    self.walletd_call("compose-identity-attribute", params)
  }

/**
This method helps you endorse an attribute that has already been registered on 
the Factom blockchain. To do this, you’ll need to create a structured entry on 
to the Identity chain. The compose-identity-attribute-endorsement method will 
return the API calls needed to create that entry. If the wallet is encrypted, 
it must be unlocked prior to using this command.

An “endorsement” is a statement that one identity agrees with, recognizes, or 
co-signs an attribute that has been given to another identity. In the example 
request, we created an endorsement entry that points to the email attribute 
entry we gave in the previous section. Depending on the application context 
this could mean different things. One such use-case would be to reconfirm that 
the attribute is still valid and that email address is still correct at present. 
Just like attributes, endorsements are very generic so that they can be used 
in a variety of ways.

The entry will be constructed based on the information you included in the 
request:

destination-chainid - the Chain ID where the attribute entry will be written. 
Could be any existing chain, dream big.

entry-hash - the entry hash of the attribute that will be endorsed

signerkey - the public identity key being used to sign the entry. Must be 
stored in the wallet already and should be a currently valid key for the 
signer identity.

signer-chainid - the Identity Chain of the signing party (who is endorsing the 
attribute located at entry-hash)

The response you receive is similar to the compose-entry response. You must 
first call the commit-entry, then the reveal-entry API calls. To be safe, 
wait a few seconds after calling commit.

#Example
```
use factom::*;

let entry_hash = "c07f1d89bb6c43e7e3166b9e53672110ff8077c367758fbe4265561c8b91e675";
let dst_chain = "3b69dabe22c014af9a9bc9dfa7917ce4602a03579597ddf184d8de56702512ae";
let signerkey = "idpub2cw4NS4JZowXTwhGeo2tTGNvnjc5n2QvHBURdvVFCKRDuLEnBh";
let signer_chain = "3b69dabe22c014af9a9bc9dfa7917ce4602a03579597ddf184d8de56702512ae";
let ecpub = "EC2ZFTmTv5Fs7UyKZzxY8km4jF635VkhR5KKBMzNP4BK4fPKaVw4";
let force = false;

let factom = Factom::new();
let query = factom
            .compose_id_attribute_endorsement(
              dst_chain,
              entry_hash,
              signerkey,
              signer_chain,
              ecpub,
              force
            )
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```
 */
  pub fn compose_id_attribute_endorsement(
    self, 
    destination_chain: &str,
    entry_hash: &str,
    signer_key: &str,
    signer_chainid: &str,
    ecpub: &str,
    force: bool
  ) -> impl Future<Item=Response, Error=FetchError> 
  {
    let mut params = HashMap::new();
    params.insert("destination-chainid".to_string(), json!(destination_chain));
    params.insert("entry-hash".to_string(), json!(entry_hash));
    params.insert("signerkey".to_string(), json!(signer_key));
    params.insert("signer-chainid".to_string(), json!(signer_chainid));
    params.insert("ecpub".to_string(), json!(ecpub));
    params.insert("force".to_string(), json!(force));
    self.walletd_call("compose-identity-attribute-endorsement", params)
  }

/**
The compose-identity-chain method will return the appropriate API calls to 
create an identity chain in factom. The chain will be constructed based on the 
name and public keys that you send in the request. The response you receive is 
similar to the compose-chain response. You must first call the commit-chain, 
then the reveal-chain API calls. To be safe, wait a few seconds after calling 
commit. If the wallet is encrypted, it must be unlocked prior to using this 
command.

The chain to be created will contain the identity name as the ExtIDs of the 
first entry (a.k.a. the Chain Name) and also a JSON object in the Content 
field signifying the identity version and initial valid keys. This first set 
of keys are listed in order of decreasing priority. The first key is a 
“master” key of sorts that should be kept as securely as possible and used 
infrequently — it is the only key that can transfer complete ownership of an 
identity. The last key in the array is the lowest priority, meaning that it 
can be kept in less secure locations and used more frequently. A higher 
priority key can always just replace a lower priority key that was 
compromised or simply lost. For more information on key replacements, see the 
compose-identity-key-replacement section.

#Example
```
use factom::*;

let name = vec!["Factom", "Test", "ID"];
let pubkeys = vec![
  "idpub2k8zGYQUfekxehyUKeqPw6QPiJ5hkV3bbc9JBgL7GNrEiqMpQX",
  "idpub3fXRj21gXveTk6RKYrpJniWV2pAanQktekEt62yhJUQXyPdvwL",
  "idpub2GU1Pcax2PibH8hHZg58fKRiSJKQWQkWYkpmt7VH1jCXBgqp9w"];
let ecpub = "EC2ZFTmTv5Fs7UyKZzxY8km4jF635VkhR5KKBMzNP4BK4fPKaVw4";
let force = false;

let factom = Factom::new();
let query = factom
            .compose_id_chain(
              name,
              pubkeys,
              ecpub,
              force
            )
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```
 */
  pub fn compose_id_chain(
    self, 
    name: Vec<&str>,
    pubkeys: Vec<&str>,
    ecpub: &str,
    force: bool
  ) -> impl Future<Item=Response, Error=FetchError> 
  {
    let mut params = HashMap::new();
    params.insert("name".to_string(), json!(name));
    params.insert("pubkeys".to_string(), json!(pubkeys));
    params.insert("ecpub".to_string(), json!(ecpub));
    params.insert("force".to_string(), json!(force));
    self.walletd_call("compose-identity-chain", params)
  }

/**
Replacing one of an identity’s keys is done by adding a structured entry onto 
the identity’s chain. This will need to be done if you feel that a key was 
compromised or has been in use for too long. The compose-identity-key-replacement 
method will return the API calls needed to create the replacement entry. The 
response you receive is similar to the compose-entry response. You must first 
call the commit-entry, then the reveal-entry API calls. To be safe, wait a few 
seconds after calling commit. If the wallet is encrypted, it must be unlocked 
prior to using this command.

The entry will be constructed based on the information you included in the 
request.

* chainid - the ChainID of the identity chain in question

* oldkey - the public identity key for the level to be replaced

* newkey - the public identity key that will be replacing oldkey

* signerkey - the public identity key that will sign the entry and authorize the 
replacement. This key must be stored in the wallet already and must be of the 
same or higher priority than the oldkey in the context of the given Identity Chain.

#Example
```
use factom::*;

let chain_id = "3b69dabe22c014af9a9bc9dfa7917ce4602a03579597ddf184d8de56702512ae";
let old_key = "idpub2GU1Pcax2PibH8hHZg58fKRiSJKQWQkWYkpmt7VH1jCXBgqp9w";
let new_key = "idpub2cw4NS4JZowXTwhGeo2tTGNvnjc5n2QvHBURdvVFCKRDuLEnBh";
let signer_key = "idpub2GU1Pcax2PibH8hHZg58fKRiSJKQWQkWYkpmt7VH1jCXBgqp9w";
let ecpub = "EC2ZFTmTv5Fs7UyKZzxY8km4jF635VkhR5KKBMzNP4BK4fPKaVw4";
let force = false;

let factom = Factom::new();
let query = factom
            .compose_id_key_replacement(
              chain_id,
              old_key,
              new_key,
              signer_key,
              ecpub,
              force
            )
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```
 */
  pub fn compose_id_key_replacement(
    self, 
    chain_id: &str,
    old_key: &str,
    new_key: &str,
    signer_key: &str,
    ecpub: &str,
    force: bool
  ) -> impl Future<Item=Response, Error=FetchError> 
  {
    let mut params = HashMap::new();
    params.insert("chainid".to_string(), json!(chain_id));
    params.insert("oldkey".to_string(), json!(old_key));
    params.insert("newkey".to_string(), json!(new_key));
    params.insert("signerkey".to_string(), json!(signer_key));
    params.insert("ecpub".to_string(), json!(ecpub));
    params.insert("force".to_string(), json!(force));
    self.walletd_call("compose-identity-key-replacement", params)
  }

}

/// Struct for deserialising the results of the functions: compose-chain
/// compose-entry
/// compose-identity-attribute
/// compose-identity-attribute-endorsement
/// compose-identity-chain
/// compose-identity-key-replacementt
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Compose {
    commit: Commit,
    reveal: Reveal,
}

/// Struct for deserialising the results of: compose-chain
/// compose-entry
/// compose-identity-attribute
/// compose-identity-attribute-endorsement
/// compose-identity-chain
/// compose-identity-key-replacementt
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Commit {
    jsonrpc: String,
    id: i64,
    // #[serde(rename = "params")]
    params: CommitParams,
    method: String,
}

/// Struct for deserialising the results of the functions: compose-chain
/// compose-entry
/// compose-identity-attribute
/// compose-identity-attribute-endorsement
/// compose-identity-chain
/// compose-identity-key-replacementt
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct CommitParams {
    message: String,
}

/// Struct for deserialising the results of the functions: compose-chain
/// compose-entry
/// compose-identity-attribute
/// compose-identity-attribute-endorsement
/// compose-identity-chain
/// compose-identity-key-replacementt
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Reveal {
    jsonrpc: String,
    id: i64,
    // #[serde(rename = "params")]
    params: RevealParams,
    method: String,
}

/// Struct for deserialising the results of the functions: compose-chain
/// compose-entry
/// compose-identity-attribute
/// compose-identity-attribute-endorsement
/// compose-identity-chain
/// compose-identity-key-replacementt
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct RevealParams {
    entry: String,
}

/// compose-transaction function
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct ComposeTx {
    jsonrpc: String,
    id: i64,
    params: TxParams,
    method: String,
}

/// compose-transaction function
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct TxParams {
    transaction: String,
}


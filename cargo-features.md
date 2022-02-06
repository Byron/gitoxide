## Cargo features guide

Cargo uses feature toggles to control which dependencies are pulled in, allowing users to specialize crates to fit their usage.
Ideally, these should be additive.
This guide documents which features are available for each of the crates provided here and how they function.

### gitoxide

Documented in [its documentation](https://docs.rs/gitoxide-core).

### gitoxide-core

Documented in [its documentation](https://docs.rs/gitoxide-core).

### git-pack

Documented in [its documentation](https://docs.rs/git-pack).

### git-actor

Documented in [its documentation](https://docs.rs/git-actor).

### git-features

Documented in [its documentation](https://docs.rs/git-features).
     
### git-packetline

Documented in [its documentation](https://docs.rs/git-packetline).

### git-transport

Documented in [its documentation](https://docs.rs/git-transport).
    
### git-protocol

Documented in [its documentation](https://docs.rs/git-protocol).
    
### git-object

Documented in [its documentation](https://docs.rs/git-object).
  
### git-repository

Documented in [its documentation](https://docs.rs/git-repository).

### Serialization Support

What follows is feature toggles to control serialization of all public facing simple data types.

* **serde1**
    * Data structures implement `serde::Serialize` and `serde::Deserialize`

The feature above is provided by the crates:

* **git-actor**
* **git-object**
* **git-ref**
* **git-url**
* **git-odb**
* **git-protocol**
* **git-repository**
* **gitoxide-core**
 

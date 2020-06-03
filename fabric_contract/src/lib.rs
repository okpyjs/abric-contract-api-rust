/*
 * SPDX-License-Identifier: Apache-2.0
 */

#![allow(unused_variables)]

//! This is the main crate for providing support for writing Smart Contracts to work with
//! Hyperledger Fabric.
//!
//! Any rust contract must be compiled to target Wasm. They can then be run in the Wasm chaincode
//!
//! In addition to this crate, there is the `contract_macros` crate that contains the macros needed
//!

mod blockchainapi;
mod contractapi;
mod error;
mod ledgerapi;
mod runtimeapi;

pub use contract_macros::*;


/// Macro to use in the lib.rs file of your contract's crate
/// 
/// Should be called with the functions that create new instances of 
/// the contract structures. 
/// 
/// # Example
/// 
/// ```
/// fabric_contract::register!( AssetContract::new );
/// fabric_contract::register!( AssetContract::new, AccountsContract::new );
/// ```
#[macro_export]
macro_rules! register {

    (  $( $contract:path ),*   )=> {
        extern crate wapc_guest as guest;
        use guest::prelude::*;

        use fabric_contract::contract::ContractManager;
        use fabric_contract::prelude::*;

        use std::sync::Once;

        static START: Once = Once::new();
        
        pub fn __launch() {
            host_log("__launch");
            $( ContractManager::register_contract(Box::new($contract())); )*
            host_log("__launched");
        }

        pub fn once_wapc(operation: &str, msg: &[u8]) -> CallResult {
            START.call_once(|| {
                 __launch();
            });
            handle_wapc(operation,msg)
        }


        // register the callback handler for the wapc calls
        wapc_handler!(once_wapc);
    };
}


pub mod prelude {   
    pub use crate::runtimeapi::wapc::handle_wapc as handle_wapc;
    pub use crate::runtimeapi::wapc::log as host_log;
    pub use wapc_guest::prelude::*;    
}

pub mod data {
    pub use crate::ledgerapi::datatype::DataType;
    pub use contract_macros::property as Property;
}

/// Module to provide 'runtime' services.
///
/// Services available are
///
/// - Registering the contracts available at start
/// - Logging
///
pub mod runtime {
    // pub use crate::runtimeapi::ContractRuntime;
}

/// Module to provide APIs to write contracts and interact with the ledger
///
/// TODO: Split this into two modules?
///
pub mod contract {
    pub use crate::contractapi::context::Context;
    pub use crate::contractapi::contract::Contract;
    pub use crate::contractapi::contract::Routing;
    pub use crate::contractapi::contractmanager::ContractManager;
    pub use crate::ledgerapi::collection::Collection;
    pub use crate::ledgerapi::collection::CollectionName;
    pub use crate::ledgerapi::ledger::Ledger;
    pub use crate::ledgerapi::state::State;

    pub use contract_macros::contract_impl as Contract_Impl;
    pub use contract_macros::transaction as Transaction;

    pub use crate::error::ContractError as ContractError;
}

/// Module to provide APIs to get information about Fabric
///
/// It is not intended to be full chain access utility but the important
/// aspects required for
///
/// - Current executing transactions
/// - Events to be added to the read/write set of the tranasction
/// - Invoking chaincode on other channels
pub mod blockchain {
    pub use crate::blockchainapi::transaction::Transaction;
}

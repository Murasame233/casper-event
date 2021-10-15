//! [For Event User]
//!
//! use
//! ```
//! set_event_callback("event_name".into(),contract_hash,callback_entry,target_hash);
//! unset_event_callback("event_name".into(),contract_hash,callback_entry,target_hash);
//! ```
extern crate alloc;

use alloc::string::String;
use casper_contract::contract_api::runtime::call_contract;
use casper_types::{runtime_args, ContractHash, RuntimeArgs};

use crate::SET_CALLBACK_ENTRY;

/// [For Event User] When you raedy a entry point for handle event use this.
/// ```
/// set_event_callback("event_name".into(),contract_hash,callback_entry,target_hash);
/// ```
pub fn set_event_callback(
    name: String,
    hash: ContractHash,
    callback_entry: String,
    target_hash: ContractHash,
) {
    call_contract(
        target_hash,
        SET_CALLBACK_ENTRY,
        runtime_args! {
        "name"=>name,
        "target"=> hash.to_formatted_string(),
        "entry" => callback_entry
        },
    )
}

/// [For Event User] When want to unset event callback use this.
/// ```
/// unset_event_callback("event_name".into(),contract_hash,callback_entry,target_hash);
/// ```
pub fn unset_event_callback(
    name: String,
    hash: ContractHash,
    callback_entry: String,
    target_hash: ContractHash,
) {
    call_contract(
        target_hash,
        SET_CALLBACK_ENTRY,
        runtime_args! {
        "name"=>name,
        "target"=> hash.to_formatted_string(),
        "entry" => callback_entry
        },
    )
}

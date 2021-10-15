extern crate alloc;

use alloc::{string::String, vec, vec::Vec};

use casper_contract::{
    contract_api::{
        runtime::{self, call_contract, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, ApiError, CLType, ContractHash, EntryPoint, Key, Parameter, RuntimeArgs,
    URef,
};

use crate::{CALLBACKS_KEY, SET_CALLBACK_ENTRY, UNSET_CALLBACK_ENTRY};

#[repr(u16)]
enum Error {
    NoCallBacksKeyOnRuntime = 0,
    NoCallBacksOnStorage = 1,
    InvalidTargetFormat = 10,
    AlreadySet = 20,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

// Helper functions

#[doc(hidden)]
fn parse_callback_args() -> (ContractHash, String, String) {
    let target_str: String = runtime::get_named_arg("target"); // callback target hash
    if ContractHash::from_formatted_str(&target_str).is_err() {
        revert(Error::InvalidTargetFormat);
    }

    let target = ContractHash::from_formatted_str(&target_str).unwrap();
    let target_entry: String = runtime::get_named_arg("entry"); // callback target entry
    let name: String = runtime::get_named_arg("name"); // event name
    (target, target_entry, name)
}

#[doc(hidden)]
fn get_callbacks_uref() -> URef {
    runtime::get_key(CALLBACKS_KEY)
        .unwrap_or_revert_with(Error::NoCallBacksKeyOnRuntime)
        .into_uref()
        .unwrap()
}

#[doc(hidden)]
fn get_callbacks() -> Vec<(ContractHash, String, String)> {
    let uref = get_callbacks_uref();

    storage::read(uref)
        .unwrap_or_revert_with(Error::NoCallBacksOnStorage)
        .unwrap()
}

#[doc(hidden)]
fn save_callbacks(callbacks: Vec<(ContractHash, String, String)>) {
    let uref = get_callbacks_uref();

    storage::write(uref, callbacks);
}

// End helper functions

// Entries

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn set_event() {
    let callback = parse_callback_args();
    let mut callbacks = get_callbacks();

    if callbacks.iter().any(|f| f == &callback) {
        revert(Error::AlreadySet)
    } else {
        callbacks.push(callback);
        save_callbacks(callbacks);
    }
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn unset_event() {
    let callback = parse_callback_args();
    let mut callbacks = get_callbacks();

    callbacks = callbacks.into_iter().filter(|f| f != &callback).collect();

    save_callbacks(callbacks);
}

// End entris

///[For Event Provider] When you think a event is evoke, send this event to target by using this
///```rust
///send_event("event_name",Some(target),Some(target_entry),runtime_args!{});
///```
///send_event function will auto add "event_name" to args;
pub fn send_event(
    name: String,
    target: Option<ContractHash>,
    target_entry: Option<String>,
    args: RuntimeArgs,
) {
    let mut callbacks = get_callbacks();
    if let Some(hash) = target {
        callbacks = callbacks.into_iter().filter(|p| p.0 == hash).collect();
    }
    if let Some(entry) = target_entry {
        callbacks = callbacks.into_iter().filter(|p| p.1 == entry).collect();
    }
    callbacks = callbacks.into_iter().filter(|p| p.2 == name).collect();

    let args = args;
    for (hash, entry, _) in callbacks {
        let mut args = args.clone();
        if args.get("name").is_none() {
            args.insert("name", name.clone()).unwrap();
        }
        call_contract::<()>(hash, &entry, args);
    }
}

/// [For Event Provider] get event entry for your new version contract
/// ```
/// let mut entry = EntryPoints::new();
/// if let (entry_set: EntryPoint,entry_unset: EntryPoint) = get_set_event_entry(){
///     entry.add_entry_point(entry_set);
///     entry.add_entry_point(entry_unset);
/// }
/// new_contract(entry, _, _, _);
/// ```
pub fn get_set_event_entry() -> (EntryPoint, EntryPoint) {
    (
        EntryPoint::new(
            SET_CALLBACK_ENTRY,
            vec![
                Parameter::new("name", CLType::String),
                Parameter::new("target", CLType::String),
                Parameter::new("entry", CLType::String),
            ],
            CLType::Unit,
            casper_types::EntryPointAccess::Public,
            casper_types::EntryPointType::Contract,
        ),
        EntryPoint::new(
            UNSET_CALLBACK_ENTRY,
            vec![
                Parameter::new("name", CLType::String),
                Parameter::new("target", CLType::String),
                Parameter::new("entry", CLType::String),
            ],
            CLType::Unit,
            casper_types::EntryPointAccess::Public,
            casper_types::EntryPointType::Contract,
        ),
    )
}

/// [For Event Provider] get named_keys stored on runtime for you contract
/// ```
/// let mut named_keys = NamedKeys::new();
/// named_keys.append(&mut get_set_event_named_keys())
/// new_contract(_, named_keys, _, _);
/// ```
pub fn get_set_event_named_keys() -> NamedKeys {
    let mut n: NamedKeys = NamedKeys::new();
    let v: Vec<(ContractHash, String, String)> = vec![];
    let uref = storage::new_uref(v);
    n.insert(CALLBACKS_KEY.into(), Key::URef(uref));
    n
}

# Intro

this project is a lib that makes developer can easily use event on their smart contract.

# how to
add to cargo.toml
```
casper_event = "0.1.0"
```

## Event Provider

```rust
#[no_mangle]
fn call(){
    ...
    let mut entry = EntryPoints::new();
    if let (entry_set: EntryPoint,entry_unset: EntryPoint) = get_set_event_entry(){
        entry.add_entry_point(entry_set);
        entry.add_entry_point(entry_unset);
    }
    let mut named_keys = NamedKeys::new();
    named_keys.append(&mut get_set_event_named_keys())
    new_contract(entry, named_keys, _, _);
    ...
}

// when you have a event to evoke
    ...
    send_event("event_name".into(),Some(target),Some(target_entry),runtime_args!{})
    ...
```

## Event User

```rust
// You need a entry to handle the event
#[no_mangle]
fn handler(){
    ...
}

// set event callback
    ...
    set_event_callback("event_name".into(),contract_hash,"handler".into(),target_hash);
    ...
```
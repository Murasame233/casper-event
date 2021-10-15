#![no_std]

pub mod constant;
#[doc(inline)]
pub use constant::CALLBACKS_KEY;
#[doc(inline)]
pub use constant::SET_CALLBACK_ENTRY;
#[doc(inline)]
pub use constant::UNSET_CALLBACK_ENTRY;

pub mod provider;
#[doc(inline)]
pub use provider::get_set_event_entry;
#[doc(inline)]
pub use provider::get_set_event_named_keys;
#[doc(inline)]
pub use provider::send_event;
pub use provider::set_event;
pub use provider::unset_event;

pub mod user;
#[doc(inline)]
pub use user::set_event_callback;
#[doc(inline)]
pub use user::unset_event_callback;

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.

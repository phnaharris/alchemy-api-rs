

//! #![deny(missing_docs)]
//! #![warn(missing_debug_implementations, rust_2018_idioms, rustdoc::all)]
//! #![allow(rustdoc::private_doc_tests)]
//! #![cfg_attr(docsrs, feature(doc_cfg))]
//! #![allow(unused)]

/// API module, contains list of API supported by Alchemy.
pub mod api;
/// Core module, contains some helpful generic traits for endpoint and request.
pub mod cores;
/// Alchemy client module, contains information about Alchemy connection.
pub mod alchemy;

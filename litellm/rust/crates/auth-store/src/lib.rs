//! Rust stores for the auth objects the proxy reads from Postgres when its auth cache misses.
//!
//! Each store owns its queries, the shaping of their rows into what the auth flow consumes, and any memory
//! tier private to that object. The shared auth cache in front of them stays with the host.

mod budget;
mod database;
mod end_user;
mod jwt_key_mapping;
mod key;
mod model_access_group;
mod user;

pub use budget::BudgetStore;
pub use database::{Database, Side};
pub use end_user::EndUserStore;
pub use jwt_key_mapping::JwtKeyMappingStore;
pub use key::{KeyStore, KeyView};
pub use model_access_group::{ModelAccessGroupBudget, ModelAccessGroupStore};
pub use user::{SignIn, UserStore};

//! Python handles on the Rust auth stores. Each wraps one store built over a `NativeDatabase`'s pools and
//! hands its answers to Python as plain values or JSON text for `model_validate_json`.

mod budget;
mod end_user;
mod jwt_key_mapping;
mod key;
mod model_access_group;
mod user;

pub(crate) use budget::RustBudgetStore;
pub(crate) use end_user::RustEndUserStore;
pub(crate) use jwt_key_mapping::RustJwtKeyMappingStore;
pub(crate) use key::RustKeyStore;
pub(crate) use model_access_group::RustModelAccessGroupStore;
pub(crate) use user::RustUserStore;

fn json<T: serde::Serialize>(value: Option<&T>) -> pyo3::PyResult<Option<String>> {
    value.map(litellm_host_python::json_dumps).transpose()
}

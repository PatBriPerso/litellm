use std::sync::Arc;

use litellm_auth_store::EndUserStore;
use pyo3::prelude::*;

use crate::db::{NativeDatabase, map_error};

/// The end user store. `find_end_user` answers the row with its budget and object permission as JSON text.
#[pyclass(frozen)]
pub(crate) struct RustEndUserStore(Arc<EndUserStore>);

#[pymethods]
impl RustEndUserStore {
    #[new]
    fn new(database: &NativeDatabase) -> PyResult<Self> {
        Ok(Self(Arc::new(EndUserStore::new(
            database.store_database()?,
        ))))
    }

    fn find_end_user<'py>(
        &self,
        py: Python<'py>,
        end_user_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let store = Arc::clone(&self.0);
        crate::execution::run_async_value(py, async move {
            super::json(store.find(&end_user_id).await.map_err(map_error)?.as_ref())
        })
    }

    /// Up to `limit` ids of end users whose row restricts something.
    fn restricted_end_user_ids<'py>(
        &self,
        py: Python<'py>,
        limit: i64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let store = Arc::clone(&self.0);
        crate::execution::run_async_value(py, async move {
            store.restricted_ids(limit).await.map_err(map_error)
        })
    }
}

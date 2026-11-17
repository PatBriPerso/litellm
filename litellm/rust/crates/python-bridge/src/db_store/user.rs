use std::sync::Arc;

use litellm_auth_store::{Side, SignIn, UserStore};
use pyo3::prelude::*;

use crate::db::{NativeDatabase, map_error};

/// The user store. `resolve_user` answers the user a sign-in resolves to, with its organization memberships,
/// as JSON text, or `None`.
#[pyclass(frozen)]
pub(crate) struct RustUserStore(Arc<UserStore>);

#[pymethods]
impl RustUserStore {
    #[new]
    fn new(database: &NativeDatabase) -> PyResult<Self> {
        Ok(Self(Arc::new(UserStore::new(database.store_database()?))))
    }

    fn resolve_user<'py>(
        &self,
        py: Python<'py>,
        #[pyo3(from_py_with = litellm_host_python::from_py_argument)] sign_in: SignIn,
        writer: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let store = Arc::clone(&self.0);
        let side = if writer { Side::Writer } else { Side::Read };
        crate::execution::run_async_value(py, async move {
            super::json(
                store
                    .resolve(&sign_in, side)
                    .await
                    .map_err(map_error)?
                    .as_ref(),
            )
        })
    }

    fn email_belongs_to_another_user<'py>(
        &self,
        py: Python<'py>,
        user_email: String,
        user_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let store = Arc::clone(&self.0);
        crate::execution::run_async_value(py, async move {
            store
                .email_belongs_to_another_user(&user_email, &user_id)
                .await
                .map_err(map_error)
        })
    }

    #[pyo3(signature = (user_id, user_email, *, links_any_user))]
    fn backfill_user_email<'py>(
        &self,
        py: Python<'py>,
        user_id: String,
        user_email: String,
        links_any_user: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let store = Arc::clone(&self.0);
        crate::execution::run_async_value(py, async move {
            store
                .backfill_email(&user_id, &user_email, links_any_user)
                .await
                .map_err(map_error)
        })
    }
}

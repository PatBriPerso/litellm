use std::sync::Arc;

use litellm_auth_store::BudgetStore;
use pyo3::prelude::*;

use crate::db::{NativeDatabase, map_error};

/// The budget store. Each lookup answers the budget row as JSON text, or `None`.
#[pyclass(frozen)]
pub(crate) struct RustBudgetStore(Arc<BudgetStore>);

#[pymethods]
impl RustBudgetStore {
    #[new]
    fn new(database: &NativeDatabase) -> PyResult<Self> {
        Ok(Self(Arc::new(BudgetStore::new(database.store_database()?))))
    }

    fn find_budget<'py>(&self, py: Python<'py>, budget_id: String) -> PyResult<Bound<'py, PyAny>> {
        let store = Arc::clone(&self.0);
        crate::execution::run_async_value(py, async move {
            super::json(store.find(&budget_id).await.map_err(map_error)?.as_ref())
        })
    }

    fn find_team_member_default_budget<'py>(
        &self,
        py: Python<'py>,
        budget_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let store = Arc::clone(&self.0);
        crate::execution::run_async_value(py, async move {
            let budget = store
                .find_team_member_default(&budget_id)
                .await
                .map_err(map_error)?;
            super::json(budget.as_ref())
        })
    }
}

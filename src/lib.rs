use std::sync::RwLock;
use hashbrown::HashMap;
use pyo3::{prelude::*, types::*};

#[pyclass(name="rust_cache")]
struct RustCache {
    cache: RwLock<HashMap<isize, PyObject>>,
    wrap: PyObject
}

#[pymethods]
impl RustCache {
    #[new]
    fn __new__(wrap: PyObject) -> Self {
        RustCache {
            cache: RwLock::new(HashMap::new()),
            wrap
        }
    }

    fn make_key(
        &self,
        args: &Bound<'_, PyTuple>, 
        kwargs: Option<&Bound<'_, PyDict>>
    ) -> PyResult<isize> {
        if let Some(kwargs) = kwargs {
            for item in kwargs.items() {
                args.add(item)?;
            }
        }
        let hash = args.hash()?;
        Ok(hash)
    }

    #[pyo3(signature=(*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<PyObject> {
        let key = self.make_key(args, kwargs)?;
        if let Some(res) = self.cache.read().unwrap().get(&key) {
            return Ok(res.clone_ref(py));
        }
        let res = self.wrap.call(py, args, kwargs)?;
        self.cache.write().unwrap().insert(key, res.clone_ref(py));
        Ok(res)
    }
}

#[pymodule]
pub fn rust_cache(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<RustCache>()?;
    Ok(())
}
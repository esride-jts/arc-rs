extern crate arcrs;

use arcrs::gp;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Creates a new toolbox
#[pyfunction]
fn create_mytoolbox(label: &str, alias: &str) -> PyResult<gp::PyToolbox> {
    let dummy_tool = arcrs::DummyGpTool {
    };

    let pytoolbox_factory = gp::PyToolboxFactory {
    };

    let py_toolbox = pytoolbox_factory.create_toolbox(label, alias, vec![dummy_tool])?;

    Ok(py_toolbox)
}



/// This module allows the implementation of Geoprocessing Tools using Rust.
#[pymodule]
fn copyfeatures(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<gp::PyToolbox>()?;
    module.add_function(wrap_pyfunction!(create_mytoolbox, module)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

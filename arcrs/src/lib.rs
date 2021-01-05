//   Copyright (C) 2021 Jan Tschada (j.tschada@esri.de).
//   
//   This program is free software: you can redistribute it and/or modify
//   it under the terms of the GNU Lesser General Public License as published by
//   the Free Software Foundation, either version 3 of the License, or
//   (at your option) any later version.
//   
//   This program is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU Lesser General Public License for more details.
//   
//   You should have received a copy of the GNU Lesser General Public License
//   along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod gp;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;



/// Creates a new toolbox
#[pyfunction]
fn create_toolbox() -> PyResult<gp::Toolbox> {
    let toolbox = gp::Toolbox {
        label: String::from("Test Toolbox"),
        alias:  String::from("test_rust")
    };

    Ok(toolbox)
}

/// Dummy GP Tool
struct DummyGpTool {

}

impl gp::api::GpTool for DummyGpTool {

    fn name(&self) -> &str {
        "Dummy Tool"
    }

    fn description(&self) -> &str {
        "Dummy tool doing nothing!"
    }

    fn parameters(&self) -> std::vec::Vec<gp::api::GpParameter> { 
        Vec::new()
    }

    fn execute(&self, _: Vec<gp::api::GpParameter>) { 
        
    }
}




/// This module allows the implementation of Geoprocessing Tools using Rust.
#[pymodule]
fn arcrs(_py: Python, module: &PyModule) -> PyResult<()> {

    // Create and initialize the GP tools
    gp::register_tool(Box::new(DummyGpTool {}));

    module.add_class::<gp::Toolbox>()?;
    module.add_function(wrap_pyfunction!(create_toolbox, module)?)?;
    
    Ok(())
}



/// Unit tests for the ArcGIS implementation.
#[cfg(test)]
mod tests {

    use super::gp;

    #[test]
    fn create_toolbox() {
        let toolbox = gp::Toolbox {
            label: String::from("Test Toolbox"),
            alias:  String::from("test_rust"),
            python_tools: Vec::new()
        };

        assert_eq!("Test Toolbox", toolbox.label, "Label is wrong!");
    }
}

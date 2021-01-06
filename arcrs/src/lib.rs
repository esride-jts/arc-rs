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

/// Dummy GP Tool
#[derive(Copy, Clone)]
pub struct DummyGpTool {

}

impl gp::api::GpTool for DummyGpTool {

    fn label(&self) -> &str {
        "Dummy Tool"
    }

    fn description(&self) -> &str {
        "Dummy tool doing nothing!"
    }

    fn parameters(&self) -> std::vec::Vec<gp::api::GpParameter> { 
        Vec::new()
    }

    fn execute(&self, parameters: Vec<gp::api::GpParameter>, messages: gp::api::PyGpMessages) -> PyResult<()> {
        messages.add_message("Hello from Rust!")?;

        Ok(())
    }
}



/// Creates a new toolbox
#[pyfunction]
fn create_toolbox(label: &str, alias: &str) -> PyResult<gp::PyToolbox> {
    // Methods from traits must be known in current scope!
    use gp::api::GpTool;
    let dummy_tool = DummyGpTool {
    };

    let py_tool = gp::PyTool {
        label: dummy_tool.label().to_string(),
        description: dummy_tool.description().to_string(),
        tool_impl: Box::new(dummy_tool)
    };

    let toolbox = gp::PyToolbox { 
        label: label.to_string(),
        alias: alias.to_string(),
        py_tools: vec![py_tool]
    };

    Ok(toolbox)
}



/// This module allows the implementation of Geoprocessing Tools using Rust.
#[pymodule]
fn arcrs(_py: Python, module: &PyModule) -> PyResult<()> {

    // Create and initialize the GP tools
    /*
    static dummy:DummyGpTool = DummyGpTool {
    };
    gp::register_tool(Box::new(dummy));
    */

    module.add_class::<gp::PyToolbox>()?;
    module.add_function(wrap_pyfunction!(create_toolbox, module)?)?;
    
    Ok(())
}



/// Unit tests for the ArcGIS implementation.
#[cfg(test)]
mod tests {

    use super::gp;
    use super::DummyGpTool;

    #[test]
    fn create_toolbox() {
        // Methods from traits must be known in current scope!
        use gp::api::GpTool;
        let dummy_tool = DummyGpTool {
        };

        let py_tool = gp::PyTool {
            label: dummy_tool.label().to_string(),
            description: dummy_tool.description().to_string(),
            tool_impl: Box::new(dummy_tool)
        };

        let toolbox = gp::PyToolbox {
            label: String::from("Test Toolbox"),
            alias:  String::from("test_rust"),
            py_tools: vec![py_tool]
        };

        assert_eq!("Test Toolbox", toolbox.label, "Label is wrong!");
    }
}

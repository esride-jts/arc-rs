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

    fn parameters(&self) -> Vec<gp::api::GpParameter> { 
        vec![gp::api::GpParameter{
            display_name: String::from("Input Features"),
            name: String::from("in_features"),
            data_type: gp::api::DataType::GPFeatureRecordSetLayer,
            parameter_type: gp::api::ParameterType::Required,
            direction: gp::api::Direction::Input
        }]
    }

    fn execute(&self, parameters: Vec<gp::api::PyParameterValue>, messages: gp::api::PyGpMessages) -> PyResult<()> {
        // IntoCursor trait must be in current scope
        use gp::api::IntoCursor;

        messages.add_message("Hello from Rust!")?;

        for gp_parameter in parameters {
            messages.add_message(&gp_parameter.display_name()?)?;
            messages.add_message(&gp_parameter.name()?)?;

            let data_type = gp_parameter.data_type()?;
            match data_type {
                gp::api::DataType::GPFeatureLayer | gp::api::DataType::GPFeatureRecordSetLayer => {
                    // Try to access the fields
                    let fields = gp_parameter.fields()?;
                    for field in fields {
                        messages.add_message(&field.name)?;
                        messages.add_message(field.field_type.as_str())?;
                    }

                    // Shape field name
                    let shape_field_name = gp_parameter.shape_field_name()?;
                    messages.add_message(&shape_field_name)?;

                    // Try to access the features
                    let search_cursor = gp_parameter.into_search_cursor()?;
                    loop {
                        match search_cursor.next() {
                            Ok(next_row) => {
                                for field_index in 0..next_row.value_count() {
                                    let row_value = next_row.value(field_index)?;
                                    messages.add_message(&row_value)?;
                                }
                            },
                            Err(_) =>  break
                        }
                    }
                }
            }
        }

        Ok(())
    }
}



/// Creates a new toolbox
#[pyfunction]
fn create_toolbox(label: &str, alias: &str) -> PyResult<gp::PyToolbox> {
    let dummy_tool = DummyGpTool {
    };

    let pytoolbox_factory = gp::PyToolboxFactory {
    };

    let py_toolbox = pytoolbox_factory.create_toolbox(label, alias, vec![dummy_tool])?;

    Ok(py_toolbox)
}



/// This module allows the implementation of Geoprocessing Tools using Rust.
#[pymodule]
fn arcrs(_py: Python, module: &PyModule) -> PyResult<()> {
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

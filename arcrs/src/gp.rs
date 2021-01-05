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

pub mod api;

use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use std::cell::RefCell;
thread_local!(static REGISTRY: RefCell<Vec<Box<dyn api::GpTool>>> = RefCell::new(Vec::new()));
thread_local!(static PYTHON_REGISTRY: RefCell<Vec<Tool>> = RefCell::new(Vec::new()));



pub struct ToolRegistry<T: api::GpTool> {
    tools: Vec<Tool>,
    gp_tools: Vec<T>
}

impl<T: api::GpTool> ToolRegistry<T> {

    fn get_gp_tool(&self, tool_index: usize) -> Option<&T> {
        if (tool_index < self.gp_tools.len()) {
            return None;
        }

        return Some(&self.gp_tools[tool_index]);
    }
}

pub fn register_tool<T: 'static +  api::GpTool>(gp_tool: Box<T>) {
    let mut tool_index: usize = 0;

    REGISTRY.with(|reg_cell| {
        let mut registry_tools = reg_cell.borrow_mut();
        tool_index = registry_tools.len();

        let new_tool = Tool {
            label: gp_tool.name().to_string(),
            description: gp_tool.description().to_string(),
            tool_index: tool_index
        };
        registry_tools.push(gp_tool);

        PYTHON_REGISTRY.with(|pyreg_cell| {
            let mut pyregistry_tools = pyreg_cell.borrow_mut();
            pyregistry_tools.push(new_tool);
        });
    });

    /*
    let mut tools: Vec<Tool> = Vec::with_capacity(gp_tools.len());
    for index in 0..gp_tools.len() {
        let new_tool = Tool {
            label: String::from("Test tool"),
            description: String::from("A simple test tool ..."),
            tool_index: index
        };

        tools.push(new_tool);
    }

    ToolRegistry {
        tools: tools,
        gp_tools: gp_tools
    }
    */
}



/// Represents a toolbox offering geoprocessing tools.
#[pyclass]
pub struct Toolbox {
    #[pyo3(get)]
    pub label: String,

    #[pyo3(get)]
    pub alias: String
}

#[pymethods]
impl Toolbox {
    
    /// Returns all tools of this toolbox.
    fn tools(&self) -> PyResult<Vec<Tool>> {
        let mut tools = Vec::new();
        
        PYTHON_REGISTRY.with(|pyreg_cell| {
            let pyregistry_tools = pyreg_cell.borrow();
            for index in 0..pyregistry_tools.len() {
                let pytool = &pyregistry_tools[index];
                let tool_copy = Tool {
                    label: pytool.label.as_str().to_string(),
                    description: pytool.description.as_str().to_string(),
                    tool_index: pytool.tool_index
                };
                tools.push(tool_copy);
            }
        });

        Ok(tools)
    }
}



/// Represents the geoprocessing utilities.

/// Creates a default parameter using arcpy.
fn create_default_parameter(py: Python, param: api::GpParameter) -> PyResult<&PyAny> {
    let locals = [("arcpy", py.import("arcpy")?)].into_py_dict(py);
    let parameter = py.eval("arcpy.Parameter()", None, Some(&locals))?;
    parameter.setattr("displayName", param.display_name())?;
    parameter.setattr("name", param.name())?;
    parameter.setattr("dataType", param.data_type().as_str())?;
    parameter.setattr("parameterType", param.parameter_type().as_str())?;
    parameter.setattr("direction", param.direction().as_str())?;

    Ok(parameter)
}

/// Creates a required input paramater of data type features.
fn create_features_input_parameter(py: Python) -> PyResult<PyObject> {
    let parameter = create_default_parameter(py, api::GpParameter {
        display_name: String::from("Input Features"),
        name: String::from("in_features"),
        data_type: api::DataType::GPFeatureLayer,
        parameter_type: api::ParameterType::Required,
        direction: api::Direction::Input
    })?;

    Ok(parameter.to_object(py))
}

/// Creates a derived output paramater of data type features.
fn create_features_output_parameter(py: Python) -> PyResult<PyObject> {
    let parameter = create_default_parameter(py, api::GpParameter { 
        display_name: String::from("Output Features"),
        name: String::from("out_features"),
        data_type: api::DataType::GPFeatureLayer,
        parameter_type: api::ParameterType::Derived,
        direction: api::Direction::Output
    })?;

    Ok(parameter.to_object(py))
}



/// Represents a geoprocessing tool.
#[pyclass]
pub struct Tool {
    #[pyo3(get)]
    pub label: String,

    #[pyo3(get)]
    pub description: String,

    pub tool_index: usize
}

#[pymethods]
impl Tool {

    /// Returns all parameters of this tool.
    fn parameter_info(&self, py: Python) -> PyResult<Vec<PyObject>> {
        let mut parameters: Vec<PyObject> = Vec::new();
        let input_parameter = create_features_input_parameter(py)?;
        parameters.push(input_parameter);

        let output_parameter = create_features_output_parameter(py)?;
        parameters.push(output_parameter);

        Ok(parameters)
    }
    


    /// Executes this tool.
    fn execute(&self, py:Python, parameters: PyObject, messages: PyObject) -> PyResult<()> {
        messages.call_method1(py, "addMessage", (String::from("Starting ..."), ))?;

        Ok(())
    }
}



/// Represents a geoprocessing tool parameter
#[pyclass]
//#[derive(FromPyObject)]
pub struct Parameter {
    #[pyo3(get)]
    //#[pyo3(item("displayName"))]
    pub display_name: String,

    #[pyo3(get)]
    //#[pyo3(item("name"))]
    pub name: String,

    #[pyo3(get)]
    //#[pyo3(item("dataType"))]
    pub data_type: String,

    #[pyo3(get)]
    //#[pyo3(item("parameterType"))]
    pub parameter_type: String,

    #[pyo3(get)]
    //#[pyo3(item("direction"))]
    pub direction: String,

    #[pyo3(get)]
    //#[pyo3(item("valueAsText"))]
    pub value: String
}
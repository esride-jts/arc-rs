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

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

/// Stores the geoprocessing tools for this environment.
use std::cell::RefCell;
thread_local!(static REGISTRY: RefCell<Vec<Box<dyn api::GpTool>>> = RefCell::new(Vec::new()));



/// Registers a new geoprocessing tool for this environment.
pub fn register_tool<T: 'static +  api::GpTool>(gp_tool: Box<T>) {
    let mut tool_index: usize = 0;

    REGISTRY.with(|reg_cell| {
        let mut registry_tools = reg_cell.borrow_mut();
        tool_index = registry_tools.len();
        registry_tools.push(gp_tool);
    });
}



/// Represents a python toolbox offering geoprocessing tools.
#[pyclass]
pub struct PyToolbox {
    #[pyo3(get)]
    pub label: String,

    #[pyo3(get)]
    pub alias: String
}

#[pymethods]
impl PyToolbox {
    
    /// Returns all tools of this toolbox.
    fn tools(&self) -> PyResult<Vec<PyTool>> {
        let mut tools = Vec::new();
        
        REGISTRY.with(|reg_cell| {
            let registry_tools = reg_cell.borrow();
            for tool_index in 0..registry_tools.len() {
                let gp_tool = &registry_tools[tool_index];
                let pytool = PyTool {
                    label: gp_tool.label().to_string(),
                    description: gp_tool.description().to_string(),
                    tool_index: tool_index
                };
                tools.push(pytool);
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

/// Creates arcpy parameters from tool parameters
fn create_arcpy_parameters(py: Python, parameters: Vec<api::GpParameter>) -> PyResult<Vec<PyObject>> {
    let mut py_parameters: Vec<PyObject> = Vec::with_capacity(parameters.len());
    for parameter in parameters {
        let py_parameter = create_default_parameter(py, parameter)?;
        py_parameters.push(py_parameter.to_object(py));
    }

    Ok(py_parameters)
}

/// Creates parameters from an arcpy parameters array
fn create_parameters_from_arcpy(py: Python, py_parameters: PyObject) -> Result<Vec<api::GpParameter>, PyErr> {
    let mut gp_parameters = Vec::new();
    let locals = [("arcpy", py.import("arcpy")?)].into_py_dict(py);
    
    Ok(gp_parameters)
}



/// Represents a Python geoprocessing tool.
#[pyclass]
pub struct PyTool {
    #[pyo3(get)]
    pub label: String,

    #[pyo3(get)]
    pub description: String,

    pub tool_index: usize
}

#[pymethods]
impl PyTool {

    /// Returns all parameters of this tool.
    fn parameter_info(&self, py: Python) -> PyResult<Vec<PyObject>> {
        let py_parameters = REGISTRY.with(|reg_cell| -> PyResult<Vec<PyObject>> {
            let registry_tools = reg_cell.borrow();
            let gp_tool = &registry_tools[self.tool_index];

            let gp_parameters = gp_tool.parameters();
            let py_parameters = create_arcpy_parameters(py, gp_parameters)?;

            Ok(py_parameters)
        })?;

        Ok(py_parameters)
    }
    


    /// Executes this tool.
    fn execute(&self, py:Python, py_parameters: PyObject, py_messages: PyObject) -> PyResult<()> {
        let gp_parameters = create_parameters_from_arcpy(py, py_parameters)?;
        let py_gpmessages = api::PyGpMessages {
            py,
            py_messages
        };

        REGISTRY.with(|reg_cell| -> PyResult<()> {
            let registry_tools = reg_cell.borrow();
            let gp_tool = &registry_tools[self.tool_index];

            gp_tool.execute(gp_parameters, py_gpmessages)?;

            Ok(())
        })?;

        Ok(())
    }
}



/// Represents a Python geoprocessing tool parameter
#[pyclass]
//#[derive(FromPyObject)]
pub struct PyParameter {
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
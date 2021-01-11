//   This file is part of arc-rs and enables the development of Geoprocessing Tools using Rust.
//   Copyright (C) 2021 Esri Deutschland GmbH
//   Contact: Jan Tschada (j.tschada@esri.de)
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
pub mod tools;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;



/// Represents a factory for creating toolboxes.
pub struct PyToolboxFactory {
}

impl PyToolboxFactory {

    /// Creates a new toolbox using the specified geoprocessing tools.
    pub fn create_toolbox<T: 'static + api::GpTool + Send>(&self, label: &str, alias: &str,
        gp_tools: impl IntoIterator<Item = T>) -> PyResult<PyToolbox> {
        let mut py_tools = Vec::new();
        for gp_tool in gp_tools {
            let py_tool = PyTool {
                label: gp_tool.label().to_string(),
                description: gp_tool.description().to_string(),
                tool_impl: Box::new(gp_tool)
            };
            py_tools.push(py_tool);
        }

        let py_toolbox = PyToolbox { 
            label: label.to_string(),
            alias: alias.to_string(),
            py_tools: py_tools
        };

        Ok(py_toolbox)
    }
}



/// Represents a python toolbox offering geoprocessing tools.
#[pyclass]
pub struct PyToolbox {
    #[pyo3(get)]
    pub label: String,

    #[pyo3(get)]
    pub alias: String,

    pub py_tools: Vec<PyTool>
}

#[pymethods]
impl PyToolbox {

    /// Returns the label of the specified tool.
    fn tool_label(&self, tool_index: usize) -> PyResult<String> {
        match self.py_tools.get(tool_index) {
            Some(py_tool) => Ok(py_tool.label.to_string()),
            _ => Err(PyValueError::new_err("Tool index is invalid!"))
        }
    }

    /// Returns the description of the specified tool.
    fn tool_description(&self, tool_index: usize) -> PyResult<String> {
        match self.py_tools.get(tool_index) {
            Some(py_tool) => Ok(py_tool.description.to_string()),
            _ => Err(PyValueError::new_err("Tool index is invalid!"))
        }
    }

    /// Returns the parameter infos for the specified tool
    fn tool_parameter_info(&self, py: Python, tool_index: usize) -> PyResult<Vec<PyObject>> {
        match self.py_tools.get(tool_index) {
            Some(py_tool) => py_tool.parameter_info(py),
            _ => Err(PyValueError::new_err("Tool index is invalid!"))
        }
    }

    /// Executes the specified tool
    fn tool_execute(&self, py: Python, tool_index: usize, py_parameters: Vec<PyObject>, py_messages: PyObject) -> PyResult<()> {
        match self.py_tools.get(tool_index) {
            Some(py_tool) => py_tool.execute(py, py_parameters, py_messages),
            _ => Err(PyValueError::new_err("Tool index is invalid!"))
        }
    }

    /// Returns all tool names of this toolbox.
    fn tools(&self) -> PyResult<Vec<String>> {
        let mut py_tool_names = Vec::with_capacity(self.py_tools.len());
        for py_tool in &self.py_tools {
            py_tool_names.push(py_tool.label.to_string());
        }

        Ok(py_tool_names)
    }
}



/// Represents the geoprocessing utilities.

/// Creates a default parameter using arcpy.
fn create_default_parameter(py: Python, param: api::GpParameter) -> PyResult<&PyAny> {
    let locals = [("arcpy", py.import("arcpy")?)].into_py_dict(py);
    let parameter = py.eval("arcpy.Parameter()", None, Some(&locals))?;
    parameter.setattr("displayName", param.display_name())?;
    parameter.setattr("name", param.name())?;
    parameter.setattr("datatype", param.data_type().as_str())?;
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
fn create_parameters_from_arcpy<'a>(py: &'a Python, py_parameters: Vec<PyObject>) -> Result<Vec<api::PyParameterValue<'a>>, PyErr> {
    let mut pyparameter_values = Vec::with_capacity(py_parameters.len());
    for py_parameter in py_parameters {
        let pyparameter_value = api::PyParameterValue::new(&py, py_parameter);
        pyparameter_values.push(pyparameter_value);
    }
    
    Ok(pyparameter_values)
}



/// Represents a Python geoprocessing tool.
#[pyclass]
pub struct PyTool {
    #[pyo3(get)]
    pub label: String,

    #[pyo3(get)]
    pub description: String,

    pub tool_impl: Box<dyn api::GpTool + Send>
}

#[pymethods]
impl PyTool {
 
    /// Returns all parameters of this tool.
    fn parameter_info(&self, py: Python) -> PyResult<Vec<PyObject>> {
        let gp_parameters = self.tool_impl.parameters();
        let py_parameters = create_arcpy_parameters(py, gp_parameters)?;
        
        Ok(py_parameters)
    }
    


    /// Executes this tool.
    fn execute(&self, py: Python, py_parameters: Vec<PyObject>, py_messages: PyObject) -> PyResult<()> {
        let gp_parameters = create_parameters_from_arcpy(&py, py_parameters)?;
        let py_gpmessages = api::PyGpMessages {
            py: &py,
            py_messages
        };

        self.tool_impl.execute(py, gp_parameters, py_gpmessages)?;

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
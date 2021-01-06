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

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;



/// Dummy GP Tool
#[derive(Copy, Clone)]
struct DummyGpTool {

}

impl api::GpTool for DummyGpTool {

    fn label(&self) -> &str {
        "Dummy Tool"
    }

    fn description(&self) -> &str {
        "Dummy tool doing nothing!"
    }

    fn parameters(&self) -> std::vec::Vec<api::GpParameter> { 
        Vec::new()
    }

    fn execute(&self, parameters: Vec<api::GpParameter>, messages: api::PyGpMessages) -> PyResult<()> {
        messages.add_message("Hello from Rust!")?;

        Ok(())
    }
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
        
        let pytool = PyTool {
            label: String::from("Dummy Tool"),
            description: String::from("Faker"),
            tool_index: 0,
            tool_impl: Box::new(DummyGpTool{

            })
        };
        tools.push(pytool);

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

    pub tool_index: usize,

    tool_impl: Box<dyn api::GpTool + Send>
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
    fn execute(&self, py: Python, py_parameters: PyObject, py_messages: PyObject) -> PyResult<()> {
        let gp_parameters = create_parameters_from_arcpy(py, py_parameters)?;
        let py_gpmessages = api::PyGpMessages {
            py: &py,
            py_messages
        };

        self.tool_impl.execute(gp_parameters, py_gpmessages)?;

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
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

use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;



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
        let test_tool = Tool {
            label: String::from("Test tool"),
            description: String::from("A simple test tool ...")
        };
        tools.push(test_tool);

        Ok(tools)
    }
}



/// Represents all available geoprocessing parameter types.
enum DataType {
    GPFeatureLayer
}

impl DataType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DataType::GPFeatureLayer => "GPFeatureLayer"
        }
    }
}



// Represents all available geoprocessing parameter types.
enum ParameterType {
    Required,
    Optional,
    Derived
}

impl ParameterType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ParameterType::Required => "Required",
            ParameterType::Optional => "Optional",
            ParameterType::Derived => "Derived"
        }
    }
}




// Represents all available geoprocessing parameter directions.
enum Direction {
    Input,
    Output
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Direction::Input => "Input",
            Direction::Output => "Output"
        }
    }
}

/// Represents the geoprocessing utilities.

/// Creates a default parameter using arcpy.
fn create_default_parameter(py: Python, display_name: String, name: String,
    data_type: DataType, parameter_type: ParameterType, direction: Direction) -> PyResult<&PyAny> {
    let locals = [("arcpy", py.import("arcpy")?)].into_py_dict(py);
    let parameter = py.eval("arcpy.Parameter()", None, Some(&locals))?;
    parameter.setattr("displayName", display_name)?;
    parameter.setattr("name", name)?;
    parameter.setattr("dataType", data_type.as_str())?;
    parameter.setattr("parameterType", parameter_type.as_str())?;
    parameter.setattr("direction", direction.as_str())?;

    Ok(parameter)
}

/// Creates a required input paramater of data type features.
fn create_features_input_parameter(py: Python) -> PyResult<PyObject> {
    let parameter = create_default_parameter(py, 
        String::from("Input Features"),
        String::from("in_features"),
        DataType::GPFeatureLayer,
        ParameterType::Required,
        Direction::Input
    )?;

    Ok(parameter.to_object(py))
}

/// Creates a derived output paramater of data type features.
fn create_features_output_parameter(py: Python) -> PyResult<PyObject> {
    let parameter = create_default_parameter(py, 
        String::from("Output Features"),
        String::from("out_features"),
        DataType::GPFeatureLayer,
        ParameterType::Derived,
        Direction::Output
    )?;

    Ok(parameter.to_object(py))
}



/// Represents a geoprocessing tool.
#[pyclass]
pub struct Tool {
    #[pyo3(get)]
    pub label: String,

    #[pyo3(get)]
    pub description: String
}

#[pymethods]
impl Tool {

    /// Returns all parameters of this tool.
    fn getParameterInfo(&self, py: Python) -> PyResult<Vec<PyObject>> {
        let mut parameters: Vec<PyObject> = Vec::new();
        let input_parameter = create_features_input_parameter(py)?;
        parameters.push(input_parameter);

        let output_parameter = create_features_output_parameter(py)?;
        parameters.push(output_parameter);

        Ok(parameters)
    }
    


    /// Executes this tool.
    fn execute(&self, py:Python, parameters: PyObject, messages: PyObject) -> PyResult<()> {
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
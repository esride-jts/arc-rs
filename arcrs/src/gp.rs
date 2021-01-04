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

    fn getParameterInfo(&self, py: Python) -> PyResult<Vec<PyObject>> {
        let mut parameters = Vec::new();

        let arcpy = PyModule::from_code(py, r#"
import arcpy

def createParam():
    return arcpy.Parameter()
        "#, "arcpy_utils.py", "arcpy_utils")?;

        for _index in 0..10 {
            let result = arcpy.call1("createParam", ())?;
            parameters.push(PyObject::from(result));
        }

        Ok(parameters)
    }

    /// Returns all parameters of this tool.
    fn parameter_info(&self) -> PyResult<Vec<Parameter>> {
        let mut parameters = Vec::new();
        let input_param = Parameter {
            display_name: String::from("Input Features"),
            name: String::from("in_features"),
            data_type: String::from("GPFeatureLayer"),
            parameter_type: String::from("Required"),
            direction: String::from("Input"),
            value: String::from("")
        };
        parameters.push(input_param);
        
        let output_param = Parameter {
            display_name: String::from("Output Features"),
            name: String::from("out_features"),
            data_type: String::from("GPFeatureLayer"),
            parameter_type: String::from("Derived"),
            direction: String::from("Output"),
            value: String::from("")
        };
        parameters.push(output_param);

        Ok(parameters)
    }

    /// Executes this tool.
    fn execute(&self, parameters: Vec<HashMap<String, String>>) -> PyResult<Vec<Parameter>> {
        let mut input_value: &String;
        let mut output_value: &String;
        for parameter in parameters {
            let parameter_entry = parameter.get("direction");
            match parameter_entry {
                Some(direction) => {
                    let parameter_entry = parameter.get("value");
                    match parameter_entry {
                        Some(value) => {
                            if "Input" == direction {
                                input_value = value;
                            }
                            else if "Output" == direction {
                                output_value = value;
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }
        }

        let mut parameters = Vec::new();
        let output_param = Parameter {
            display_name: String::from("Output Features"),
            name: String::from("out_features"),
            data_type: String::from("GPFeatureLayer"),
            parameter_type: String::from("Derived"),
            direction: String::from("Output"),
            value: String::from("New features ...")
        };
        parameters.push(output_param);

        Ok(parameters)
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
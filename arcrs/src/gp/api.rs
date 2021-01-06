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

use pyo3::prelude::*;

/// Represents all available geoprocessing parameter types.
pub enum DataType {
    GPFeatureLayer
}

impl DataType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DataType::GPFeatureLayer => "GPFeatureLayer"
        }
    }
}



/// Represents all available geoprocessing parameter types.
pub enum ParameterType {
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




/// Represents all available geoprocessing parameter directions.
pub enum Direction {
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


/// Defines a geoprocessing parameter.
pub struct GpParameter {
    pub display_name: String,
    pub name: String,
    pub data_type: DataType,
    pub parameter_type: ParameterType,
    pub direction: Direction
}

impl GpParameter {

    pub fn display_name(&self) -> &str {
        return &self.display_name;
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn data_type(&self) -> &DataType {
        return &self.data_type;
    }

    pub fn parameter_type(&self) -> &ParameterType {
        return &self.parameter_type;
    }

    pub fn direction(&self) -> &Direction {
        return &self.direction;
    }
}



/// Represents the Python geoprocessing messages environment.
pub struct PyGpMessages<'a> {
    pub py: Python<'a>,
    pub py_messages: PyObject
}

impl PyGpMessages<'_> {
    
    pub fn add_message(&self, message: &str) -> PyResult<()> {
        self.py_messages.call_method1(self.py, "addMessage", (message.to_string(), ))?;

        Ok(())
    }

}



/// Offers the functionalities of a geoprocessing tool
pub trait GpTool {

    fn label(&self) -> &str;

    fn description(&self) -> &str;

    fn parameters(&self) -> Vec<GpParameter>;

    fn execute(&self, parameters: Vec<GpParameter>, messages: PyGpMessages) -> PyResult<()>;
}
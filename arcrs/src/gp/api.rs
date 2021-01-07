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
use std::str::FromStr;

/// Represents all available geoprocessing parameter types.
pub enum DataType {
    GPFeatureLayer,
    GPFeatureRecordSetLayer
}

impl DataType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DataType::GPFeatureLayer => "GPFeatureLayer",
            DataType::GPFeatureRecordSetLayer => "GPFeatureRecordSetLayer"
        }
    }
}

impl FromStr for DataType {

    type Err = ();

    fn from_str(data_type_str: &str) -> Result<DataType, Self::Err> {
        match data_type_str {
            "GPFeatureLayer" => Ok(DataType::GPFeatureLayer),
            "GPFeatureRecordSetLayer" => Ok(DataType::GPFeatureRecordSetLayer),
            "Feature-Set" => Ok(DataType::GPFeatureRecordSetLayer),
            //_ => Err(())
            _ => todo!()
            //_ => unimplemented!()
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

impl FromStr for ParameterType {
    type Err = ();

    fn from_str(parameter_type_str: &str) -> Result<ParameterType, Self::Err> {
        match parameter_type_str {
            "Required" => Ok(ParameterType::Required),
            "Optional" => Ok(ParameterType::Optional),
            "Derived" => Ok(ParameterType::Derived),
            _ => Err(())
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

impl FromStr for Direction {

    type Err = ();

    fn from_str(direction_str: &str) -> Result<Direction, Self::Err> {
        match direction_str {
            "Input" => Ok(Direction::Input),
            "Output" => Ok(Direction::Output),
            _ => Err(())
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

// Represents a builder for a geoprocessing parameter
pub struct GpParameterBuilder {
    display_name: String,
    name: String,
    data_type: DataType,
    parameter_type: ParameterType,
    direction: Direction
}

impl GpParameterBuilder {

    pub fn new() -> GpParameterBuilder {
        GpParameterBuilder {
            display_name: String::from(""),
            name: String::from(""),
            data_type: DataType::GPFeatureLayer,
            parameter_type: ParameterType::Optional,
            direction: Direction::Input
        }
    }

    pub fn with_display_name(mut self, display_name: &str) -> GpParameterBuilder {
        self.display_name = display_name.to_owned();
        self
    }

    pub fn with_name(mut self, name: &str) -> GpParameterBuilder {
        self.name = name.to_owned();
        self
    }

    pub fn with_data_type(mut self, data_type: DataType) -> GpParameterBuilder {
        self.data_type = data_type;
        self
    }

    pub fn with_parameter_type(mut self, parameter_type: ParameterType) -> GpParameterBuilder {
        self.parameter_type = parameter_type;
        self
    }

    pub fn with_direction(mut self, direction: Direction) -> GpParameterBuilder {
        self.direction = direction;
        self
    }

    pub fn build(self) -> GpParameter {
        GpParameter {
            display_name: self.display_name,
            name: self.name,
            data_type: self.data_type,
            parameter_type: self.parameter_type,
            direction: self.direction
        }
    }
}



/// Represents a geoprocessing parameter having a value.
/// Extracts the value out of the underlying parameter.
/// A parameter can contain various values like primitive type (Double, String ...)
/// or complex types like path to a feature class being hosted in a file geodatabase.
pub struct PyParameterValue<'a> {
    py: &'a Python<'a>,
    py_parameter: PyObject
}

impl PyParameterValue<'_> {

    pub fn new<'a>(py: &'a Python, py_parameter: PyObject) -> PyParameterValue<'a> {
        PyParameterValue {
            py,
            py_parameter
        }
    }

    pub fn data_type(&self) -> PyResult<DataType> {
        let pydata_type = self.py_parameter.getattr(*self.py, "datatype")?;
        let data_type_as_text: &str = pydata_type.extract(*self.py)?;
        match DataType::from_str(data_type_as_text) {
            Ok(data_type) => Ok(data_type),
            _ => todo!()
        }        
    }

    /// Extracts the catalog path out of this parameter.
    /// The parameter must represent a feature layer or feature set.
    pub fn catalog_path(&self) -> PyResult<String> {
        let arcpy = PyModule::import(*self.py, "arcpy")?;
        let pyparameter_describe = arcpy.call1("Describe", (&self.py_parameter,))?;
        let pycatalog_path = pyparameter_describe.getattr("catalogPath")?;
        let catalog_path_as_text: String = pycatalog_path.extract()?;
        
        Ok(catalog_path_as_text)
    }

    pub fn value_as_text(&self) -> PyResult<String> {
        let pyvalue_as_text = self.py_parameter.getattr(*self.py, "valueAsText")?;
        let value_as_text: String = pyvalue_as_text.extract(*self.py)?;

        Ok(value_as_text)
    }

    pub fn value(&self) -> PyResult<PyObject> {
        let pyvalue = self.py_parameter.getattr(*self.py, "value")?;
        
        Ok(pyvalue)
    }
}



/// Represents the Python geoprocessing messages environment.
pub struct PyGpMessages<'a> {
    pub py: &'a Python<'a>,
    pub py_messages: PyObject
}

impl PyGpMessages<'_> {
    
    pub fn add_message(&self, message: &str) -> PyResult<()> {
        self.py_messages.call_method1(*self.py, "addMessage", (message.to_string(), ))?;

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
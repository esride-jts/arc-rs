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

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pyclass::PyClass;
use pyo3::types::PyTuple;
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
            _ => todo!("DataType")
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

    pub fn name(&self) -> PyResult<String> {
        let pyname = self.py_parameter.getattr(*self.py, "name")?;
        let name = pyname.extract(*self.py)?;

        Ok(name)
    }

    pub fn display_name(&self) -> PyResult<String> {
        let pydisplay_name = self.py_parameter.getattr(*self.py, "displayName")?;
        let display_name = pydisplay_name.extract(*self.py)?;

        Ok(display_name)
    }

    pub fn data_type(&self) -> PyResult<DataType> {
        let pydata_type = self.py_parameter.getattr(*self.py, "datatype")?;
        let data_type_as_text: &str = pydata_type.extract(*self.py)?;
        match DataType::from_str(data_type_as_text) {
            Ok(data_type) => Ok(data_type),
            _ => todo!("DataType")
        }        
    }

    /// Extracts the catalog path out of this parameter.
    /// The parameter must represent a table or record set.
    pub fn catalog_path(&self) -> PyResult<String> {
        let arcpy = PyModule::import(*self.py, "arcpy")?;
        let pyparameter_describe = arcpy.call1("Describe", (&self.py_parameter,))?;
        let pycatalog_path = pyparameter_describe.getattr("catalogPath")?;
        let catalog_path_as_text: String = pycatalog_path.extract()?;
        
        Ok(catalog_path_as_text)
    }

    /// Extracts the fields out of this paramater.
    /// The parameter must represent a table of record set.
    pub fn fields(&self) -> PyResult<Vec<GpField>> {
        let arcpy = PyModule::import(*self.py, "arcpy")?;
        let pyvalue_describe = arcpy.call1("Describe", (self.value()?,))?;
        let pyfields = pyvalue_describe.getattr("fields")?;
        let fields: Vec<&PyAny> = pyfields.extract()?;
        let mut gp_fields = Vec::with_capacity(fields.len());
        for pyfield in fields {
            let field_name: String = pyfield.getattr("name")?.extract()?;
            let field_type_as_text = pyfield.getattr("type")?.extract()?;
            match FieldType::from_str(field_type_as_text) {
                Ok(field_type) => {
                    let gp_field = GpField {
                        name: field_name,
                        field_type
                    };
                    gp_fields.push(gp_field);
                }
                _ => todo!("FieldType")
            }
        }

        Ok(gp_fields)
    }

    /// Extracts the name of the shape field out of this parameter.
    /// The parameter must represent a feature layer of feature set.
    pub fn shape_field_name(&self) -> PyResult<String> {
        let arcpy = PyModule::import(*self.py, "arcpy")?;
        let pyvalue_describe = arcpy.call1("Describe", (self.value()?,))?;
        let shape_field_name = pyvalue_describe.getattr("shapeFieldName")?.extract()?;

        Ok(shape_field_name)
    }

    /// Extracts the shape type of the shape field out of this parameter.
    /// The parameter must represent a feature layer of feature set.
    pub fn shape_type(&self) -> PyResult<ShapeType> {
        let arcpy = PyModule::import(*self.py, "arcpy")?;
        let pyvalue_describe = arcpy.call1("Describe", (self.value()?,))?;
        let shape_type_as_text: String = pyvalue_describe.getattr("shapeType")?.extract()?;
        match ShapeType::from_str(&shape_type_as_text) {
            Ok(shape_type) => Ok(shape_type),
            _ => todo!("ShapeType")
        }
    }

    /// Extracts the spatial reference out of this parameter.
    /// The parameter must represent a feature layer of feature set.
    pub fn spatial_reference(&self) -> PyResult<GpSpatialReference> {
        let arcpy = PyModule::import(*self.py, "arcpy")?;
        let pyvalue_describe = arcpy.call1("Describe", (self.value()?,))?;
        let pyspatial_reference = pyvalue_describe.getattr("spatialReference")?;
        let wkid = pyspatial_reference.getattr("factoryCode")?.extract()?;
        let spatial_reference = GpSpatialReference {
            wkid
        };
        
        Ok(spatial_reference)
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

/// Implements the conversion from catalog path into a search cursor.
impl IntoCursor for PyParameterValue<'_> {
    
    fn into_search_cursor(&self) -> PyResult<PySearchCursor> {
        let search_cursor = PySearchCursor::new(self.py, &self.catalog_path()?, vec!["*".to_string()], "1=1")?;

        Ok(search_cursor)
    }

}



/// Represents a field returned by arcpy.Describe or arcpy.ListFields.
pub struct GpField {
    pub name: String,
    pub field_type: FieldType
}



/// Represents all known field types.
pub enum FieldType {
    OID,
    Geometry,
    Date,
    Double,
    Integer,
    String
}

impl FieldType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            FieldType::OID => "OID",
            FieldType::Geometry => "Geometry",
            FieldType::Date => "Date",
            FieldType::Double => "Double",
            FieldType::Integer => "Integer",
            FieldType::String => "String"
        }
    }
}

impl FromStr for FieldType {

    type Err = ();

    fn from_str(direction_str: &str) -> Result<FieldType, Self::Err> {
        match direction_str {
            "OID" => Ok(FieldType::OID),
            "Geometry" => Ok(FieldType::Geometry),
            "Date" => Ok(FieldType::Date),
            "Double" => Ok(FieldType::Double),
            "Integer" => Ok(FieldType::Integer),
            "String" => Ok(FieldType::String),
            _ => Err(())
        }
    }
}



/// Represents all known shape types.
pub enum ShapeType {
    Point,
    Polyline,
    Polygon,
    Multipoint
}

impl ShapeType {

    pub fn as_str(&self) -> &'static str {
        match *self {
            ShapeType::Point => "POINT",
            ShapeType::Polyline => "POLYLINE",
            ShapeType::Polygon => "POLYGON",
            ShapeType::Multipoint => "MULTIPOINT"
        }
    }
}

impl FromStr for ShapeType {

    type Err = ();

    fn from_str(shape_type_str: &str) -> Result<ShapeType, Self::Err> {
        match shape_type_str {
            "Point" | "POINT" => Ok(ShapeType::Point),
            "Polyline" | "POLYLINE" => Ok(ShapeType::Polyline),
            "Polygon" | "POLYGON" => Ok(ShapeType::Polygon),
            "Multipoint" | "MULTIPOINT" => Ok(ShapeType::Multipoint),
            _ => Err(())
        }
    }
}



/// Represents a spatial reference.
pub struct GpSpatialReference {
    pub wkid: i32
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



/// Represents a search cursor.
pub struct PySearchCursor<'a> {
    pub py: &'a Python<'a>,
    pub py_cursor: &'a PyAny
}

impl PySearchCursor<'_> {

    pub fn new<'a>(py: &'a Python, catalog_path: &str, field_names: Vec<String>, where_clause: &str) -> PyResult<PySearchCursor<'a>> {
        let arcpy_da = PyModule::import(*py, "arcpy.da")?;
        let py_cursor = arcpy_da.call1("SearchCursor", (catalog_path, field_names, where_clause))?;

        let new_instance = PySearchCursor {
            py,
            py_cursor
        };

        Ok(new_instance)
    }

    pub fn next(&self) -> PyResult<PyRow> {
        let row_values = self.py_cursor.call_method0("next")?.extract()?;
        let row = PyRow::new(self.py, row_values);

        Ok(row)
    }
}



/// Represents a row from a cursor.
pub struct PyRow<'a> {
    pub py: &'a Python<'a>,
    pub py_values: Vec<PyObject>
}

impl PyRow<'_> {

    pub fn new<'a>(py: &'a Python, py_values: Vec<PyObject>) -> PyRow<'a> {
        PyRow {
            py,
            py_values
        }
    }

    pub fn value(&self, index: usize) -> PyResult<String> {
        match &self.py_values.get(index) {
            Some(pytuple) => {
                let any: PyObject = pytuple.extract(*self.py)?;

                Ok(any.to_string())
            },
            _ => Err(PyValueError::new_err("Failed to access the row value!"))
        }
    }

    pub fn as_intvalue(&self, index: usize) -> PyResult<i32> {
        match &self.py_values.get(index) {
            Some(pytuple) => {
                let value: i32 = pytuple.extract(*self.py)?;
                
                Ok(value)
            },
            _ => Err(PyValueError::new_err("Failed to access the row value!"))
        }
    }

    pub fn as_doublevalue(&self, index: usize) -> PyResult<f64> {
        match &self.py_values.get(index) {
            Some(pytuple) => {
                let value: f64 = pytuple.extract(*self.py)?;
                
                Ok(value)
            },
            _ => Err(PyValueError::new_err("Failed to access the row value!"))
        }
    }

    pub fn value_count(&self) -> usize {
        self.py_values.len()
    }
}



/// Offers access to the underlying features by offering a cursor.
pub trait IntoCursor {

    fn into_search_cursor(&self) -> PyResult<PySearchCursor>;
}



/// Offers the functionalities of a geoprocessing tool
pub trait GpTool {

    fn label(&self) -> &str;

    fn description(&self) -> &str;

    fn parameters(&self) -> Vec<GpParameter>;

    fn execute(&self, parameters: Vec<PyParameterValue>, messages: PyGpMessages) -> PyResult<()>;
}
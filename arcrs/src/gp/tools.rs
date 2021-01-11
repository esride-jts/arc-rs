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

use super::api;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyList, PyTuple};
use pyo3::prelude::*;

/// Represents a result from a geoprocessing tool.
pub struct GpResult {
    results: Vec<PyObject>
}

impl GpResult {

    pub fn first_as_str(&self, py: Python) -> PyResult<String> {
        match self.results.get(0) {
            Some(first) => Ok(first.extract(py)?),
            None => Err(PyValueError::new_err("No first result!"))
        }
    }

    pub fn as_vecstr(&self) -> Vec<String> {
        let mut results_as_text = Vec::with_capacity(self.results.len());
        for result in &self.results {
            results_as_text.push(result.to_string());
        }
        results_as_text
    }
}



/// Offers the execution of a geoprocessing tool.
pub trait GpToolExecute {

    fn execute(&self, py: Python) -> PyResult<GpResult>;
}

/// Represents a geoprocessing tool for creating a new feature class
pub struct GpCreateFeatureClassTool {
    out_path: String,
    out_name: String,
    geometry_type: api::ShapeType,
    wkid: i32
}

impl GpCreateFeatureClassTool {

    pub fn new(out_path: String, out_name: String, geometry_type: api::ShapeType, wkid: i32) -> GpCreateFeatureClassTool {
        GpCreateFeatureClassTool {
            out_path,
            out_name,
            geometry_type,
            wkid
        }
    }
}

impl GpToolExecute for GpCreateFeatureClassTool {

    fn execute(&self, py: Python) -> PyResult<GpResult> {
        let arcpy_management = PyModule::import(py, "arcpy.management")?;
        let arguments = (&self.out_path, &self.out_name, self.geometry_type.as_str(), (), (), (), self.wkid);
        let pyresult = arcpy_management.call1("CreateFeatureclass", arguments)?;
        let results = pyresult.extract()?;
        let gp_result = GpResult {
            results
        };

        Ok(gp_result)
    }
}



/// Represents a geoprocessing tool for adding fields to an existing table.
pub struct GpAddFieldsTool {
    catalog_path: String,
    fields: Vec<api::GpField>
}

impl GpAddFieldsTool {

    pub fn new(catalog_path: String, fields: Vec<api::GpField>) -> GpAddFieldsTool {
        GpAddFieldsTool {
            catalog_path,
            fields
        }
    }
}

impl GpToolExecute for GpAddFieldsTool {

    fn execute(&self, py: Python) -> PyResult<GpResult> {
        let arcpy_management = PyModule::import(py, "arcpy.management")?;
        let mut fields_argument: Vec<&PyList> = Vec::with_capacity(self.fields.len());
        for field in &self.fields {
            let pyfield = PyList::new(py, vec![&field.name, &field.field_type.as_gpstr().to_string(), &field.name]);
            fields_argument.push(pyfield);
        }

        let arguments = (&self.catalog_path, fields_argument);
        let pyresult = arcpy_management.call1("AddFields", arguments)?;
        let results = pyresult.extract()?;
        let gp_result = GpResult {
            results
        };

        Ok(gp_result)
    }
}
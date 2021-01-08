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

use super::api;
use pyo3::prelude::*;

/// Offers the execution of a geoprocessing tool.
pub trait GpToolExecute {

    fn execute(&self, py: Python) -> PyResult<()>;
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

    fn execute(&self, py: Python) -> PyResult<()> {
        let arcpy_management = PyModule::import(py, "arcpy.management")?;
        let arguments = (&self.out_path, &self.out_name, self.geometry_type.as_str(), (), (), (), self.wkid);
        let pytool = arcpy_management.call1("CreateFeatureclass", arguments)?;

        Ok(())
    }
}
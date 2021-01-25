extern crate arcrs;

use arcrs::gp;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::path::Path;

/// Copy features Tool
pub struct CopyFeaturesTool {

}

impl gp::api::GpTool for CopyFeaturesTool {
    
    fn label(&self) -> &str { "Copy features" }
    
    fn description(&self) -> &str { "Copies features from one feature class into another." }

    fn parameters(&self) -> Vec<gp::api::GpParameter> { 
        vec![gp::api::GpParameter{
            display_name: String::from("Input Features"),
            name: String::from("in_features"),
            data_type: gp::api::DataType::GPFeatureRecordSetLayer,
            parameter_type: gp::api::ParameterType::Required,
            direction: gp::api::Direction::Input
        }, gp::api::GpParameter{
            display_name: String::from("Output Features"),
            name: String::from("out_features"),
            data_type: gp::api::DataType::DEFeatureClass,
            parameter_type: gp::api::ParameterType::Required,
            direction: gp::api::Direction::Output
        }]
    }
    
    fn execute(&self, py: Python, parameters: Vec<gp::api::PyParameterValue>, messages: gp::api::PyGpMessages) -> PyResult<()> {
        messages.add_message("Copy features...")?;

        let input_param: &gp::api::PyParameterValue = &parameters[0];
        
        // Try to access the fields
        let fields = input_param.fields()?;
        let mut attribute_fields = Vec::with_capacity(fields.len());
        let mut input_field_names = vec!["SHAPE@".to_string()];
        let mut output_field_names = vec!["SHAPE@".to_string()];
        for field in &fields {
            if input_param.oid_field_name()? != field.name 
            && input_param.shape_field_name()? != field.name {
                let attribute_field = gp::api::GpField {
                    name: field.name.to_string(),
                    field_type: field.field_type
                };
                attribute_fields.push(attribute_field);
                
                input_field_names.push(field.name.to_string());
                output_field_names.push(field.name.to_string());
            }
        }

        let output_param = &parameters[1];

        // Get the output path
        let output_path = output_param.catalog_path()?;
        let file_path = Path::new(&output_path);
        let gdb_path = file_path.parent().unwrap().to_str().unwrap().to_string();
        let table_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        // Create a new feature class
        use gp::tools::GpToolExecute;
        let shape_type = input_param.shape_type()?;
        let spatial_reference = input_param.spatial_reference()?;
        let wkid = spatial_reference.wkid;
        let create_tool = gp::tools::GpCreateFeatureClassTool::new(gdb_path, table_name, shape_type, wkid);
        match create_tool.execute(py) {
            Ok(gp_result) => {
                // Try to access the catalog path from the geoprocessing result
                let catalog_path = gp_result.first_as_str(py)?;
                let fields_tool = gp::tools::GpAddFieldsTool::new(catalog_path, attribute_fields);
                match fields_tool.execute(py) {
                    Ok(_) => {
                        // Try to access the features
                        // The API traits must be in the current scope
                        use gp::api::IntoCursor;
                        let where_clause = "1=1";
                        let search_cursor = input_param.into_search_cursor(input_field_names, where_clause)?;
                        let field_count = output_field_names.len();
                        let insert_cursor = output_param.into_insert_cursor(output_field_names)?;
                        loop {
                            match search_cursor.next_row() {
                                Ok(next_row) => {
                                    // Insert the row
                                    insert_cursor.insert_row(next_row)?;
                                },
                                Err (_) => break
                            }

                            /*
                            match search_cursor.next() {
                                Ok(next_row) => {
                                    // Fill the feature buffer
                                    let mut feature_buffer = gp::api::InsertBuffer::new(field_count);
                                    for field_index in 0..next_row.value_count() {
                                        let row_value: PyObject = next_row.value(field_index)?;
                                        feature_buffer.add_value(py, &row_value);
                                    }
                                        
                                    insert_cursor.insert(feature_buffer)?;
                                },
                                Err(_) => break
                            }
                            */
                        }
                    }
                    Err(py_err) => Err(py_err)?
                }
            }
            Err(py_err) => Err(py_err)?
        }

        messages.add_message("Copy features done.")?;

        Ok(())
    }
}




/// Dummy GP Tool
#[derive(Copy, Clone)]
pub struct DummyGpTool {

}

impl gp::api::GpTool for DummyGpTool {

    fn label(&self) -> &str {
        "Dummy Tool"
    }

    fn description(&self) -> &str {
        "Dummy tool doing nothing!"
    }

    fn parameters(&self) -> Vec<gp::api::GpParameter> { 
        vec![gp::api::GpParameter{
            display_name: String::from("Input Features"),
            name: String::from("in_features"),
            data_type: gp::api::DataType::GPFeatureRecordSetLayer,
            parameter_type: gp::api::ParameterType::Required,
            direction: gp::api::Direction::Input
        }, gp::api::GpParameter{
            display_name: String::from("Output Features"),
            name: String::from("out_features"),
            data_type: gp::api::DataType::DEFeatureClass,
            parameter_type: gp::api::ParameterType::Required,
            direction: gp::api::Direction::Output
        }]
    }

    fn execute(&self, py: Python, parameters: Vec<gp::api::PyParameterValue>, messages: gp::api::PyGpMessages) -> PyResult<()> {
        // The API traits must be in the current scope
        use gp::api::{GeometryFromValues, IntoCursor};

        messages.add_message("Hello from Rust!")?;

        // Call any tool
        let pyresult = gp::tools::execute_tool(py, "arcpy", "ListFeatureClasses", ())?;
        let results_as_text = pyresult.as_vecstr();
        for result_as_text in results_as_text {
            messages.add_message(&result_as_text)?;
        }

        let mut out_features_parmeter: Option<gp::api::PyParameterValue> = None;

        for gp_parameter in parameters {
            messages.add_message(&gp_parameter.display_name()?)?;
            messages.add_message(&gp_parameter.name()?)?;
            messages.add_message(&gp_parameter.data_type_as_str()?)?;

            let data_type = gp_parameter.data_type()?;
            match data_type {
                gp::api::DataType::DEFeatureClass |
                gp::api::DataType::GPFeatureLayer | 
                gp::api::DataType::GPFeatureRecordSetLayer => {

                    // Check whether the dataset exists
                    if gp_parameter.path_exists()? {
                        // OID field name
                        let oid_field_name = gp_parameter.oid_field_name()?;
                        messages.add_message(&oid_field_name)?;

                        // Shape field name
                        let shape_field_name = gp_parameter.shape_field_name()?;
                        messages.add_message(&shape_field_name)?;

                        // Shape type
                        let shape_type = gp_parameter.shape_type()?;
                        messages.add_message(shape_type.as_str())?;

                        // Try to access the spatial reference
                        let spatial_reference = gp_parameter.spatial_reference()?;
                        messages.add_message(&spatial_reference.wkid.to_string())?;

                        // Try to access the fields
                        let fields = gp_parameter.fields()?;
                        let mut attribute_field_names = Vec::with_capacity(fields.len());
                        for field in fields {
                            messages.add_message(&field.name)?;
                            messages.add_message(field.field_type.as_str())?;

                            if oid_field_name != field.name 
                            && shape_field_name != field.name {
                                attribute_field_names.push(field.name);
                            }
                        }

                        // Try to access the features
                        let mut field_names = vec![oid_field_name, "SHAPE@".to_string()];
                        field_names.append(&mut attribute_field_names);
                        let where_clause = "1=1";
                        let search_cursor = gp_parameter.into_search_cursor(field_names, where_clause)?;
                        loop {
                            match search_cursor.next() {
                                Ok(next_row) => {
                                    // Try to access OID
                                    let oid: i32 = next_row.as_intvalue(0)?;
                                    messages.add_message(&oid.to_string())?;

                                    // Try to access the geometry instance
                                    let geometry_as_json = next_row.to_geometry_as_json(1)?;
                                    messages.add_message(&geometry_as_json)?;
                                    
                                    // Try to extract a point from the geometry instance
                                    let point: gp::api::Point = next_row.value(1)?;
                                    messages.add_message("Next point...")?;
                                    messages.add_message(&point.to_string())?;

                                    for field_index in 2..next_row.value_count() {
                                        let row_value = next_row.as_strvalue(field_index)?;
                                        messages.add_message(&row_value)?;
                                    }
                                },
                                Err(_) =>  break
                            }
                        }
                    } else {
                        messages.add_message("Dataset does not exists!")?;

                        // Check for output parameter type
                        out_features_parmeter = Some(gp_parameter);
                    }
                }
            }
        }

        // Create a insert cursor
        match out_features_parmeter {
            Some(gp_param) => {

                // Get the output path
                let output_path = gp_param.catalog_path()?;
                let file_path = Path::new(&output_path);
                let gdb_path = file_path.parent().unwrap().to_str().unwrap().to_string();
                let table_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
                messages.add_message(&gdb_path)?;
                messages.add_message(&table_name)?;

                // Create a new feature class
                use gp::tools::GpToolExecute;
                let wkid = 4326;
                let create_tool = gp::tools::GpCreateFeatureClassTool::new(gdb_path, table_name, gp::api::ShapeType::Point, wkid);
                match create_tool.execute(py) {
                    Ok(gp_result) =>  {

                        // Try to access the catalog path from the geoprocessing result
                        let catalog_path = gp_result.first_as_str(py)?;
                        messages.add_message(&catalog_path)?;

                        let text_field = gp::api::GpField {
                            name: String::from("Description"),
                            field_type: gp::api::FieldType::String
                        };

                        let fields = vec![text_field];
                        let fields_tool = gp::tools::GpAddFieldsTool::new(catalog_path, fields);
                        match fields_tool.execute(py) {
                            Ok(_) => {

                                // Bump some features into it
                                let field_names = vec![String::from("SHAPE@"), String::from("Description")];
                                let dessau_location = gp::api::Point {
                                    x: 12.24555,
                                    y: 51.83864
                                };

                                // Fill the feature buffer
                                let mut feature_buffer = gp::api::InsertBuffer::new(2);
                                feature_buffer.add_value(py, dessau_location);
                                feature_buffer.add_value(py, "Dessau");

                                let insert_cursor = gp_param.into_insert_cursor(field_names)?;
                                insert_cursor.insert(feature_buffer)?;

                                messages.add_message("Feature was inserted!")?;

                                Ok(())
                            },
                            Err(err) => Err(err)
                        }
                    },
                    Err(err) => {
                        //messages.add_message(&err.to_string())?;
                        Err(err)
                    }
                }
            },
            None => todo!("Ouput parameter type not found!")
        }
    }
}



/// Creates a new toolbox
#[pyfunction]
fn create_toolbox(label: &str, alias: &str) -> PyResult<gp::PyToolbox> {
    let copy_features = CopyFeaturesTool {
    };

    let pytoolbox_factory = gp::PyToolboxFactory {
    };

    let py_toolbox = pytoolbox_factory.create_toolbox(label, alias, vec![copy_features])?;

    Ok(py_toolbox)
}

/// This module allows the implementation of Geoprocessing Tools using Rust.
#[pymodule]
fn copyfeatures(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<gp::PyToolbox>()?;
    module.add_function(wrap_pyfunction!(create_toolbox, module)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::gp;
    use super::DummyGpTool;

    #[test]
    fn create_toolbox() {
        // Methods from traits must be known in current scope!
        use gp::api::GpTool;
        let dummy_tool = DummyGpTool {
        };

        let py_tool = gp::PyTool {
            label: dummy_tool.label().to_string(),
            description: dummy_tool.description().to_string(),
            tool_impl: Box::new(dummy_tool)
        };

        let toolbox = gp::PyToolbox {
            label: String::from("Test Toolbox"),
            alias:  String::from("test_rust"),
            py_tools: vec![py_tool]
        };

        assert_eq!("Test Toolbox", toolbox.label, "Label is wrong!");
    }
}

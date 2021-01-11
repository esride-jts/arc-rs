# arc-rs
Represents a thin Python wrapper for implementing Geoprocessing Tools using Rust. A Geoprocessing Tool performs specific tasks on geospatial data. The tools can be integrated in complex geospatial workflows. These workflows are usually designed by Geospatial Analysts in a "Model First" environment.

A custom Geoprocessing Tool can be implemented using Python. But, there is a need for implementing high sophistical geospatial analyses using solid rock runtime environments, libraries and frameworks. As an engineer I do not want to miss the capabilities of
- Efficient code generation (Compiler Buddy)
- Fail fast and often (Compiler says: "No!")
- Pay only for what you use (#YAGNI)

Nowadays, the best option seems to be Rust.

## Features
- Create custom Geoprocessing Tools using Rust
```
pub trait GpTool {

    fn label(&self) -> &str;

    fn description(&self) -> &str;

    fn parameters(&self) -> Vec<GpParameter>;

    fn execute(&self, py: Python, parameters: Vec<PyParameterValue>, messages: PyGpMessages) -> PyResult<()>;
}
```
- Execute any registered Geoprocessing Tool using Rust
```
let pyresult = gp::tools::execute_tool(py, "arcpy", "ListFeatureClasses", ())?;
let results_as_text = pyresult.as_vecstr();
for result_as_text in results_as_text {
    messages.add_message(&result_as_text)?;
}
```

## Instructions
Building the source by using cargo build release. There is a sample [Python toolbox](https://github.com/esride-jts/arc-rs/blob/main/deploy/arcintegration.pyt) using the provided custom Geoprocessing Tools implemented in Rust.

## Requirements
- Rust v1.44.1
- pyo3 v0.12.3

## Runtime requirements
- ArcGIS Pro v2.6 or later on Windows 10

## Resources
- [Creating Geoprocessing Tools with Python](https://pro.arcgis.com/en/pro-app/arcpy/geoprocessing_and_python/a-quick-tour-of-creating-tools-in-python.htm)

- [What is ModelBuilder?](https://pro.arcgis.com/en/pro-app/help/analysis/geoprocessing/modelbuilder/what-is-modelbuilder-.htm)

- [Geospatial Intelligence @ Medium](https://medium.com/geospatial-intelligence)
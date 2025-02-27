use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::fs;

#[derive(Debug, FromPyObject)]
struct MyItem {
    #[pyo3(item("id"))]
    id: String,
    #[pyo3(item("value"))]
    value: i64,
    // If "result" might not always be present or might be None,
    // you could use an Option<i64> here. But let's assume it's always an int.
    #[pyo3(item("result"))]
    result: i64,
}

fn main() -> PyResult<()> {
    // 1) Read the external Python script:
    let python_code = fs::read_to_string("script.py").expect("Could not read 'script.py' file");

    // 2) Acquire the Python GIL and run the code:
    Python::with_gil(|py| {
        // Create a new "global" dict that will hold the Python code's globals
        let globals = PyDict::new(py);

        // Execute the Python code, loading definitions into 'globals'
        py.run(&python_code, Some(globals), None)?;

        // Get the Python function 'transform_data' from 'globals'
        let transform_data = globals
            .get_item("transform_data")
            .expect("No function named 'transform_data' found in script.py");

        // 3) Build a list of dictionary-like objects in Rust:
        //    Each item is a PyDict with "id" (string) and "value" (integer).
        let item1 = {
            let dict = PyDict::new(py);
            dict.set_item("id", "item1")?;
            dict.set_item("value", 10)?;
            dict
        };

        let item2 = {
            let dict = PyDict::new(py);
            dict.set_item("id", "item2")?;
            dict.set_item("value", 42)?;
            dict
        };

        // Create a Python list of these two dicts
        let py_list = PyList::new(py, &[item1, item2]);

        // 4) Call the Python function with our list
        let result_any = transform_data.call1((py_list,))?;

        // Now directly extract the entire Python list into a Rust Vec<MyItem>
        let items: Vec<MyItem> = result_any.extract()?;


        // Print them in a nice debug format
        println!("Items: {:?}", items);

        // You can also iterate them:
        for item in &items {
            println!(
                "id={}, value={}, result={}",
                item.id, item.value, item.result
            );
        }

        Ok(())
    })
}

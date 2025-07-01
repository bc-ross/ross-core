use anyhow::{Result, anyhow};
use pyo3::prelude::*;
use pyo3::types::*;
use std::env;
use std::path::Path;

// fn main() {
//     let output = Command::new("python")
//         .arg("scraper.py")
//         .arg(env::var("CRATE_PROFILE").unwrap())
//         .output()
//         .expect("failed to execute process");

//     println!("{}", String::from_utf8_lossy(&output.stdout));
// }

fn main() -> Result<()> {
    let old_bt = env::var("RUST_BACKTRACE")?;
    env::set_var("RUST_BACKTRACE", "0");

    let script_dir = Path::new(&env::var("CRATE_MANIFEST_DIR")?).join("scripts");
    let data_dir = Path::new(&env::var("CRATE_MANIFEST_DIR")?).join("scraped_programs");
    let exec_path =
        Path::new(&env::var("CRATE_OUT_DIR")?).join(&env::var("CARGO_MAKE_CRATE_NAME")?);

    dbg!(&env::var("CONDA_PREFIX")?);
    dbg!(&script_dir, &exec_path, &data_dir);

    // if !matches!(env::var("PYO3_PYTHON"), Ok(x) if Path::new(&x) == Path::new(&env::var("CRATE_MANIFEST_DIR")?).join("env"))
    // {
    //     // TODO: add conda env to CRATE_MANIFEST_DIR
    //     return Err(anyhow!("Could not locate build-script Conda environment."));
    // }

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let sys = py.import("sys")?;
        println!("Python {}", sys.getattr("version")?.extract::<&str>()?);

        let sys_path = sys.getattr("path")?;
        sys_path
            .downcast::<PyList>()?
            .call_method1("insert", (0, script_dir.to_string_lossy()))?;

        let my_script = py.import("scraper")?;
        dbg!(my_script.call_method1(
            "inject",
            (data_dir.to_string_lossy(), exec_path.to_string_lossy()),
        )?);

        Ok(())
    })?;

    env::set_var("RUST_BACKTRACE", old_bt);
    Ok(())
}

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;
use oslom_core::{
    Network, NetworkBuilder, OslomConfig, OslomResult, run_oslom,
    NodeId, OslomError
};

#[pyclass]
pub struct PyOslom {
    config: OslomConfig,
    directed: bool,
    result: Option<OslomResult>,
}

#[pymethods]
impl PyOslom {
    #[new]
    #[pyo3(signature = (directed=false, r=None, hr=None, threshold=None, cp=None, find_singletons=false, random_seed=None, verbose=false))]
    fn new(
        directed: bool,
        r: Option<usize>,
        hr: Option<usize>,
        threshold: Option<f64>,
        cp: Option<f64>,
        find_singletons: bool,
        random_seed: Option<u64>,
        verbose: bool,
    ) -> Self {
        let mut config = OslomConfig::default();
        
        if let Some(r) = r {
            config.r = r;
        }
        if let Some(hr) = hr {
            config.hr = hr;
        }
        if let Some(t) = threshold {
            config.threshold = t;
        }
        if let Some(cp) = cp {
            config.cp = cp;
        }
        
        config.find_singletons = find_singletons;
        config.random_seed = random_seed;
        config.verbose = verbose;

        Self {
            config,
            directed,
            result: None,
        }
    }

    fn fit(&mut self, edges: Vec<(usize, usize, f64)>) -> PyResult<()> {
        // Build network from edge list
        let mut builder = NetworkBuilder::new(self.directed);
        
        for (from, to, weight) in edges {
            builder.add_edge(from, to, weight);
        }
        
        let network = builder.build();
        
        // Run OSLOM algorithm
        match run_oslom(&network, &self.config) {
            Ok(result) => {
                self.result = Some(result);
                Ok(())
            }
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("OSLOM failed: {}", e)
            ))
        }
    }

    fn get_clusters(&self) -> PyResult<HashMap<usize, HashMap<usize, Vec<NodeId>>>> {
        match &self.result {
            Some(result) => Ok(result.to_python_format()),
            None => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Must call fit() first"
            ))
        }
    }

    fn get_statistics(&self) -> PyResult<HashMap<String, f64>> {
        match &self.result {
            Some(result) => {
                let mut stats = HashMap::new();
                stats.insert("num_modules".to_string(), result.statistics.num_modules as f64);
                stats.insert("coverage".to_string(), result.statistics.coverage as f64);
                stats.insert("modularity".to_string(), result.statistics.modularity);
                stats.insert("total_nodes".to_string(), result.statistics.total_nodes as f64);
                stats.insert("homeless_nodes".to_string(), result.statistics.homeless_nodes as f64);
                stats.insert("num_levels".to_string(), result.num_levels() as f64);
                Ok(stats)
            }
            None => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Must call fit() first"
            ))
        }
    }

    fn get_config(&self) -> PyResult<HashMap<String, PyObject>> {
        Python::with_gil(|py| {
            let mut config = HashMap::new();
            config.insert("directed".to_string(), self.directed.into_py(py));
            config.insert("r".to_string(), self.config.r.into_py(py));
            config.insert("hr".to_string(), self.config.hr.into_py(py));
            config.insert("threshold".to_string(), self.config.threshold.into_py(py));
            config.insert("cp".to_string(), self.config.cp.into_py(py));
            config.insert("find_singletons".to_string(), self.config.find_singletons.into_py(py));
            config.insert("random_seed".to_string(), self.config.random_seed.into_py(py));
            config.insert("verbose".to_string(), self.config.verbose.into_py(py));
            Ok(config)
        })
    }

    fn __repr__(&self) -> String {
        format!("PyOslom(directed={}, r={}, hr={}, threshold={}, cp={})", 
                self.directed, self.config.r, self.config.hr, self.config.threshold, self.config.cp)
    }
}

// Convenience functions for direct usage
#[pyfunction]
#[pyo3(signature = (edges, directed=false, r=10, hr=50, threshold=0.1, cp=0.5, find_singletons=false, random_seed=None, verbose=false))]
fn run_oslom_direct(
    edges: Vec<(usize, usize, f64)>,
    directed: bool,
    r: usize,
    hr: usize,
    threshold: f64,
    cp: f64,
    find_singletons: bool,
    random_seed: Option<u64>,
    verbose: bool,
) -> PyResult<HashMap<usize, HashMap<usize, Vec<NodeId>>>> {
    let mut builder = NetworkBuilder::new(directed);
    
    for (from, to, weight) in edges {
        builder.add_edge(from, to, weight);
    }
    
    let network = builder.build();
    let config = OslomConfig {
        r,
        hr,
        threshold,
        cp,
        find_singletons,
        random_seed,
        verbose,
        ..Default::default()
    };
    
    match run_oslom(&network, &config) {
        Ok(result) => Ok(result.to_python_format()),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            format!("OSLOM failed: {}", e)
        ))
    }
}

#[pyfunction]
fn set_verbose(verbose: bool) {
    // This function exists for compatibility with the C++ version
    // In the Rust version, verbosity is controlled per-instance
    println!("Verbose mode: {}", verbose);
}

#[pymodule]
fn _rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyOslom>()?;
    m.add_function(wrap_pyfunction!(run_oslom_direct, m)?)?;
    m.add_function(wrap_pyfunction!(set_verbose, m)?)?;
    Ok(())
}
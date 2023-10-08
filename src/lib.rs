use extism_pdk::*;
use lazy_static::lazy_static;
use matricks_plugin::{MatrixConfiguration, PluginUpdate};
use serde_json::from_str;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref CONFIG: Arc<Mutex<MatrixConfiguration>> =
        Arc::new(Mutex::new(MatrixConfiguration::default()));
}

#[plugin_fn]
pub fn setup(cfg_json: String) -> FnResult<()> {
    // Set the matrix configuration struct
    let mut config = CONFIG.lock().unwrap();
    let config = config.deref_mut();
    *config = from_str(&*cfg_json)
        .expect("Unable to deserialize matrix config!");

    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<PluginUpdate>> {
    let config = CONFIG.lock().unwrap();

    Ok(Json(PluginUpdate {
        state: vec![vec![[0, 0, 0, 0]; config.width]; config.height],
        ..Default::default()
    }))
}
use extism_pdk::*;

#[plugin_fn]
pub fn setup(_: ()) -> FnResult<()> {
    // Setup your plugin here

    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<Option<Vec<Vec<[u8; 4]>>>>> {
    // Generate and return updates here

    Ok(Json(None))
}
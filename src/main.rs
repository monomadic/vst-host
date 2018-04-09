extern crate vst;
extern crate colored;

mod error;
use error::VSTHostError;

mod display;

mod host;
use host::*;

use std::path::Path;
use std::sync::{Arc, Mutex};

use vst::plugin::Plugin;
use vst::host::PluginLoader;

fn main() {
    match load_plugin(Path::new("./plugin/dd-subsynth.vst/Contents/MacOS/dd-subsynth")) {
        Ok(_) => display::log("done."),
        Err(e) => display::error(e),
    }
}

fn load_plugin(path: &Path) -> Result<(), VSTHostError> {
    let host = Arc::new(Mutex::new(PluginHost));
    println!("Loading {}...", path.to_str().expect("plugin path is not valid"));

    let mut loader = PluginLoader::load(path, host.clone())?;
    let mut instance = loader.instance()?;

    // Get the plugin information
    let info = instance.get_info();

    println!("Loaded '{}':\n\t\
              Vendor: {}\n\t\
              Presets: {}\n\t\
              Parameters: {}\n\t\
              VST ID: {}\n\t\
              Version: {}\n\t\
              Initial Delay: {} samples",
              info.name,
              info.vendor,
              info.presets,
              info.parameters,
              info.unique_id,
              info.version,
              info.initial_delay);

    instance.init();

    Ok(())
}

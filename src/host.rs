use vst::host::{Host};

pub struct PluginHost;

impl Host for PluginHost {
    fn automate(&mut self, index: i32, value: f32) {
        ::display::log(&format!("Parameter Change: {} -> {}", index, value));
    }
}

// impl PluginHost {
//     pub fn load(path: &Path) {

//     }
// }

use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum VSTHostError {
    LoadingPlugin,
}

impl fmt::Display for VSTHostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl From<::vst::host::PluginLoadError> for VSTHostError {
    fn from(_error: ::vst::host::PluginLoadError) -> VSTHostError {
        VSTHostError::LoadingPlugin
    }
}

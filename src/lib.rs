#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#[cfg(feature = "runtime-tokio")]
use crate::tokio::RhaiHook;
use serde::{Deserialize, Serialize};
use vetis::{VetisHook, VetisResult, matcher::MatcherConfig};

/// Request module
pub(crate) mod request;
/// Response module
pub(crate) mod response;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-tokio")]
/// Tokio runtime support module
pub mod tokio;

/// Builder for creating `RhaiHookConfig` instances.
pub struct HookConfigBuilder {
    matcher: Option<Box<dyn MatcherConfig>>,
    script: String,
}

impl HookConfigBuilder {
    /// Allow set a matcher
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn matcher<M>(mut self, matcher: M) -> Self
    where
        M: MatcherConfig + Send + Sync + 'static,
    {
        self.matcher = Some(Box::new(matcher));
        self
    }

    /// Allow set the script of rhai path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn script(mut self, script: &str) -> Self {
        self.script = script.to_string();
        self
    }

    /// Build the `StaticHookConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<FlashConfig>` - The `StaticHookConfig` with the configured settings.
    pub fn build(self) -> VetisResult<HookConfig> {
        Ok(HookConfig {
            matcher: self
                .matcher
                .map_or(None, |val| Some(val)),
            script: self.script,
        })
    }
}

/// Configuration for static file serving.
#[derive(Clone, Serialize, Deserialize)]
pub struct HookConfig {
    matcher: Option<Box<dyn MatcherConfig>>,
    script: String,
}

#[typetag::serde(name = "rhai")]
impl vetis::hook::HookConfig for HookConfig {
    fn matcher(&self) -> Option<&Box<dyn MatcherConfig>> {
        self.matcher
            .as_ref()
    }

    fn boxed_clone(&self) -> Box<dyn vetis::hook::HookConfig> {
        Box::new(self.clone())
    }

    fn boxed_hook(&self) -> Box<dyn VetisHook> {
        Box::new(RhaiHook::new(self.clone()))
    }
}

impl HookConfig {
    /// Allow create a new `RhaiHookConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `StaticHookConfigBuilder` - The builder.
    pub fn builder() -> HookConfigBuilder {
        HookConfigBuilder { matcher: None, script: "index.rhai".into() }
    }

    /// Returns script
    ///
    /// # Returns
    ///
    /// * `&str` - The script.
    pub fn script(&self) -> &str {
        &self.script
    }
}

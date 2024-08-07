#[cfg(feature = "compat-http")]
use std::str::FromStr;

pub use http::header::{HeaderMap, HeaderName, HeaderValue};
pub use either_1_13::Either;
pub use void::Void;

#[cfg(feature = "compat-http")]
pub use http_0_2::{
    HeaderMap as CompatHeaderMap, HeaderName as CompatHeaderName, HeaderValue as CompatHeaderValue,
};

/// Converts [`CompatHeaderMap`] from `http@0.2` to [`HeaderMap`] from `http@1.1`.
#[cfg(feature = "compat-http")]
pub trait ToCurrentHeaderVersion {
    /// Update map version
    fn to_current_version(self) -> HeaderMap;
}

#[cfg(feature = "compat-http")]
impl ToCurrentHeaderVersion for CompatHeaderMap {
    fn to_current_version(self) -> HeaderMap {
        self.into_iter()
            .filter_map(|(key, value)| {
                Some((
                    HeaderName::from_str(key?.as_str()).ok()?,
                    HeaderValue::from_bytes(value.as_bytes()).ok()?,
                ))
            })
            .collect()
    }
}

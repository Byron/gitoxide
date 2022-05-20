use crate::{value, values, File};
use bstr::BStr;
use std::borrow::Cow;
use std::convert::TryFrom;

/// Comfortable API for accessing values
impl<'a> File<'a> {
    /// Like [`value()`][GitConfig::value()], but returning an `Option` if the string wasn't found.
    ///
    /// As strings perform no conversions, this will never fail.
    pub fn string(&'a self, section_name: &str, subsection_name: Option<&str>, key: &str) -> Option<Cow<'a, BStr>> {
        self.raw_value(section_name, subsection_name, key)
            .ok()
            .map(|v| values::String::from(v).value)
    }

    /// Like [`value()`][GitConfig::value()], but returning an `Option` if the path wasn't found.
    ///
    /// Note that this path is not vetted and should only point to resources which can't be used
    /// to pose a security risk.
    ///
    /// As paths perform no conversions, this will never fail.
    // TODO: add `secure_path()` or similar to make use of our knowledge of the trust associated with each configuration
    //       file, maybe even remove the insecure version to force every caller to ask themselves if the resource can
    //       be used securely or not.
    pub fn path(&'a self, section_name: &str, subsection_name: Option<&str>, key: &str) -> Option<values::Path<'a>> {
        self.raw_value(section_name, subsection_name, key)
            .ok()
            .map(values::Path::from)
    }

    /// Like [`value()`][GitConfig::value()], but returning an `Option` if the boolean wasn't found.
    pub fn boolean(
        &'a self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Option<Result<bool, value::parse::Error>> {
        self.raw_value(section_name, subsection_name, key)
            .ok()
            .map(|v| values::Boolean::try_from(v).map(|b| b.to_bool()))
    }

    /// Like [`value()`][GitConfig::value()], but returning an `Option` if the integer wasn't found.
    pub fn integer(
        &'a self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Option<Result<i64, value::parse::Error>> {
        let int = self.raw_value(section_name, subsection_name, key).ok()?;
        Some(values::Integer::try_from(int.as_ref()).and_then(|b| {
            b.to_decimal()
                .ok_or_else(|| value::parse::Error::new("Integer overflow", int.into_owned()))
        }))
    }

    /// Similar to [`multi_value(…)`][GitConfig::multi_value()] but returning strings if at least one of them was found.
    pub fn strings(&self, section_name: &str, subsection_name: Option<&str>, key: &str) -> Option<Vec<Cow<'_, BStr>>> {
        self.raw_multi_value(section_name, subsection_name, key)
            .ok()
            .map(|values| values.into_iter().map(|v| values::String::from(v).value).collect())
    }

    /// Similar to [`multi_value(…)`][GitConfig::multi_value()] but returning integers if at least one of them was found
    /// and if none of them overflows.
    pub fn integers(
        &self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Option<Result<Vec<i64>, value::parse::Error>> {
        self.raw_multi_value(section_name, subsection_name, key)
            .ok()
            .map(|values| {
                values
                    .into_iter()
                    .map(|v| {
                        values::Integer::try_from(v.as_ref()).and_then(|int| {
                            int.to_decimal()
                                .ok_or_else(|| value::parse::Error::new("Integer overflow", v.into_owned()))
                        })
                    })
                    .collect()
            })
    }
}

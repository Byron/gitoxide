use std::{borrow::Cow, convert::TryFrom};

use bstr::BStr;

use crate::{value, File};

/// Comfortable API for accessing values
impl<'event> File<'event> {
    /// Like [`value()`][File::value()], but returning an `None` if the string wasn't found.
    ///
    /// As strings perform no conversions, this will never fail.
    pub fn string(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&str>,
        key: impl AsRef<str>,
    ) -> Option<Cow<'_, BStr>> {
        self.raw_value(section_name, subsection_name, key).ok()
    }

    /// Like [`value()`][File::value()], but returning `None` if the path wasn't found.
    ///
    /// Note that this path is not vetted and should only point to resources which can't be used
    /// to pose a security risk.
    ///
    /// As paths perform no conversions, this will never fail.
    // TODO: add `secure_path()` or similar to make use of our knowledge of the trust associated with each configuration
    //       file, maybe even remove the insecure version to force every caller to ask themselves if the resource can
    //       be used securely or not.
    pub fn path(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&str>,
        key: impl AsRef<str>,
    ) -> Option<crate::Path<'_>> {
        self.raw_value(section_name, subsection_name, key)
            .ok()
            .map(crate::Path::from)
    }

    /// Like [`value()`][File::value()], but returning `None` if the boolean value wasn't found.
    pub fn boolean(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&str>,
        key: impl AsRef<str>,
    ) -> Option<Result<bool, value::Error>> {
        self.raw_value(section_name, subsection_name, key)
            .ok()
            .map(|v| crate::Boolean::try_from(v).map(|b| b.into()))
    }

    /// Like [`value()`][File::value()], but returning an `Option` if the integer wasn't found.
    pub fn integer(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&str>,
        key: impl AsRef<str>,
    ) -> Option<Result<i64, value::Error>> {
        let int = self.raw_value(section_name, subsection_name, key).ok()?;
        Some(crate::Integer::try_from(int.as_ref()).and_then(|b| {
            b.to_decimal()
                .ok_or_else(|| value::Error::new("Integer overflow", int.into_owned()))
        }))
    }

    /// Similar to [`values(…)`][File::values()] but returning strings if at least one of them was found.
    pub fn strings(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&str>,
        key: impl AsRef<str>,
    ) -> Option<Vec<Cow<'_, BStr>>> {
        self.raw_values(section_name, subsection_name, key).ok()
    }

    /// Similar to [`values(…)`][File::values()] but returning integers if at least one of them was found
    /// and if none of them overflows.
    pub fn integers(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&str>,
        key: impl AsRef<str>,
    ) -> Option<Result<Vec<i64>, value::Error>> {
        self.raw_values(section_name, subsection_name, key).ok().map(|values| {
            values
                .into_iter()
                .map(|v| {
                    crate::Integer::try_from(v.as_ref()).and_then(|int| {
                        int.to_decimal()
                            .ok_or_else(|| value::Error::new("Integer overflow", v.into_owned()))
                    })
                })
                .collect()
        })
    }
}

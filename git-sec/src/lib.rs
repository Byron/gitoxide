#![deny(unsafe_code, rust_2018_idioms, missing_docs)]
//! A shared trust model for `gitoxide` crates.

/// Various types to identify entities.
pub mod identity {

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    /// An account based identity
    pub struct Account {
        /// The user's name
        pub username: String,
        /// The user's password
        pub password: String,
    }

    use std::borrow::Cow;
    use std::path::Path;

    /// Returns true if the given `path` is owned by the user who is executing the current process.
    ///
    /// Note that this method is very specific to avoid having to deal with any operating system types.
    pub fn is_path_owned_by_current_user(path: Cow<'_, Path>) -> std::io::Result<bool> {
        impl_::is_path_owned_by_current_user(path)
    }

    #[cfg(not(windows))]
    mod impl_ {
        use std::borrow::Cow;
        use std::path::Path;

        pub fn is_path_owned_by_current_user(path: Cow<'_, Path>) -> std::io::Result<bool> {
            fn from_path(path: Cow<'_, Path>) -> std::io::Result<u32> {
                use std::os::unix::fs::MetadataExt;
                let meta = std::fs::symlink_metadata(path)?;
                Ok(meta.uid())
            }

            fn from_process() -> std::io::Result<u32> {
                // SAFETY: there is no documented possibility for failure
                #[allow(unsafe_code)]
                let uid = unsafe { libc::geteuid() };
                Ok(uid)
            }

            Ok(from_path(path)? == from_process()?)
        }
    }

    #[cfg(windows)]
    mod impl_ {
        use std::borrow::Cow;
        use std::path::Path;

        fn err(msg: &str) -> std::io::Error {
            std::io::Error::new(std::io::ErrorKind::Other, msg)
        }

        pub fn is_path_owned_by_current_user(path: Cow<'_, Path>) -> std::io::Result<bool> {
            use windows::Win32::{
                Foundation::{CloseHandle, ERROR_SUCCESS, HANDLE, PSID},
                Security,
                Security::Authorization::SE_FILE_OBJECT,
                System::Threading,
            };
            let mut handle = HANDLE::default();
            let mut descriptor = Security::PSECURITY_DESCRIPTOR::default();
            let mut err_msg = None;
            let mut is_owned = false;

            #[allow(unsafe_code)]
            unsafe {
                Threading::OpenProcessToken(Threading::GetCurrentProcess(), Security::TOKEN_QUERY, &mut handle)
                    .ok()
                    .map_err(|_| err("Failed to open process token"))?;

                let mut len = 0_u32;
                if Security::GetTokenInformation(&handle, Security::TokenUser, std::ptr::null_mut(), 0, &mut len)
                    .as_bool()
                {
                    let mut token_user = Security::TOKEN_USER::default();
                    if Security::GetTokenInformation(
                        &handle,
                        Security::TokenUser,
                        &mut token_user as *mut _ as *mut std::ffi::c_void,
                        len,
                        &mut len,
                    )
                    .as_bool()
                    {
                        // NOTE: we avoid to copy the sid or cache it in any way for now, even though it should be possible
                        //       with a custom allocation/vec/box and it's just very raw. Can the `windows` crate do better?
                        //       When/If yes, then let's improve this.
                        if Security::IsValidSid(token_user.User.Sid).as_bool() {
                            use std::os::windows::ffi::OsStrExt;
                            let mut wide_path: Vec<_> = path.as_ref().as_os_str().encode_wide().collect();
                            // err = GetNamedSecurityInfoW(wpath, SE_FILE_OBJECT,
                            //                             OWNER_SECURITY_INFORMATION |
                            //                                 DACL_SECURITY_INFORMATION,
                            //                             &sid, NULL, NULL, NULL, &descriptor);
                            let mut path_sid = PSID::default();
                            let res = Security::Authorization::GetNamedSecurityInfoW(
                                windows::core::PCWSTR(wide_path.as_mut_ptr()),
                                SE_FILE_OBJECT,
                                Security::OWNER_SECURITY_INFORMATION | Security::DACL_SECURITY_INFORMATION,
                                &mut path_sid,
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                                &mut descriptor,
                            );

                            if res == ERROR_SUCCESS.0 && Security::IsValidSid(path_sid).as_bool() {
                                is_owned = Security::EqualSid(path_sid, token_user.User.Sid).as_bool();
                            } else {
                                err_msg = "couldn't get owner for path or it wasn't valid".into();
                            }
                        } else {
                            err_msg = "owner id of current process wasn't set or valid".into();
                        }
                    } else {
                        err_msg = "Could not get information about the token user".into();
                    }
                } else {
                    err_msg = "Could not get token information for length of token user".into();
                }
                CloseHandle(handle);
                if !descriptor.is_invalid() {
                    windows::core::heap_free(descriptor.0);
                }
            }
            err_msg.map(|msg| Err(err(msg))).unwrap_or(Ok(is_owned))
        }
    }
}

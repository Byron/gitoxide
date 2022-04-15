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

        fn err(msg: impl Into<String>) -> std::io::Error {
            std::io::Error::new(std::io::ErrorKind::Other, msg.into())
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
                if !Security::GetTokenInformation(handle, Security::TokenUser, std::ptr::null_mut(), 0, &mut len)
                    .as_bool()
                {
                    let mut info_buf = Vec::<u8>::new();
                    info_buf.reserve_exact(len as usize);
                    if Security::GetTokenInformation(
                        handle,
                        Security::TokenUser,
                        info_buf.as_mut_ptr() as *mut std::ffi::c_void,
                        len,
                        &mut len,
                    )
                    .as_bool()
                    {
                        // NOTE: we avoid to copy the sid or cache it in any way for now, even though it should be possible
                        //       with a custom allocation/vec/box and it's just very raw. Can the `windows` crate do better?
                        //       When/If yes, then let's improve this.
                        //       It should however be possible to create strings from SIDs, check this once more.
                        let info: *const Security::TOKEN_USER = std::mem::transmute(info_buf.as_ptr());
                        if Security::IsValidSid((*info).User.Sid).as_bool() {
                            let wide_path = to_wide_path(&path);
                            let mut path_sid = PSID::default();
                            let res = Security::Authorization::GetNamedSecurityInfoW(
                                windows::core::PCWSTR(wide_path.as_ptr()),
                                SE_FILE_OBJECT,
                                Security::OWNER_SECURITY_INFORMATION | Security::DACL_SECURITY_INFORMATION,
                                &mut path_sid as *mut _,
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                                &mut descriptor as *mut _,
                            );

                            if res == ERROR_SUCCESS.0 && Security::IsValidSid(path_sid).as_bool() {
                                is_owned = Security::EqualSid(path_sid, (*info).User.Sid).as_bool();
                                dbg!(is_owned, path.as_ref());
                            } else {
                                err_msg = format!("couldn't get owner for path or it wasn't valid: {}", res).into();
                            }
                        } else {
                            err_msg = String::from("owner id of current process wasn't set or valid").into();
                        }
                    } else {
                        err_msg = String::from("Could not get information about the token user").into();
                    }
                } else {
                    err_msg = String::from("Could not get token information for length of token user").into();
                }
                CloseHandle(handle);
                if !descriptor.is_invalid() {
                    windows::core::heap_free(descriptor.0);
                }
            }
            err_msg.map(|msg| Err(err(msg))).unwrap_or(Ok(is_owned))
        }

        fn to_wide_path(path: &Path) -> Vec<u16> {
            use std::os::windows::ffi::OsStrExt;
            let mut wide_path: Vec<_> = path.as_os_str().encode_wide().collect();
            wide_path.push(0);
            wide_path
        }
    }
}

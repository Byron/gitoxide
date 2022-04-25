use std::path::Path;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
/// An account based identity
pub struct Account {
    /// The user's name
    pub username: String,
    /// The user's password
    pub password: String,
}

/// Returns true if the given `path` is owned by the user who is executing the current process.
///
/// Note that this method is very specific to avoid having to deal with any operating system types.
pub fn is_path_owned_by_current_user(path: impl AsRef<Path>) -> std::io::Result<bool> {
    impl_::is_path_owned_by_current_user(path)
}

#[cfg(not(windows))]
mod impl_ {
    use std::path::Path;

    pub fn is_path_owned_by_current_user(path: impl AsRef<Path>) -> std::io::Result<bool> {
        fn owner_from_path(path: impl AsRef<Path>) -> std::io::Result<u32> {
            use std::os::unix::fs::MetadataExt;
            let meta = std::fs::symlink_metadata(path)?;
            Ok(meta.uid())
        }

        fn owner_of_current_process() -> std::io::Result<u32> {
            // SAFETY: there is no documented possibility for failure
            #[allow(unsafe_code)]
            let uid = unsafe { libc::geteuid() };
            Ok(uid)
        }

        Ok(owner_from_path(path)? == owner_of_current_process()?)
    }
}

#[cfg(windows)]
mod impl_ {
    use std::path::Path;

    fn err(msg: impl Into<String>) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, msg.into())
    }

    pub fn is_path_owned_by_current_user(path: impl AsRef<Path>) -> std::io::Result<bool> {
        use windows::{
            core::{Error, PCWSTR},
            Win32::{
                Foundation::{ERROR_SUCCESS, HANDLE, PSID},
                Security::{
                    Authorization::{GetNamedSecurityInfoW, SE_FILE_OBJECT},
                    EqualSid, GetTokenInformation, TokenOwner, OWNER_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR,
                    TOKEN_OWNER, TOKEN_QUERY,
                },
                System::{
                    Memory::LocalFree,
                    Threading::{GetCurrentProcess, OpenProcessToken},
                },
            },
        };

        let mut err_msg = None;
        let mut is_owned = false;
        let path = path.as_ref();

        #[allow(unsafe_code)]
        unsafe {
            let mut folder_owner = PSID::default();
            let mut pdescriptor = PSECURITY_DESCRIPTOR::default();
            let result = GetNamedSecurityInfoW(
                PCWSTR(to_wide_path(path).as_ptr()),
                SE_FILE_OBJECT,
                OWNER_SECURITY_INFORMATION,
                &mut folder_owner,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut pdescriptor,
            );

            // Workaround for https://github.com/microsoft/win32metadata/issues/884
            if result == ERROR_SUCCESS.0 {
                let mut token = HANDLE::default();
                OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).ok()?;

                let mut buffer_size = 0;
                let mut buffer = Vec::<u8>::new();
                GetTokenInformation(token, TokenOwner, std::ptr::null_mut(), 0, &mut buffer_size);
                if buffer_size != 0 {
                    buffer.resize(buffer_size as usize, 0);
                    if GetTokenInformation(
                        token,
                        TokenOwner,
                        buffer.as_mut_ptr() as _,
                        buffer_size,
                        &mut buffer_size,
                    )
                    .as_bool()
                    {
                        let token_owner = buffer.as_ptr() as *const TOKEN_OWNER;
                        let token_owner = (*token_owner).Owner;

                        is_owned = EqualSid(folder_owner, token_owner).as_bool();
                    } else {
                        err_msg = format!(
                            "Couldn't get actual token information for current process with err: {}",
                            Error::from_win32()
                        )
                        .into();
                    }
                } else {
                    err_msg = format!(
                        "Couldn't get token information size info for current process with err: {}",
                        Error::from_win32()
                    )
                    .into();
                }
            } else {
                err_msg = format!(
                    "Couldn't get security information for path '{}' with err {}",
                    path.display(),
                    Error::from_win32()
                )
                .into();
            }
            LocalFree(pdescriptor.0 as isize);
        }

        err_msg.map(|msg| Err(err(msg))).unwrap_or(Ok(is_owned))
    }

    fn to_wide_path(path: impl AsRef<Path>) -> Vec<u16> {
        use std::os::windows::ffi::OsStrExt;
        let mut wide_path: Vec<_> = path.as_ref().as_os_str().encode_wide().collect();
        wide_path.push(0);
        wide_path
    }
}

use std::path::Path;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
pub fn is_path_owned_by_current_user(path: &Path) -> std::io::Result<bool> {
    impl_::is_path_owned_by_current_user(path)
}

#[cfg(not(windows))]
mod impl_ {
    use std::path::Path;

    pub fn is_path_owned_by_current_user(path: &Path) -> std::io::Result<bool> {
        fn owner_from_path(path: &Path) -> std::io::Result<u32> {
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
        use std::str::FromStr;

        let owner_of_path = owner_from_path(path)?;
        let owner_of_process = owner_of_current_process()?;
        if owner_of_path == owner_of_process {
            Ok(true)
        } else if let Some(sudo_uid) =
            std::env::var_os("SUDO_UID").and_then(|val| val.to_str().and_then(|val_str| u32::from_str(val_str).ok()))
        {
            Ok(owner_of_path == sudo_uid)
        } else {
            Ok(false)
        }
    }
}

#[cfg(windows)]
mod impl_ {
    use std::path::Path;

    fn err(msg: impl Into<String>) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, msg.into())
    }

    pub fn is_path_owned_by_current_user(path: &Path) -> std::io::Result<bool> {
        use windows::{
            core::{Error, PCWSTR},
            Win32::{
                Foundation::{CloseHandle, BOOL, HANDLE, HLOCAL, PSID},
                Security::{
                    Authorization::{GetNamedSecurityInfoW, SE_FILE_OBJECT},
                    CheckTokenMembership, EqualSid, GetTokenInformation, IsWellKnownSid, TokenOwner,
                    WinBuiltinAdministratorsSid, OWNER_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, TOKEN_OWNER,
                    TOKEN_QUERY,
                },
                System::{
                    Memory::LocalFree,
                    Threading::{GetCurrentProcess, GetCurrentThread, OpenProcessToken, OpenThreadToken},
                },
            },
        };

        let mut err_msg = None;
        let mut is_owned = false;

        if !path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?} does not exist.", path),
            ));
        }

        // Home is not actually owned by the corresponding user
        // but it can be considered de-facto owned by the user
        // Ignore errors here and just do the regular checks below
        if gix_path::realpath(path).ok() == gix_path::env::home_dir() {
            return Ok(true);
        }

        #[allow(unsafe_code)]
        unsafe {
            let mut folder_owner = PSID::default();
            let mut pdescriptor = PSECURITY_DESCRIPTOR::default();
            let result = GetNamedSecurityInfoW(
                PCWSTR(to_wide_path(path).as_ptr()),
                SE_FILE_OBJECT,
                OWNER_SECURITY_INFORMATION,
                Some(&mut folder_owner),
                None,
                None,
                None,
                &mut pdescriptor,
            );

            // Workaround for https://github.com/microsoft/win32metadata/issues/884
            if result.is_ok() {
                let mut token = HANDLE::default();
                // Use the current thread token if possible, otherwise open the process token
                OpenThreadToken(GetCurrentThread(), TOKEN_QUERY, true, &mut token)
                    .ok()
                    .or_else(|_| OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).ok())?;

                let mut buffer_size = 0;
                let mut buffer = Vec::<u8>::new();
                GetTokenInformation(token, TokenOwner, None, 0, &mut buffer_size);
                if buffer_size != 0 {
                    buffer.resize(buffer_size as usize, 0);
                    if GetTokenInformation(
                        token,
                        TokenOwner,
                        Some(buffer.as_mut_ptr() as *mut std::ffi::c_void),
                        buffer_size,
                        &mut buffer_size,
                    )
                    .as_bool()
                    {
                        let token_owner = buffer.as_ptr() as *const TOKEN_OWNER;
                        let token_owner = (*token_owner).Owner;

                        is_owned = EqualSid(folder_owner, token_owner).as_bool();

                        // Admin-group owned folders are considered owned by the current user, if they are in the admin group
                        if !is_owned && IsWellKnownSid(token_owner, WinBuiltinAdministratorsSid).as_bool() {
                            let mut is_member = BOOL::default();
                            // TODO: re-use the handle
                            match CheckTokenMembership(HANDLE::default(), token_owner, &mut is_member).ok() {
                                Err(e) => err_msg = Some(format!("Couldn't check if user is an administrator: {}", e)),
                                Ok(()) => is_owned = is_member.as_bool(),
                            }
                        }
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
                CloseHandle(token);
            } else {
                err_msg = format!(
                    "Couldn't get security information for path '{}' with err {}",
                    path.display(),
                    Error::from_win32()
                )
                .into();
            }
            LocalFree(HLOCAL(pdescriptor.0 as isize)).ok();
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

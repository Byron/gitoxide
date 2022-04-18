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
            core::PCWSTR,
            Win32::{
                Foundation::{BOOL, ERROR_SUCCESS, HANDLE, PSID},
                Security::{
                    Authorization::{GetNamedSecurityInfoW, SE_FILE_OBJECT},
                    CheckTokenMembershipEx, OWNER_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR,
                },
                System::Memory::LocalFree,
            },
        };

        let mut err_msg = None;
        let mut is_owned = false;

        #[allow(unsafe_code)]
        unsafe {
            let mut psid = PSID::default();
            let mut pdescriptor = PSECURITY_DESCRIPTOR::default();
            let wpath = to_wide_path(&path);

            let result = GetNamedSecurityInfoW(
                PCWSTR(wpath.as_ptr()),
                SE_FILE_OBJECT,
                OWNER_SECURITY_INFORMATION,
                &mut psid,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut pdescriptor,
            );

            if result == ERROR_SUCCESS.0 {
                let mut is_member = BOOL(0);
                if CheckTokenMembershipEx(HANDLE::default(), psid, 0, &mut is_member).as_bool() {
                    is_owned = is_member.as_bool();
                } else {
                    err_msg = String::from("Could not check token membership").into();
                }
            } else {
                err_msg = format!("Could not get security information for path with err: {}", result).into();
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

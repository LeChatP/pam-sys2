//! FFI wrappers for the Linux Pluggable Authentication Modules (PAM)
//!
//! This crate provides raw access to the Linux-PAM API.
//! Constants, types and functions are supported and created with `bindgen`.
//!
//! Note: Currently only tested on Linux as I lack access to other OSes at
//! the moment. Both `build.rs` and `wrapper.h` probably need to be customized
//! to exclude missing libraries such as `pam_misc` when they are not present.

#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    deref_nullptr
)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PamImplementation {
    LinuxPAM,
    OpenPAM,
}

pub const PAM_IMPLEMENTATION: PamImplementation = {
    #[cfg(PAM_SYS_IMPL = "linux-pam")]
    {
        PamImplementation::LinuxPAM
    }

    #[cfg(PAM_SYS_IMPL = "openpam")]
    {
        PamImplementation::OpenPAM
    }

    #[cfg(not(any(
        PAM_SYS_IMPL = "linux-pam",
        PAM_SYS_IMPL = "openpam",
    )))]
    compile_error!("No valid PAM implementation selected")
};

#[cfg(all(any(doc, PAM_SYS_IMPL = "linux-pam"), feature = "generate-bindings"))]
pub mod linuxpam {
    include!(concat!(env!("OUT_DIR"), "/linux_pam.rs"));
}

#[cfg(all(any(doc, PAM_SYS_IMPL = "linux-pam"), not(feature = "generate-bindings")))]
pub mod linuxpam;

#[cfg(all(any(doc, PAM_SYS_IMPL = "openpam"), feature = "generate-bindings"))]
pub mod openpam {
    include!(concat!(env!("OUT_DIR"), "/openpam.rs"));
}

#[cfg(all(any(doc, PAM_SYS_IMPL = "openpam"), not(feature = "generate-bindings")))]
pub mod openpam;

#[cfg(PAM_SYS_IMPL = "linux-pam")]
pub use linuxpam::*;

#[cfg(PAM_SYS_IMPL = "openpam")]
pub use openpam::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pam_implementation() {
        match PAM_IMPLEMENTATION {
            PamImplementation::LinuxPAM => {
                assert!(cfg!(PAM_SYS_IMPL = "linux-pam"));
            }
            PamImplementation::OpenPAM => {
                assert!(cfg!(PAM_SYS_IMPL = "openpam"));
            }
        }
    }

    #[test]
    fn test_pam_is_working() {
        unsafe{
            match PAM_IMPLEMENTATION {
                PamImplementation::LinuxPAM => {
                    use std::ffi::CString;
                    let service = CString::new("test_service").unwrap();
                    let user = CString::new("test_user").unwrap();
                    let mut pamh: *mut linuxpam::pam_handle_t = std::ptr::null_mut();
                    assert_eq!(linuxpam::pam_start(service.as_ptr(), user.as_ptr(), std::ptr::null_mut(), &mut pamh), PAM_SUCCESS);
                    assert!(!pamh.is_null());
                    assert_eq!(linuxpam::pam_end(pamh, PAM_SUCCESS), PAM_SUCCESS);
                }
                PamImplementation::OpenPAM => {
                    assert!(openpam::openpam_start("test_service", "test_user").is_ok());
                }
            }
        }
        
    }
        
}

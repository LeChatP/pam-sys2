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

    #[cfg(not(any(PAM_SYS_IMPL = "linux-pam", PAM_SYS_IMPL = "openpam",)))]
    compile_error!("No valid PAM implementation selected")
};

#[cfg(all(any(doc, PAM_SYS_IMPL = "linux-pam"), feature = "generate-bindings"))]
pub mod linuxpam {
    include!(concat!(env!("OUT_DIR"), "/linuxpam.rs"));
}

#[cfg(all(
    any(doc, PAM_SYS_IMPL = "linux-pam"),
    not(feature = "generate-bindings")
))]
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
    #[cfg(PAM_SYS_IMPL = "linux-pam")]
    fn test_linuxpam_is_working() {
        unsafe {
            match PAM_IMPLEMENTATION {
                PamImplementation::LinuxPAM => {
                    use std::ffi::CString;
                    let service = CString::new("test_service").unwrap();
                    let user = CString::new("test_user").unwrap();

                    // Create a minimal conversation structure
                    extern "C" fn conv_func(
                        _num_msg: ::std::os::raw::c_int,
                        _msg: *mut *const pam_message,
                        _resp: *mut *mut pam_response,
                        _appdata_ptr: *mut ::std::os::raw::c_void,
                    ) -> ::std::os::raw::c_int {
                        PAM_SUCCESS
                    }

                    let conv = pam_conv {
                        conv: Some(conv_func),
                        appdata_ptr: std::ptr::null_mut(),
                    };

                    let mut pamh: *mut pam_handle_t = std::ptr::null_mut();
                    assert_eq!(
                        pam_start(service.as_ptr(), user.as_ptr(), &conv, &mut pamh),
                        PAM_SUCCESS
                    );
                    assert!(!pamh.is_null());
                    assert_eq!(pam_end(pamh, PAM_SUCCESS), PAM_SUCCESS);
                }
                PamImplementation::OpenPAM => {
                    panic!("pam_sys is not configured for OpenPAM");
                }
            }
        }
    }

    #[test]
    #[cfg(PAM_SYS_IMPL = "openpam")]
    fn test_openpam_is_working() {
        unsafe {
            match PAM_IMPLEMENTATION {
                PamImplementation::OpenPAM => {
                    use std::ffi::CString;
                    let service = CString::new("test_service").unwrap();
                    let user = CString::new("test_user").unwrap();

                    // Create a minimal conversation structure
                    extern "C" fn conv_func(
                        _num_msg: ::std::os::raw::c_int,
                        _msg: *mut *const pam_message,
                        _resp: *mut *mut pam_response,
                        _appdata_ptr: *mut ::std::os::raw::c_void,
                    ) -> ::std::os::raw::c_int {
                        PAM_SUCCESS
                    }

                    let conv = pam_conv {
                        conv: Some(conv_func),
                        appdata_ptr: std::ptr::null_mut(),
                    };

                    let mut pamh: *mut pam_handle_t = std::ptr::null_mut();
                    assert_eq!(
                        pam_start(service.as_ptr(), user.as_ptr(), &conv, &mut pamh),
                        PAM_SUCCESS
                    );
                    assert!(!pamh.is_null());
                    assert_eq!(pam_end(pamh, PAM_SUCCESS), PAM_SUCCESS);
                }
                PamImplementation::LinuxPAM => {
                    panic!("pam_sys is not configured for LinuxPAM");
                }
            }
        }
    }
}

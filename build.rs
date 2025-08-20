// build.rs

use std::env;
#[cfg(feature = "generate-bindings")]
use std::path::{Path, PathBuf};

const PAM_IMPL_ENV_VAR: &str = "PAM_SYS_IMPL";
#[cfg(feature = "generate-bindings")]
const LINUX_PAM_REPO: &str = "https://github.com/linux-pam/linux-pam.git";
#[cfg(feature = "generate-bindings")]
const OPEN_PAM_REPO: &str = "https://git.des.dev/OpenPAM/OpenPAM.git";
#[cfg(feature = "generate-bindings")]
const LINUX_PAM_CLONE_DIR: &'static str = "target/linux-pam";
#[cfg(feature = "generate-bindings")]
const OPEN_PAM_CLONE_DIR: &'static str = "target/openpam";
#[cfg(feature = "generate-bindings")]
const LINUX_PAM_SUBFOLDERS: &'static[&'static str; 2] = &["libpam/include/security/", "libpamc/include/security/"];
#[cfg(feature = "generate-bindings")]
const OPEN_PAM_SUBFOLDERS: &'static[&'static str; 1] = &["include/security/"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PamImplementation {
    LinuxPam,
    OpenPam,
}

impl PamImplementation {
    fn resolve() -> Self {
        if let Ok(pam_imp) = env::var(PAM_IMPL_ENV_VAR) {
            let pam_impl = pam_imp.to_lowercase();
            return match &pam_impl[..] {
                "linuxpam" => Self::LinuxPam,
                "openpam" => Self::OpenPam,
                _ => {
                    panic!("Unrecognized '{}' environment variable value '{}'. Assessing from other information.", PAM_IMPL_ENV_VAR, pam_impl);
                }
            };
        }

        // LinuxPAM is used by linux and android
        if cfg!(target_os = "linux") || cfg!(target_os = "android") {
            Self::LinuxPam
        } else if cfg!(target_os = "freebsd")
            || cfg!(target_os = "netbsd")
            || cfg!(target_os = "macos")
            || cfg!(target_os = "ios")
            || cfg!(target_os = "dragonfly")
        {
            Self::OpenPam
        } else {
            panic!("Failed to resolve the PAM implementation. Use an appropriate target platform or set the `{}` environment variable to either `LINUXPAM` or `OPENPAM`.", PAM_IMPL_ENV_VAR);
        }
    }

    fn impl_name(self) -> &'static str {
        match self {
            Self::LinuxPam => "linux-pam",
            Self::OpenPam => "openpam",
        }
    }

    fn get_additional_libs(self) -> &'static [&'static str] {
        match self {
            Self::LinuxPam => &["pam_misc"],
            Self::OpenPam => &[],
        }
    }

    #[cfg(feature = "generate-bindings")]
    fn generate_bindings(self, out_dir: &PathBuf) {
        match self {
            Self::LinuxPam => generate_linuxpam(out_dir),
            Self::OpenPam => generate_openpam(out_dir),
        }
    }
}

#[cfg(feature = "generate-bindings")]
fn base_builder(
    header: &str,
    includes: &[&str],
    defines: &[(&str, Option<&str>)],
) -> bindgen::Builder {
    let workspace_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let clang_args: Vec<String> = includes
        .iter()
        .map(|inc| format!("-I{}", workspace_dir.join(inc).display()))
        .chain(defines.iter().map(|def| {
            format!(
                "-D{}{}",
                def.0,
                def.1.map_or_else(String::new, |v| format!("={}", v))
            )
        }))
        .collect();

    bindgen::Builder::default()
        .header(workspace_dir.join(header).to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .layout_tests(true)
        .generate_comments(true)
        .ctypes_prefix("libc")
        .opaque_type("pam_handle_t")
        .blocklist_type("va_list")
        .blocklist_type("__va_list")
        .blocklist_type("__builtin_va_list")
        .blocklist_type("__gnuc_va_list")
        .blocklist_type("__va_list_tag")
        .blocklist_function("pam_v.*")
        .allowlist_var("PAM_.*")
        .allowlist_function("pam_.*")
        .blocklist_function("pam_sm_*")
        .clang_args(clang_args)
}

#[cfg(feature = "generate-bindings")]
fn sparse_checkout(url: &str, dir: &str, subfolders: &[&str]) {
    // Clone the repository if it doesn't exist
    if !PathBuf::from(dir).exists() {
        let output = std::process::Command::new("git")
            .args(&["clone", "--no-checkout", url, dir])
            .output()
            .expect("Failed to clone repository");
        assert!(
            output.status.success(),
            "Failed to clone repository: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        // Initialize sparse checkout
        let output = std::process::Command::new("git")
            .args(&["sparse-checkout", "init", "--no-cone"])
            .current_dir(dir)
            .output()
            .expect("Failed to initialize sparse checkout");
        assert!(
            output.status.success(),
            "Failed to initialize sparse checkout: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        // Set sparse checkout paths
        let output = std::process::Command::new("git")
            .args(&mut ["sparse-checkout", "set", "--no-cone"].iter().chain(subfolders.iter()).into_iter())
            .current_dir(dir)
            .output()
            .expect("Failed to set sparse checkout paths");
        assert!(
            output.status.success(),
            "Failed to set sparse checkout paths: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        // Checkout the repository
        let output = std::process::Command::new("git")
            .args(&["checkout", "master"])
            .current_dir(dir)
            .output()
            .expect("Failed to checkout repository");
        assert!(
            output.status.success(),
            "Failed to checkout repository: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    // git update
    let output = std::process::Command::new("git")
        .args(&["pull", "--rebase"])
        .current_dir(dir)
        .output()
        .expect("Failed to update repository");
    assert!(
        output.status.success(),
        "Failed to update repository: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).display()
    );
}

#[cfg(feature = "generate-bindings")]
fn generate_linuxpam(out_dir: &PathBuf) {
    sparse_checkout(
        LINUX_PAM_REPO,
        LINUX_PAM_CLONE_DIR,
        LINUX_PAM_SUBFOLDERS,
    );
    base_builder(
        "wrapper-linuxpam.h",
        &["/usr/include"],
        if cfg!(target_arch = "x86_64") {
            &[
                ("__FLOAT128__", Some("1")),
                ("__SIZEOF_FLOAT128__", Some("16")),
            ]
        } else {
            &[]
        },
    )
    .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
    .raw_line("use libc::{uid_t, gid_t, group, passwd, spwd};")
    .blocklist_type(".*gid_t")
    .blocklist_type(".*uid_t")
    .blocklist_type("group")
    .blocklist_type("passwd")
    .blocklist_type("spwd")
    .generate()
    .expect("Unable to generate Linux-PAM bindings")
    .write_to_file(out_dir.join("linux_pam.rs"))
    .expect("Couldn't write Linux-PAM bindings");
}

#[cfg(feature = "generate-bindings")]
fn generate_openpam(out_dir: &PathBuf) {
    sparse_checkout(
        OPEN_PAM_REPO,
        OPEN_PAM_CLONE_DIR,
        OPEN_PAM_SUBFOLDERS,
    );
    base_builder("wrapper-openpam.h", &[], &[])
        .raw_line("use libc::passwd;")
        .blocklist_type("passwd")
        .allowlist_var("OPENPAM_.*")
        .allowlist_function("openpam_.*")
        .generate()
        .expect("Unable to generate OpenPAM bindings")
        .write_to_file(out_dir.join("openpam.rs"))
        .expect("Couldn't write OpenPAM bindings");
}

fn main() {
    let pam_implementation = PamImplementation::resolve();
    #[cfg(feature = "generate-bindings")]
    pam_implementation.generate_bindings(&PathBuf::from(env::var("OUT_DIR").unwrap()));

    println!(
        "cargo:rustc-cfg=PAM_SYS_IMPL=\"{impl_name}\"",
        impl_name = pam_implementation.impl_name()
    );

    // Tell cargo to tell rustc to link the system pam shared library.
    println!("cargo:rustc-link-lib=pam");
    for additional_lib in pam_implementation.get_additional_libs() {
        println!("cargo:rustc-link-lib={additional_lib}",);
    }
}

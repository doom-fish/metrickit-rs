use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-env-changed=DEVELOPER_DIR");
    println!("cargo:rerun-if-env-changed=SDKROOT");

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rustc-link-lib=framework=MetricKit");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-arg=-mmacosx-version-min=12.0");

    let swift_dir = "swift-bridge";
    let out_dir = env::var("OUT_DIR").unwrap();
    let swift_build_dir = format!("{out_dir}/swift-build");

    println!("cargo:rerun-if-changed={swift_dir}");

    if let Ok(output) = Command::new("swiftlint")
        .args(["lint"])
        .current_dir(swift_dir)
        .output()
    {
        if !output.status.success() {
            eprintln!(
                "SwiftLint warnings:\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        }
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let swift_triple = match target_arch.as_str() {
        "x86_64" => "x86_64-apple-macosx",
        "aarch64" => "arm64-apple-macosx",
        other => panic!("metrickit-rs: unsupported target arch '{other}'"),
    };

    let swift_args = vec![
        "build",
        "-c",
        "release",
        "--triple",
        swift_triple,
        "--package-path",
        swift_dir,
        "--scratch-path",
        &swift_build_dir,
    ];

    let output = Command::new("swift")
        .args(&swift_args)
        .output()
        .expect("Failed to build Swift bridge");

    if !output.status.success() {
        eprintln!(
            "Swift build STDOUT:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        eprintln!(
            "Swift build STDERR:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!(
            "Swift build failed with exit code: {:?}",
            output.status.code()
        );
    }

    link_swift_bridge(&swift_build_dir);
}

fn link_swift_bridge(swift_build_dir: &str) {
    println!("cargo:rustc-link-search=native={swift_build_dir}/release");
    println!("cargo:rustc-link-lib=static=MetricKitBridge");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    if let Some(sdk_root) = sdk_root() {
        let sdk_swift_path = format!("{sdk_root}/usr/lib/swift");
        println!("cargo:rustc-link-search=native={sdk_swift_path}");

        for library in [
            "swiftCore",
            "swiftCoreFoundation",
            "swiftFoundation",
            "swiftDispatch",
            "swiftObjectiveC",
            "swiftDarwin",
            "swiftOSLog",
            "swiftMetricKit",
            "swift_Concurrency",
            "swiftSwiftOnoneSupport",
        ] {
            println!("cargo:rustc-link-lib={library}");
        }
    }

    if let Some(xcode_path) = xcode_select_path() {
        let swift_lib_path_old =
            format!("{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift-5.5/macosx");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{swift_lib_path_old}");

        let swift_lib_path =
            format!("{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
        println!("cargo:rustc-link-search=native={swift_lib_path}");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{swift_lib_path}");

        for library in [
            "swiftCompatibility50",
            "swiftCompatibility51",
            "swiftCompatibility56",
            "swiftCompatibilityConcurrency",
            "swiftCompatibilityDynamicReplacements",
            "swiftCompatibilityPacks",
            "swiftCxx",
            "swiftCxxStdlib",
        ] {
            println!("cargo:rustc-link-lib={library}");
        }
    }
}

fn sdk_root() -> Option<String> {
    let output = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn xcode_select_path() -> Option<String> {
    let output = Command::new("xcode-select").arg("-p").output().ok()?;
    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

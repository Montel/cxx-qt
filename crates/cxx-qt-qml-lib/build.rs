// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    let feature_qt_gui_enabled = std::env::var("CARGO_FEATURE_QT_GUI").is_ok();
    let feature_qt_qml_enabled = std::env::var("CARGO_FEATURE_QT_QML").is_ok();
    let mut qt_modules = vec!["Core".to_owned()];
    if feature_qt_qml_enabled {
        qt_modules.push("Qml".to_owned());
    }
    if feature_qt_gui_enabled {
        qt_modules.push("Gui".to_owned());
    }


    let qtbuild = qt_build_utils::QtBuild::new(qt_modules).expect("Could not find Qt installation");

    // Required for tests
    qt_build_utils::setup_linker();

    // Find the Qt version and tell the Rust compiler
    // this allows us to have conditional Rust code
    println!(
        "cargo:rustc-cfg=qt_version_major=\"{}\"",
        qtbuild.version().major
    );

    let mut rust_bridges = vec![
    ];

    if feature_qt_qml_enabled {
        rust_bridges.extend(["qml/qqmlapplicationengine", "qml/qqmlengine"]);
    }

    for bridge in &rust_bridges {
        println!("cargo:rerun-if-changed=src/{bridge}.rs");
    }

    for include_path in qtbuild.include_paths() {
        cxx_build::CFG
            .exported_header_dirs
            .push(include_path.as_path());
    }

    let mut builder =
        cxx_build::bridges(rust_bridges.iter().map(|bridge| format!("src/{bridge}.rs")));

    qtbuild.cargo_link_libraries(&mut builder);

    let mut cpp_files = vec![
    ];

    if feature_qt_qml_enabled {
        cpp_files.extend(["qml/qqmlapplicationengine", "qml/qqmlengine"]);
    }

    for cpp_file in &cpp_files {
        builder.file(format!("src/{cpp_file}.cpp"));
        println!("cargo:rerun-if-changed=src/{cpp_file}.cpp");
    }

    // Write this library's manually written C++ headers to files and add them to include paths
    let out_dir = std::env::var("OUT_DIR").unwrap();
    cxx_qt_qml_lib_headers::write_headers(format!("{out_dir}/cxx-qt-qml-lib"));
    builder.include(out_dir);

    // Enable Qt Qml in C++ if the feature is enabled
    if feature_qt_gui_enabled {
        builder.define("CXX_QT_QML_FEATURE", None);
    }
    // Enable Qt Qml in C++ if the feature is enabled
    if feature_qt_gui_enabled {
        builder.define("CXX_QT_QML_FEATURE", None);
    }
    // Note, ensure our settings stay in sync across cxx-qt-build and cxx-qt-core-lib
    builder.cpp(true);
    // MSVC
    builder.flag_if_supported("/std:c++17");
    builder.flag_if_supported("/Zc:__cplusplus");
    builder.flag_if_supported("/permissive-");
    builder.flag_if_supported("/bigobj");

    // GCC + Clang
    builder.flag_if_supported("-std=c++17");
    // MinGW requires big-obj otherwise debug builds fail
    builder.flag_if_supported("-Wa,-mbig-obj");

    builder.compile("cxx-qt-qml-lib");
}

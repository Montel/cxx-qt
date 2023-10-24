// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]

//! This crate is a hack so build.rs for cxx-qt-qml-lib and cxx-qt-build both have access to cxx-qt-qml-lib's C++ headers.
//! This must be a separate crate from cxx-qt-qml-lib because cxx-qt-qml-lib cannot be a build dependency of cxx-qt-build.
//! Otherwise Cargo links the executable compiled from a build.rs that uses cxx-qt-build to Qt, so running
//! build.rs fails when Qt is linked dynamically if the Qt libraries are not in PATH (Windows)/LD_LIBRARY_PATH (Unix).

use std::{fs::File, io::Write, path::Path};

/// Write the cxx-qt-qml-lib headers to the specified directory.
pub fn write_headers(directory: impl AsRef<Path>) {
    let directory = directory.as_ref();
    std::fs::create_dir_all(directory).expect("Could not create cxx-qt-qml-lib header directory");
    for (file_contents, file_name) in [
        #[cfg(feature = "qt_qml")]
        (include_str!("../include/qml/qqmlengine.h"), "qqmlengine.h"),
    ] {
        // Note that we do not need rerun-if-changed for these files
        // as include_str causes a rerun when the header changes
        // and the files are always written to the target.
        let h_path = format!("{}/{file_name}", directory.display());
        let mut header = File::create(h_path).expect("Could not create cxx-qt-qml-lib header");
        write!(header, "{file_contents}").expect("Could not write cxx-qt-qml-lib header");
    }
}

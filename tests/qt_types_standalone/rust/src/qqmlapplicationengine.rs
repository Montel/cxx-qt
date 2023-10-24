// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_core_lib::{QUrl};
use cxx_qt_qml_lib::{QQmlApplicationEngine}

#[cxx::bridge]
mod qqmlapplicationengine_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-qml-lib/qqmlapplicationengine.h");
        type QQmlApplicationEngine = cxx_qt_qml_lib::QQmlApplicationEngine;
    }

    extern "Rust" {
        fn construct_qqmlapplicationengine() -> UniquePtr<QQmlApplicationEngine>;
        fn read_qqmlapplicationengine(c: &QQmlApplicationEngine) -> bool;
    }
}

fn construct_qqmlapplicationengine() -> cxx::UniquePtr<QQmlApplicationEngine> {
    let mut engine = QQmlApplicationEngine::new();
    if let Some(engine) = engine.as_mut() {
        engine.set_base_url(&QUrl::from("qrc:/kdab.qml"));
    }
    engine
}

fn read_qqmlapplicationengine(engine: &QQmlApplicationEngine) -> bool {
    engine.base_url().to_string() == "qrc:/kdab.qml"
}

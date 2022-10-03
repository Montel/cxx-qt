// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "custom_base")]
mod ffi {
    unsafe extern "C++" {
        include!(<QtCore/QStringListModel>);
    }

    #[cxx_qt::qobject(base = "QStringListModel")]
    #[derive(Default)]
    pub struct CustomBase;
}
// ANCHOR_END: book_macro_code

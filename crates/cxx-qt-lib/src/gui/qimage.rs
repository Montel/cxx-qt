// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qimage.h");
        type QImage = super::QImage;
        include!("cxx-qt-lib/qsize.h");
        type QSize = crate::QSize;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;


        /// Whether the QImage is null.
        ///
        /// This means that the QImage has all parameters set to zero and no allocated data.
        #[rust_name = "is_null"]
        fn isNull(self: &QImage) -> bool;

        /// Returns true if all the colors in the image are shades of gray
        #[rust_name = "all_gray"]
        fn allGray(self: &QImage) -> bool;

        /// For 32-bit images, this function is equivalent to allGray().
        /// For color indexed images, this function returns true if color(i) is QRgb(i, i, i) 
        /// for all indexes of the color table; otherwise returns false.
        #[rust_name = "is_gray_scale"]
        fn isGrayscale(self: &QImage) -> bool;

        /// Returns true if the image has a format that respects the alpha channel, otherwise returns false.
        #[rust_name = "has_alpha_channel"]
        fn hasAlphaChannel(self: &QImage) -> bool;

        /// Returns the size of the color table for the image.
        #[rust_name = "color_count"]
        fn colorCount(self: &QImage) -> i32;

        /// Returns the depth of the image.
        fn depth(self: &QImage) -> i32;

        /// Returns the height of the image.
        fn height(self: &QImage) -> i32;

        /// Returns the width of the image.
        fn width(self: &QImage) -> i32;

        /// Returns the size of the image.
        fn size(self: &QImage) -> QSize;

        /// Returns the enclosing rectangle (0, 0, width(), height()) of the image.
        fn rect(self: &QImage) -> QRect;

        /// Returns the color of the pixel at coordinates (x, y) as a QColor.
        #[rust_name = "pixel_color"]        
        fn pixelColor(self: &QImage, x: i32, y: i32) -> QColor;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qimage_drop"]
        fn drop(image: &mut QImage);

        #[doc(hidden)]
        #[rust_name = "qimage_init_from_data"]
        fn qimageInitFromData(data: &[u8], format: &str) -> QImage;
    }
}

/// > ⚠ **Warning**: The QImage API in CXX-Qt-lib is not yet complete and subject to change.
///
/// This struct is the Rust representation of the [`QImage`](https://doc.qt.io/qt-6/qimage.html)
/// class.
///
/// It provides a way to store and manipulate images in a hardware-independent manner.
#[repr(C)]
pub struct QImage {
    // Static checks on the C++ side ensure this is true.
    // See qcolor.cpp
    #[cfg(qt_version_major = "5")]
    _data: MaybeUninit<[usize; 4]>,
    #[cfg(qt_version_major = "6")]
    _data: MaybeUninit<[usize; 3]>,
}

// Safety:
//
// Static checks on the C++ side to ensure the size & alignment is the same.
unsafe impl ExternType for QImage {
    type Id = type_id!("QImage");
    type Kind = cxx::kind::Trivial;
}

impl Drop for QImage {
    fn drop(&mut self) {
        ffi::qimage_drop(self);
    }
}

impl QImage {
    /// Convert raw image data to a [`QImage`].
    ///
    /// The data must be in the given `format`.
    /// See [`QImageReader::supportedImageFormats()`](https://doc.qt.io/qt-6/qimagereader.html#supportedImageFormats) for the list of supported formats.
    ///
    /// If no `format` is provided, the format will be quessed from the image header.
    pub fn from_data(data: &[u8], format: Option<&str>) -> Option<Self> {
        let image = ffi::qimage_init_from_data(data, format.unwrap_or(""));

        if !image.is_null() {
            Some(image)
        } else {
            None
        }
    }
}

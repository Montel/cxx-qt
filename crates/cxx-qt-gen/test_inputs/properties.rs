#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-core-lib/qpoint.h");
        type QPoint = cxx_qt_core_lib::QPoint;
    }

    extern "RustQt" {
        #[qobject]
        #[derive(Default)]
        #[qproperty(i32, primitive)]
        #[qproperty(QPoint, trivial)]
        type MyObject = super::MyObjectRust;
    }
}

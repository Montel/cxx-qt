mod my_object {
    use cxx_qt_lib::QString;

    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {}

        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QPointF = cxx_qt_lib::QPointF;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;
            #[namespace = ""]
            type QVariant = cxx_qt_lib::QVariant;

            #[namespace = "CxxQt"]
            type Variant = cxx_qt_lib::Variant;

            #[rust_name = "new_cpp_object"]
            fn newCppObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "testPointfWrapper"]
            fn test_pointf_wrapper(
                self: &RustObj,
                _cpp: Pin<&mut MyObject>,
                pointf: &QPointF,
            ) -> QPointF;

            #[cxx_name = "testStringWrapper"]
            fn test_string_wrapper(
                self: &RustObj,
                _cpp: Pin<&mut MyObject>,
                string: &QString,
            ) -> String;

            #[cxx_name = "testVariantWrapper"]
            fn test_variant_wrapper(
                self: &RustObj,
                _cpp: Pin<&mut MyObject>,
                variant: &QVariant,
            ) -> Variant;

            #[cxx_name = "createRs"]
            fn create_rs() -> Box<RustObj>;

            #[cxx_name = "initialiseCpp"]
            fn initialise_cpp(cpp: Pin<&mut MyObject>);
        }
    }

    pub type FFICppObj = ffi::MyObject;
    pub type Property = ffi::Property;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn test_pointf_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            pointf: &QPointF,
        ) -> QPointF {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_pointf(&mut _cpp, pointf);
        }

        fn test_string_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            string: &QString,
        ) -> String {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_string(&mut _cpp, string);
        }

        fn test_variant_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            variant: &QVariant,
        ) -> Variant {
            let mut _cpp = CppObj::new(_cpp);
            return self.test_variant(&mut _cpp, variant);
        }

        fn test_pointf(&self, _cpp: &mut CppObj, pointf: &QPointF) -> QPointF {
            pointf
        }

        fn test_string(&self, _cpp: &mut CppObj, string: &QString) -> String {
            string.to_rust()
        }

        fn test_variant(&self, _cpp: &mut CppObj, variant: &QVariant) -> Variant {
            variant
        }
    }

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
            use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

            let ptr: *const FFICppObj = unsafe { &*self.cpp.as_ref() };
            unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
        }

        pub fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;
        }
    }

    #[derive(Default)]
    struct Data;

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(_value: &CppObj<'a>) -> Self {
            Self {}
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(_value: &mut CppObj<'a>) -> Self {
            Self::from(&*_value)
        }
    }

    fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(&Data::default());
    }
}
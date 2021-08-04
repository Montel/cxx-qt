mod my_object {
    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            type QString = cxx_qt_lib::QString;

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type MyObjectRs;

            #[cxx_name = "sayHi"]
            fn say_hi(self: &MyObjectRs, string: &str, number: i32);
            #[cxx_name = "sayBye"]
            fn say_bye(self: &MyObjectRs);

            #[cxx_name = "getNumber"]
            fn number(self: &MyObjectRs) -> &i32;
            #[cxx_name = "setNumber"]
            fn set_number(self: &mut MyObjectRs, value: i32);

            #[cxx_name = "getString"]
            fn string(self: &MyObjectRs) -> &String;
            #[cxx_name = "setString"]
            fn set_string(self: &mut MyObjectRs, value: String);

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<MyObjectRs>;
        }
    }

    pub type CppObj = ffi::MyObject;

    #[derive(Default)]
    struct MyObjectRs {
        number: i32,
        string: String,
    }

    impl MyObjectRs {
        fn say_hi(&self, string: &str, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        fn say_bye(&self) {
            println!("Bye from Rust!");
        }

        fn number(self: &MyObjectRs) -> &i32 {
            &self.number
        }

        fn set_number(self: &mut MyObjectRs, value: i32) {
            self.number = value;
        }

        fn string(self: &MyObjectRs) -> &String {
            &self.string
        }

        fn set_string(self: &mut MyObjectRs, value: String) {
            self.string = value;
        }
    }

    #[derive(Default)]
    struct MyObjectData {
        number: i32,
        string: String,
    }

    impl From<MyObjectData> for MyObjectRs {
        fn from(value: MyObjectData) -> Self {
            Self {
                number: value.number,
                string: value.string,
            }
        }
    }

    impl From<&MyObjectRs> for MyObjectData {
        fn from(value: &MyObjectRs) -> Self {
            Self {
                number: value.number.clone(),
                string: value.string.clone(),
            }
        }
    }

    fn create_my_object_rs() -> Box<MyObjectRs> {
        Box::new(MyObjectRs::default())
    }
}
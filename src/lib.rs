#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    extern "Rust" {
        type Object;
        fn new_object() -> Box<Object>;
        fn get_name(&mut self) -> String;
    }
}

fn new_object() -> Box<Object> {
    Box::new(Object { name: "hello".to_string() } )
}

struct Object {
    name: String,
}

impl Object {
    fn get_name(&mut self) -> String {
        self.name.clone()
    }
}
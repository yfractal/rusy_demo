#[macro_use]
extern crate rutie;
extern crate lazy_static;

use rutie::types::Value;
use rutie::{AnyObject, Class, NilClass, Object, RString};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Store {
    hash_map: HashMap<String, Arc<AnyObject>>,
}

impl Store {
    fn new() -> Self {
        Store {
            hash_map: HashMap::new(),
        }
    }
}

wrappable_struct!(Store, StoreWrapper, STORE_WRAPPER);
class!(RubyStore);

methods_safe!(
  RubyStore,
  rtself,
  fn new() -> AnyObject {
    let store = Store::new();
    Class::from_existing("RubyStore").wrap_data(store, &*STORE_WRAPPER)
  },
  fn insert(key: &RString, obj: &AnyObject) -> AnyObject {
    let rbself = rtself.get_data_mut(&*STORE_WRAPPER);
    let obj = obj.as_ref().unwrap(); // is &AnyObject
    rbself
        .hash_map
        .insert(key.as_ref().unwrap().to_string(), Arc::new(obj.clone()));
    NilClass::new().into()
  },
  fn get(rb_key: &RString) -> AnyObject {
    let rbself = rtself.get_data_mut(&*STORE_WRAPPER);

    let key = rb_key.as_ref().unwrap().to_string();
    let val = rbself.hash_map.get(&key).unwrap();

    (*val.as_ref()).clone()
  },
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rusy_demo() {
    Class::new("RubyStore", None).define(|klass| {
        klass.def_self("new", new);
        klass.def("insert", insert);
        klass.def("get", get);
    });
}

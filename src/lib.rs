#[macro_use]
extern crate rutie;
extern crate lazy_static;

use rutie::types::Value;
use rutie::{AnyObject, Class, NilClass, Object, RString};
use std::collections::HashMap;
use std::sync::Arc;

pub struct RubyObject {
    value: Value,
}

impl RubyObject {
    pub fn value(&mut self) -> Value {
        self.value
    }
}

impl From<Value> for RubyObject {
    fn from(value: Value) -> Self {
        RubyObject { value }
    }
}

impl Into<AnyObject> for RubyObject {
    fn into(self) -> AnyObject {
        AnyObject::from(self.value)
    }
}

pub struct Store {
    hash_map: HashMap<String, Arc<RubyObject>>,
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

methods!(
    RubyStore,
    rtself,
    fn ruby_new() -> AnyObject {
        let store = Store::new();
        Class::from_existing("RubyStore").wrap_data(store, &*STORE_WRAPPER)
    },
    fn ruby_insert(key: RString, obj: AnyObject) -> AnyObject {
        let rbself = rtself.get_data_mut(&*STORE_WRAPPER);

        let ruby_obj = RubyObject::from(obj.unwrap().value());

        rbself
            .hash_map
            .insert(key.unwrap().to_string(), Arc::new(ruby_obj));
        NilClass::new().into()
    },
    fn ruby_get(rb_key: RString) -> AnyObject {
        let rbself = rtself.get_data_mut(&*STORE_WRAPPER);

        let key = rb_key.unwrap().to_string();
        let val = rbself.hash_map.get(&key).unwrap();

        AnyObject::from(val.value)
    },
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rusy_demo() {
    Class::new("RubyStore", None).define(|klass| {
        klass.def_self("new", ruby_new);
        klass.def("insert", ruby_insert);
        klass.def("get", ruby_get);
    });
}

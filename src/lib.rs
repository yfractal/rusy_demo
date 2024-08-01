#[macro_use]
extern crate rutie;
extern crate lazy_static;

use rutie::types::Value;
use rutie::{AnyObject, Class, NilClass, Object, RString};
use std::collections::HashMap;
use std::sync::Arc;
use concat_idents::concat_idents;

macro_rules! define_function_with_deref {
  (
      fn $method_name:ident
      ($($arg_name:ident: &$arg_type:ty),*) -> $return_type:ty $body:block
  ) => {
      fn $method_name($($arg_name: $arg_type),*) -> $return_type {
          // Dereference each argument that is a reference to AnyObject
          $(
              let $arg_name = &$arg_name;
          )*
          $body
      }
  };
}


define_function_with_deref! {
  fn ruby_insertx(key: &RString, obj: &AnyObject) -> AnyObject {
      println!("Key: {:?}", key);
      println!("Obj: {:?}", obj);

      obj.clone()
  }
}

define_function_with_deref! {
  fn ruby_insertxxxx(key: &RString, obj: &AnyObject) -> AnyObject {
    obj.clone()
  }
}

fn do_ruby_insert(store: &mut Store, key: String, obj: &RubyObject) {
  store.hash_map.insert(key, Arc::new(obj.clone()));
}

pub struct RubyObject {
    value: Value,
}

impl RubyObject {
    pub fn value(&mut self) -> Value {
        self.value
    }
}

impl Clone for RubyObject {
  fn clone(&self) -> Self {
      RubyObject {
        // TODO: record this value
        // TODO: impl drop
        value: self.value.clone(),
      }
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

        // let ruby_obj = RubyObject::from(obj.unwrap().value());

        // rbself
        //     .hash_map
        //     .insert(key.unwrap().to_string(), Arc::new(ruby_obj));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
      // let ruby_object = RubyObject { value: Value::new() };
      // let arc = Arc::new(&ruby_object);
    }
}

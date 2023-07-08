#[macro_use]
extern crate rutie;

use rutie::{Array, Class, Hash, Integer, Object, RString, Symbol, VM};
use std::collections::HashMap;

fn key() -> Symbol {
    Symbol::new(&"racksonracks.response")
}

class!(RacksOnRacks);

methods!(
    RacksOnRacks,
    _rtself,
    fn pub_call(env: Hash) -> Hash {
        let mut ruby_hash = env.map_err(|e| VM::raise_ex(e)).expect("hash");

        // Call the middleware function, if it returns a response, put it back into the env hash
        // which will be picked up by the small Ruby middleware
        if let Some(response) = app::call(&EnvHash::new(&ruby_hash)) {
            ruby_hash.store(key(), response.to_rack_response());
        }

        ruby_hash
    },
    fn pub_key() -> Symbol {
        key()
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_racksonracks() {
    Class::new("RacksOnRacks", None).define(|klass| {
        klass.def_self("call", pub_call);
        klass.def_self("key", pub_key);
    });
}

pub struct EnvHash<'a> {
    env: &'a Hash,
}

impl<'a> EnvHash<'a> {
    pub fn new(env: &'a Hash) -> EnvHash<'a> {
        EnvHash { env }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.env
            .at(&RString::new_utf8(&key))
            .try_convert_to::<RString>()
            .ok()
            .map(|rs| rs.to_string())
    }
}

impl<'a> From<&EnvHash<'a>> for http::Request<String> {
    fn from(env: &EnvHash<'a>) -> Self {
        use http::method::Method;
        use http::request::Request;
        use http::uri::Uri;

        let method = env
            .get("REQUEST_METHOD")
            .as_ref()
            .map(|s| Method::try_from(s.as_str()).ok())
            .flatten()
            .unwrap_or(Method::POST);

        let uri = env
            .get("REQUEST_URI")
            .as_ref()
            .map(|s| s.parse::<Uri>().ok())
            .flatten()
            .unwrap();

        let mut request: Request<String> = Request::default();
        *request.method_mut() = method;
        *request.uri_mut() = uri;
        request
    }
}

pub struct Response {
    status: i64,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn new<S: Into<String>>(
        status: i64,
        headers: HashMap<String, String>,
        body: S,
    ) -> Response {
        Response {
            status,
            headers,
            body: body.into(),
        }
    }

    pub fn to_rack_response(self) -> Array {
        let status = Integer::new(self.status);
        let headers = self
            .headers
            .into_iter()
            .fold(Hash::new(), |mut hash, (k, v)| {
                hash.store(RString::new_utf8(&k), RString::new_utf8(&v));
                hash
            });

        // Body must response to #each in Ruby land, the easiest way to do that is to wrap the
        // string response in an Array
        let mut body = Array::with_capacity(1);
        body.push(RString::new_utf8(&self.body));

        // [status, headers, body]
        let mut rack_response = Array::with_capacity(3);
        rack_response.push(status);
        rack_response.push(headers);
        rack_response.push(body);

        rack_response
    }
}

// Represents the "app" code
mod app {
    use super::{EnvHash, Response};
    use std::collections::HashMap;

    // NOTE: If any code panics the Ruby VM will crash
    pub fn call(env: &EnvHash) -> Option<Response> {
        let request = http::Request::from(env);

        // Routing
        match (request.method(), request.uri().path()) {
            (&http::method::Method::GET, "/rust") => Some(Response::new(
                200,
                HashMap::new(),
                "Greetings from RacksOnRacks\n",
            )),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        VM::init();
        let mut env_hash = Hash::new();
        env_hash.store(RString::new_utf8("foo"), RString::new_utf8("bar"));
        let env = EnvHash::new(&env_hash);
        assert!(env.get("foo").is_some());
        // FIXME: This fails
        assert!(env.get("bar").is_some());
    }
}

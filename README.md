# RacksOnRacks

Write Rack middleware in Rust.

This repo is a proof of concept and should be used as an example. The best way to use this at the moment would be to copy the code into your own repo and take it from there.

It's a small wrapper around [rutie][1], which is doing all the heavy lifting.

Example middleware written in Rust:

```rust
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
```

Example `config.ru`:

```ruby
require "lib.rb"

class App
  def call(env)
    [200, {"Content-Type" => "text/plain"}, ["Greetings from Ruby\n"]]
  end
end

# Initialize our normal rack app
app = App.new

# Wrap the rack app in our middleware
app = RacksOnRacksMiddleware.new(app)

run app
```

## Running the example code

```
$ cd test-app
$ cargo build --release && puma
# ...
$ curl localhost:9292/anything
Greetings from Ruby
$ curl localhost:9292/rust
Greetings from RacksOnRacks
```

[1]: https://github.com/danielpclark/rutie

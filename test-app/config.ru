require_relative "../src/lib.rb"

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

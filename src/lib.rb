require "rutie"

Rutie.new(:racksonracks).init "Init_racksonracks", __dir__

class RacksOnRacksMiddleware
  def initialize(app)
    @app = app
  end

  def call(env)
    response = begin
      RacksOnRacks.call(env)[RacksOnRacks.key]
    rescue => e
      puts "ERROR: #{e}"
      nil
    end

    response || @app.call(env)
  end
end

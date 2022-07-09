require 'socket'

PORT = 8080
server = TCPServer.new('0.0.0.0', PORT)

# one client
# client = server.accept
# puts "connected"
# client.puts "Hello There. I am Cosmic Server"
# client.close


# muptiple client
puts "Listening on #{PORT}. Press ctrl+C to cancel."

def handle_connection(client)
    puts "New client #{client}"

    client.puts "Hello from server"
    client.close
end

loop do
    client = server.accept

    Thread.new { handle_connection(client) }
end


require 'socket'

PORT = 8080
client = TCPSocket.new('localhost', PORT)

while data = client.gets
    puts data
end

client.close
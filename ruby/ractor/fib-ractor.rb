require 'benchmark'
require 'ractor'
require_relative './fibonnacci'

puts Benchmark.measure {
  8.times.map {
    Ractor.new { fib(38) }
  }.each { |r| p r.take }
}

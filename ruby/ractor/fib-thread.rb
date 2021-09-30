require 'benchmark'
require_relative './fibonnacci'

puts Benchmark.measure {
  4.times.map {
    Thread.new { fib(38) }
  }.each { |t| p t.value }
}

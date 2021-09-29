code =<<-RUBY

p 'hello world'

RUBY

puts RubyVM::InstructionSequence.new(code).disasm
code =<<-RUBY

def add(x)
  # x + 2
  2 + 3 * 5 - 1
end

RUBY

puts RubyVM::InstructionSequence.new(code).disasm
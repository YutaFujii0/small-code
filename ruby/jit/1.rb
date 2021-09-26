code =<<-RUBY

p 'hello world'

RUBY

p RubyVM::InstructionSequence.new(code).disasm
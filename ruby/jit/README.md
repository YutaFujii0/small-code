

* rb_iseq_compile_node function (compile.c)
https://github.com/ruby/ruby/blob/806a27f98b5d414f8e7daa853072113ec41451fb/compile.c#L742

* iseq_compile_each function (compile.c)
https://github.com/ruby/ruby/blob/806a27f98b5d414f8e7daa853072113ec41451fb/compile.c#L8964

this converts node type to instruction set

* insns.def (instruction set to VM stack behaviour)
https://github.com/ruby/ruby/blob/master/insns.def

```bash
ruby -e'p RubyVM::InstructionSequence.new("p 1").disasm'
```
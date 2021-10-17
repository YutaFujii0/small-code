# Zero Cost Abstraction

What does this mean?
- You can add abstractions without affecting runtime performance
- improve code quality and readability without any runtime performance cost
- No longer have to trade off between maintainability and performance

# example

```rust
fn add(x: i32) -> i32 {
    x + 2 - 3 * 5 - x
}
```

this function does not use x, but just return -13 whatever x is.
So, compiler would optimize this.

# Tips

how to write output to assenbly code|[stackoverflow](https://stackoverflow.com/questions/39219961/how-to-get-assembly-output-from-building-with-cargo)

```bash
cargo rustc -- --emit asm

# for release mode: I recommend this
cargo rustc --release -- --emit asm

# --emit mir (rust intermediate representation)
# --emit llvm-ir (llvm intermediate representation)
# --emit llvm-bc (llvm byte code)
# --emit asm (assembly)
```


Another way, with [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) installed.

use cargo objdump
```bash
# installation
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview

# disassemble a binary
$ cargo objdump --release -- --disassemble --no-show-raw-insn
```

## Reading disassembly code
general purpose registers
rax: accumulator
rbx: base
rcx: counter
rdx: data

pointers/indexes
rsi: register stack index
rdi: register destination index
rbp: base pointer
rsp: source pointer

control pointer
rip: instruction pointer

>The first four registers rax, rbx, rcx, and rdx, are general purpose registers accumulator, base, counter and data respectively. These registers can be used for a whole host of things, however, in this case, they are used as temporary variables for the CPU, when executing machine code.

>The next four registers rsi, rdi, rbp and rsp are also general purpose, better known as pointers and indexes, the registers above stand for stack index, destination index, base pointer and source pointer, which are used for storing addresses that points to location in memory.

>The rip register is the instruction pointer and will keep track of what the CPU is reading. It is an important register to concentrate on while debugging, as it is essentially the control pointer. The example above shows the CPU is currently pointing at memory address 0x4004f8.

>The final set of eflags registers contain several big flags and are used for comparison and memory segmentation.

https://perspectiverisk.com/intro-to-basic-disassembly-reverse-engineering/


## Assembly x86

```rust
fn main() {
    // example2();
    example();
}

fn example() {
    // println!("{}", example::do_stuff(3.0));
    println!("{}", 6.0);
}
```

results in
```asm
_main:
	.cfi_startproc  :beginning of a function
	pushq	%rbp    :push source(rbp) to stack
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16 :save the preveous value of register(%rbp) to offset from CFA
	movq	%rsp, %rbp :move source to base pointer(rsp to rbp)
	.cfi_def_cfa_register %rbp  :offset remains the same, 
	subq	$16, %rsp  :subtract 16 from rsp, what's in rsp here?
	movq	%rsi, %rcx :move stack index to rcx
	movslq	%edi, %rdx  :3bytes of 8byte register moved to data
	leaq	__ZN21zero_cost_abstraction4main17h2402699a3191719cE(%rip), %rax
	movq	%rax, -8(%rbp)
	leaq	l___unnamed_1(%rip), %rsi
	leaq	-8(%rbp), %rdi
	callq	__ZN3std2rt19lang_start_internal17h5f9b030f0a63c040E
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

```


## materials
* An introduction to structs, traits, and zero-cost abstractions by Tim McLean - Rust KW Meetup
https://www.youtube.com/watch?v=Sn3JklPAVLk&t=237s

* When zero cost abstraction aren't zero cost
https://blog.polybdenum.com/2021/08/09/when-zero-cost-abstractions-aren-t-zero-cost.html

* The Embedded Rust Book
https://doc.rust-lang.org/beta/embedded-book/static-guarantees/zero-cost-abstractions.html


* Oracle Solaris x86 assembly language syntax(some of them may be different from macOSX)
https://docs.oracle.com/cd/E37838_01/html/E61064/eoiyg.html

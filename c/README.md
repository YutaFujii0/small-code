# Some Tips for Programming in C

1. Preprocessor Stuff
2. Variable Declarations
3. Type Definitions
4. Function Definitions
5. Statements & Expressions
6. Comments

# Type in C

- Use `int8_t` or `uint64_t` (stdint.h) rather than `int` to avoid dependency to your system
  - `int` usually 4bytes but some micro processor don't


# Buffer

### Difference between line buffered / fully buffered
- STDOUT is usually line buffered, each time new line is detected I/O is performed
- other streams are usually fully buffered, I/O won't be performed until flush is fired

```bash
$ echo "hello\nworld" > tmp

$ ./io-buff < tmp
# ->
# hello (after 5sec)
# world (after 10sec)
# This is because STDOUT is usually line buffered, each time new line is detected I/O is performed

$ ./io-buff < tmp > output
# ->
# after 10sec, "hello\nworld" is written to output
# This is because file is usually fully buffered, I/O won't be performed until flush is fired
```

### charcter-at-a-time vs line-at-a-time

- Given an input of million lines of text(10 characters for each line)
- `getc` and `fgetc` read one character at a time, thus 10 millions of loop occurs
- `fgets` reads line at a time, thus 1 millions of loop needed
- `read` reads buf at a time(given in source code), so optimization need to be done by user
- all those functions/macros have little difference in system CPU time

```bash
# create input file (I created 12M bytes, 1 million lines of text)
$ touch input
$ hyperfine --warmup 5 "./io-getc < input"
  Time (mean ± σ):     836.4 ms ±   5.8 ms    [User: 823.9 ms, System: 7.4 ms]
  Range (min … max):   829.7 ms … 849.5 ms    10 runs

$ hyperfine --warmup 5 "./io-fgetc < input"
  Time (mean ± σ):     837.0 ms ±   3.3 ms    [User: 824.0 ms, System: 7.5 ms]
  Range (min … max):   833.6 ms … 845.4 ms    10 runs

$ hyperfine --warmup 5 "./io-fgets < input"
  Time (mean ± σ):      90.1 ms ±   4.2 ms    [User: 80.6 ms, System: 5.6 ms]
  Range (min … max):    87.0 ms … 103.5 ms    31 runs

$ hyperfine --warmup 5 "./io-read < input"
  Time (mean ± σ):      10.8 ms ±   1.6 ms    [User: 2.4 ms, System: 5.1 ms]
  Range (min … max):     9.5 ms …  29.2 ms    158 runs
```

### write vs printf

- write is non buffered(precisely it's just kernel buffered, not in main memory)
  - I/O occurs immediately
- printf(standard I/O library) uses buffer
  - if file desctipter points to STDOUT(terminal), new line will fire I/O
  - if file descripter points to a file entry, I/O occurs at the end of the process

```bash
$ ./ps-fork
# `printf` buffer string array and that fire I/O to the terminal before fork()

$ ./ps-fork > tmp
# `printf` buffer string array and that is copied by fork(), thus sentence is put twice
```

# Process

### ppid

- when child terminates before parent, it can be a zombie process
- when parent terminates before child, ppid of child process become 1(init process)

```bash
$ ./ps-ppid
before fork
hello from child process
pid = 45942, ppid = 45941
hello from parent process
hello from parent process-2
#(here terminal returns)
pid = 45950, ppid = 1
```

### Orphaned process group

- a process whose parent terminates is called an orphan
- even if such a situation happens, child process would remain running

```bash
$ echo "foo" > tmp

$ ./ps-orphaned < tmp
parent: pid = 58195, ppid = 7860, pgrp = 58195, tpgrp = -1
child: pid = 58196, ppid = 58195, pgrp = 58195, tpgrp = -1
SIGHUP received, pid = 58196
child: pid = 58196, ppid = 1, pgrp = 58195, tpgrp = -1
hello world%
# you'll see read and write actually executed!
```

# Resources

- Jacob Sorber[Youtube](https://www.youtube.com/watch?v=SC8uWXmDJs4&list=PL9IEJIKnBJjG5H0ylFAzpzs9gSmW_eICB)
- [Advanced Programming in the UNIX® Environment, Third Edition](http://www.apuebook.com/code3e.html)

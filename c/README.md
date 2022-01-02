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

# Resources

- Jacob Sorber[Youtube](https://www.youtube.com/watch?v=SC8uWXmDJs4&list=PL9IEJIKnBJjG5H0ylFAzpzs9gSmW_eICB)
- [Advanced Programming in the UNIX® Environment, Third Edition](http://www.apuebook.com/code3e.html)

# Command Line Calculator

This is a simple yet powerful calculator built with programmers in mind. It 
supports all standard arithmetic, bitwise and logical operators, numerous 
mathematical functions and numbers with differing bases. It also features a
lose "type system" allowing calculations to be performed on fixed-width
integers. The calculator also maintains a persistent buffer of previous 
results that can be used in your calculations with the reference syntax: `$0`,
`$1`, etc.

While this can be used as a simple shell calculator, it was originally written
to be used with the popular macOS productivity tool [Alfred](https://www.alfredapp.com/). 
As such, support for Alfred has been built-in to the calculator itself, and the provided 
Makefile can be used to build the Alfred workflow.

## Options

By default, expressions are read from stdin but you may also supply the 
them in a file with the `-f` option, or on the command line using the `-e`
option.

The buffer file is located by default at `$HOME/.clc_history` but this 
location can be changed by supplying the `-B` option.

```
USAGE:
    clc [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -b, --buffer-size <SIZE>    Set the max buffer size [default: 32]
    -B, --buffer-file <FILE>    Specify an alternate buffer file
    -e, --expr <EXPRESSION>     Expression to evaluate
    -f, --file <FILE>           Read program from file
    -o <FORMAT>                 Output format [all|bin|hex|oct|alfred]
```

## Built-ins

The following types are supported:

- `i8/u8` - Signed/unsigned 8-bit integral types
- `i16/u16` - Signed/unsigned 16-bit integral types
- `i32/u32` - Signed/unsigned 32-bit integral types
- `i64/u64` - Signed/unsigned 64-bit integral types
- `f64` - Double precision floating point type


### Constants

| **Name**     | **Description**          | **Type** |
|--------------|--------------------------|----------|
| `PI`         | Archimedes' constant (π) | `f64`    |
| `E`          | Euler's number (ℇ)       | `f64`    |
| `NAN`        | Not a number (NaN)       | `f64`    |
| `INF`        | Infinity (∞)             | `f64`    |
| `NEG_INF`    | Negative infinity (-∞)   | `f64`    |
| `MIN_<type>` | Minimum value of type    | `type`   |
| `MAX_<type>` | Maximum value of type    | `type`   |   

### Functions

| **Name**  | **Description**                     | **Type** |
|-----------|-------------------------------------|----------|
| `abs()`   | Absolute value function             | `type`   |
| `sin()`   | Compute sine of number              | `f64`    |
| `cos()`   | Compute cosine of number            | `f64`    |
| `tan()`   | Compute tangent of number           | `f64`    |
| `asin()`  | Compute arcsine of number           | `f64`    |
| `acos()`  | Compute arccosine of number         | `f64`    |
| `atan()`  | Compute arctangent of number        | `f64`    |
| `floor()` | Rounds down to nearest whole number | `f64`    |
| `ceil()`  | Rounds up to nearest whole number   | `f64`    |
| `round()` | Rounds to nearest whole number      | `f64`    |
| `sqrt()`  | Computes square root of number      | `f64`    |
| `exp()`   | Returns `E` to the power of number  | `f64`    |
| `ln()`    | Compute natural log of number       | `f64`    |
| `log2()`  | Compute base 2 logarithm of number  | `f64`    |
| `log10()` | Compute base 10 logarithm of number | `f64`    |
| `deg()`   | Converts degrees to radians         | `f64`    |
| **Name**  | **Description**                     | **Type** |
| `u64()`   | Casts number to u64                 | `u64`    |
| `u32()`   | Casts number to u32                 | `u32`    |
| `u16()`   | Casts number to u16                 | `u16`    |
| `u8() `   | Casts number to u8                  | `u8`     |
| `i64()`   | Casts number to i64                 | `i64`    |
| `i32()`   | Casts number to i32                 | `i32`    |
| `i16()`   | Casts number to i16                 | `i16`    |
| `i8()`    | Casts number to i8                  | `i8`    |

## Author

Aaron Gill-Braun aarongillbraun@gmail.com

This project was inspired by [radix-calc](https://github.com/goodell/radix-calc).

## License

MIT License, see the LICENSE file.

The icons used in the Alfred workflow were taken from radix-calc. The main 
calculator icon was originally taken from [Aiconica](https://aiconica.net/detail/calculator-1000),
licensed CC0 1.0. The dec/hex/oct/bin icons are orginal works by the author of
[radix-calc](https://github.com/goodell/radix-calc) and are also licensed under CC0 1.0.


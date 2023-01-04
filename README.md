# Command Line Calculator

This is a simple yet powerful calculator built with programmers in mind. It 
supports all standard arithmetic, bitwise and logical operators and a number 
of built-in constants and functions. It also features a loose type system which
allows calculations to be performed on floats or fixed-width integers. It also 
supports a set of standard units and conversion between them.

While this can be used as a simple shell calculator, it is mainly written to be 
used with the popular macOS productivity tool [Alfred](https://www.alfredapp.com/). 
As such, support for Alfred has been built-in to the calculator itself, and the 
provided Makefile can be used to build the Alfred workflow.

## Options

By default, expressions are read from stdin but you may also supply the 
them in a file with the `-f` option, or on the command line using the `-e`
option.

```
USAGE:
    clc [OPTIONS]

OPTIONS:
    -f, --file <FILE>  Read expression from file
    -e, --expr <EXPR>  Expression to evaluate
        --alfred       Enables alfred JSON output
    -h, --help         Print help information
    -V, --version      Print version information
```

With the `--alfred` option, the calculator will output Alfred JSON Script Filter
items. For floating point results, it will output a just the value, but for integer 
results, it will output items for the decimal, binary, octal and hexadecimal forms. 
In the case of results with a unit, it will output items for all common conversions 
for the result.

## Usage

The calculator supports standard expressions that include numbers, binary and
unary operators, as well as built-in functions and constants. It also accepts
units specified in the form of `<number><unit>`.

The following number formats are supported:
- `1.234` - decimal (type: `f64`)
- `1234` - integer (type: `u64`)
- `0b1010` - binary (type: `u64`)
- `0o1234` - octal (type: `u64`)
- `0x1234` - hexadecimal (type: `u64`)

As an expression is being evaluated, values are implicitly cast and unit conversion 
is performed when necessary. For binary operators, the right-hand side is always cast 
to the type of the left-hand side before the operation is performed. For some functions, 
the parameter is cast to the expected type before the function is called.

### Types and Units

The following table describes the types supported by the calculator. Each name is a
built-in function that can be used to cast to the specified type. When used, the unit
of the number is lost.

| **Name**  | **Description**     |
|-----------|---------------------|
| `u64()`   | Casts number to u64 |
| `u32()`   | Casts number to u32 |
| `u16()`   | Casts number to u16 |
| `u8()`    | Casts number to u8  |
| `i64()`   | Casts number to i64 |
| `i32()`   | Casts number to i32 |
| `i16()`   | Casts number to i16 |
| `i8()`    | Casts number to i8  |
| `f64()`   | Casts number to f64 |

The following table describes the units supported by the calculator. They can be used
in expressions like literals `<number><suffix>` or as a function call to convert to
the specified unit `<name>(<number>)`.

| **Name**       | **Suffix** | **Type** |
|----------------|------------|----------|
| `bytes()`      | `B`        | `u64`    |
| `kilobyte()`   | `K`        | `u64`    |
| `megabyte()`   | `M`        | `u64`    |
| `gigabyte()`   | `G`        | `u64`    |
| `terabyte()`   | `T`        | `u64`    |
| `petabyte()`   | `P`        | `u64`    |
| **Name**       | **Suffix** | **Type** |
| `celsius()`    | `°`, `°C`  | `f64`    |
| `fahrenheit()` | `°F`       | `f64`    |
| `kelvin()`     | `°K`       | `f64`    |

For example:
```
celsius(32.0°F) // converts 32.0 fahrenheit to degrees celsius
fahrenheit(100) // casts 100 to f64 and specifies it is in fahrenheit

kilobyte(1°C)   // not allowed - units not of the same type
```

### Built-in Constants

| **Name**     | **Description**          | **Type** |
|--------------|--------------------------|----------|
| `PI`         | Archimedes' constant (π) | `f64`    |
| `E`          | Euler's number (ℇ)       | `f64`    |
| `NAN`        | Not a number (NaN)       | `f64`    |
| `INF`        | Infinity (∞)             | `f64`    |
| `NEG_INF`    | Negative infinity (-∞)   | `f64`    |
| `<TYPE>_MIN` | Minimum value of type    | `type`   |
| `<TYPE>_MAX` | Maximum value of type    | `type`   |

View the [source](https://github.com/aar10n/clc/blob/master/src/functions.rs#L113) for the full list.

### Built-in Functions

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
| `rad()`   | Converts radians to degrees         | `f64`    |

View the [source](https://github.com/aar10n/clc/blob/master/src/functions.rs#L140) for the full list
of operators, functions, conversions and aliases.

## Author

Aaron Gill-Braun aarongillbraun@gmail.com

This project was inspired by [radix-calc](https://github.com/goodell/radix-calc).

## License

MIT License, see the LICENSE file.

The main calculator icon was originally taken from [Aiconica](https://aiconica.net/detail/calculator-1000),
licensed CC0 1.0.


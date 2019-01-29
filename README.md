# Simple Configuration Language (SCL)

## Objectives
To be a language used for any kind of configuration
file without any of the ambiguities some languages like
YAML or TOML have.

It is *only* meant for configurations and doesn't allow any logic in it.

To ensure ease of use, SCL allows including other files. This makes composing
and inheriting (such as the classic production settings inheriting a base setting file
and changing the secrets) trivial at the configuration file level.

## Example

```nginx
# This is a SCL document. Boom.
include "base.scl"

owner = {
  name = "Vincent Prouillet",
  bio = "Someone",
  dob = 1979-05-27, # first-class date type
}

database = {
  server = "192.168.1.1",
  ports = [ 8001, 8001, 8002 ],
  connection_max = 5000,
  enabled = true,
}

servers = {
  max_upload_size = 10Mb, # first class byte size
  alpha = {
    ip = "10.0.0.1",
    dc = "eqdc10",
  },
  # Inline objects as well
  beta = { ip = "10.0.0.2", dc = "eqdc10" },
}

clients = {
  data = [
    ["gamma", "delta"],
    [1, 2],
  ],
  hosts = ["alpha", "omega"],
}

```

## Spec

- SCL is case sensitive.
- A SCL file must be a valid UTF-8 encoded Unicode document.
- Whitespace means tab (0x09) or space (0x20).
- Newline means LF (0x0A) or CRLF (0x0D0A).

## Comments
A hash symbol marks the rest of the line as a comment.

```
key = "value" # This is a comment at the end of a line
```

## Key/Value pair
Keys are on the left of the equals sign and values are on the right.
Whitespace is ignored around key names and values. The key, equals sign, and value must be on the same line.

```
key = "value"
```

Keys may only contain ASCII letters, ASCII digits, underscores, and dashes (`A-Za-z0-9_-`).
Values must be of the following types: String, Integer, Float, Boolean, Date, Array, or Dictionary.
Unspecified values are invalid.

## Booleans
Booleans are just the tokens you're used to. Always lowercase.

```
bool1 = true
bool2 = false
```

## String
All strings must contain only valid UTF-8 characters.

There are 2 kinds of strings: basic and multi-line.

### Basic
They are surrounded by double quotes:

```
str = "The dog says \"woof\"."
```
As you can see, double quotes have to be escaped if used in basic strings. Valid escape secuences are quotes with `\"`, newlines with `\n` and two digit hexadecimal characters with `\x00`.

### Multi-line
They are surrounded by three double quotes on each side.

```
str = """
This a string where " is allowed without having to escape.
New lines are easy as well.
C:\Users\nodejs\templates will work and <\i\c*\s*> as well
"""
```
A newline immediately following the opening delimiter will be left trimmed. All other whitespace and newline characters remain intact.

## Integer
Integers are whole numbers. Positive numbers may be prefixed with a plus sign. Negative numbers are prefixed with a minus sign.

```
int2 = 42
int3 = 0
int4 = -17
```
For large numbers, you may use underscores between digits to enhance readability.
Each underscore must be preceded by at least one digit and followed by 3 digits.

```
int1 = 1_000_010
int1 = 1_000
int2 = 1_10 # INVALID
int2 = _10 # INVALID
```

64 bit (signed long) range expected (−9,223,372,036,854,775,808 to 9,223,372,036,854,775,807).

## Float

Floats should be implemented as IEEE 754 binary64 values.
A float consists of an integer part (which follows the same rules as integer values) followed by a fractional part and/or an exponent part.
If both a fractional part and exponent part are present, the fractional part must precede the exponent part.

```
flt1 = 1.0
flt2 = 3.1415
flt3 = -0.01

```

A fractional part is a decimal point followed by one or more digits.


Similar to integers, you may use underscores on the integer part to enhance readability.
Each underscore must be preceded by at least one digit and followed by 3 digits.

```
flt4 = 9_224_617.445
```

### Byte size
SCL has first-class support for filesize in powers of ten, following [SI](https://en.wikipedia.org/wiki/Kilobyte).

You can append the following strings to a positive integer or a positive float and SCL will convert it in bytes:

- kB and KB
- MB
- GB
- TB
- PB

## Dates

You may use the date part of
[RFC 3339](http://tools.ietf.org/html/rfc3339) to represent a date.

```toml
date = 1979-05-27
```

## Array

Arrays are square brackets with values inside. Whitespace is ignored.
Elements are separated by commas. Data types may not be mixed.

```toml
arr1 = [ 1, 2, 3 ]
arr2 = [ "red", "yellow", "green" ]
arr3 = [ [ 1, 2 ], [3, 4, 5] ]
arr5 = [ [ 1, 2 ], ["a", "b", "c"] ]

arr6 = [ 1, 2.0 ] # INVALID
```

Arrays can also be multiline. A single terminating comma (also called trailing commas)
is allowed after the last value of the array. There can be an arbitrary number of
newlines and comments before a value and before the closing bracket.

```toml
arr7 = [
  1, 2, 3
]

arr8 = [
  1,
  2, # this is ok
]
```

## Dictionaries
Dictionaries (also called hash tables, hashmaps) are collections of key/value pairs.
They appear in curly brackets in the value position of a key/value pair:

```toml
my_dict = {
    # the elements
}
```

You can nest any type of key/value pairs inside.
Dict elements are separated by a comma and a trailing comma is allowed.

```toml
my_dict = {
    admin = true,
    ratings = [ 1, 2, 3],
    a_nested_dict = {
        hello = "world",
    }
}
```

Dictionaries can also be written inline:

```toml
my_dict = { admin = true, ratings = [ 1, 2, 3], } # trailing comma still allowed
```

## Includes
A SCL file can include another SCL file in two ways:

- without a key
- with a key

Includes can only happen at the root level or in a dictionary and the paths
are relative to each other.

```toml
# without a key (local include)
include "ssl.scl"

# with a key
ssl = include "ssl.scl"

# in a dict
site = {
    # in a dict without a key
    include "something.scl"
    # in a dict with a key
    ssl = include "ssl.scl"
}

# INVALID
ips = [include "ssl.scl"]
```

If there is no key, the data from the included file will be directly in the current level: the root or the dictionary the `include` is in.

Includes can be used to simulate inheritance: place the `include` at the top and you can then override some specific values below.
Order matters however: any key/value set before an `include` also present in the included file will be overriden. It is not
possible to override a particular key from a dictionary.

A few examples:

```toml
# secrets.scl
stripe = ${STRIPE_SECRET}
mail = 1GB

# base.scl
logging = include "logs.scl"
logging_enabled = false

```

## Environment variables
SCL has first-class support for environment variables:

```toml
site_url = ${SITE_URL}
```

SCL should only look for a variable named `SITE_URL`: BASH usage is not allowed.

If the environment variable is not found, an error should be returned.

It is possible to define a default value in case the environment variable is not found to avoid an error:

```toml
site_url = ${SITE_URL || "some val"}
```

The default value types allowed are: boolean, string, integer, float and date.

As every value that comes from the environment is a string, you might want to cast the value to a different type:

```toml
site_url = ${DB_PORT as integer || 5000}
debug = ${DB_PORT as bool || false}
```

The cast types allowed are: `bool`, `integer`, `float` and `date`.
If a cast is done and there is a default value, the types of those need to match.

```toml
site_url = ${DB_PORT as integer || false}  # ERROR
```

## Filename extension
SCL files should use the extension `.scl`.

## Comparison with other formats

SCL has first-class support for file inclusion and environment variable, making
it unique compared to the classic formats used for configuration.

### Comparison with TOML

SCL is inspired by TOML on some aspects.
The main advantages of SCL versus TOML are:

- nested tables/dict can be multi-lines
- byte size type
- include statement to allow composition from several files
- environment variable support built-in

SCL allows aims to be easier to understand than TOML, especially with regards to tables/dict
and array of tables.
Consider the following TOML:


```toml
title = "TOML Example"

[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00

# How do I write a variable at the same level as `title` from here?
```

SCL has removed some features of TOML however:

- only date support, no datetime
- no exponents for number

### Comparison with YAML
YAML is more user friendly than either SCL or TOML for very short files but breaks down
after a small-ish amount of lines. YAML is also very hard to parse safely.

YAML doesn't support `include` statements or environment variables.

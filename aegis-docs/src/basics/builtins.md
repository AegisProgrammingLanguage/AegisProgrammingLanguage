# Built-in Functions

Aegis comes with a set of utility functions that are available globally in any scope. You do not need to import anything to use them.

## Type Conversion

### String
**Syntax**: `to_str(value)` 

Converts any value into its string representation.

```aegis
var x = 10
print "Value: " + to_str(x)
```

### Integer 
**Syntax**: `to_int(value)` 

Converts a value to an integer.
- Floats are truncated (not rounded).
- Strings are parsed.

```aegis
print to_int("42")   // 42
print to_int(3.99)   // 3
```

### Float 
**Syntax**: `to_float(value)` 

Converts a value to a floating-point number.

```aegis
print to_float("3.14") // 3.14
print to_float(10)     // 10.0
```

## Type Inspection

### Type of
**Syntax**: `typeof(value)` 

Returns a string representing the type of the value. Possible return values:
- `int`, `float`, `string`, `bool`, `null`
- `list`, `dict`, `range`, `enum`
- `function`, `class`

For class instances, it returns the name of the Class (e.g., `"User"`).

```aegis
print typeof(10)       // "int"
print typeof([1, 2])   // "list"

class User {}
var u = new User()
print typeof(u)        // "User"
```

### Is Instance
**Syntax**: `is_instance(object, class)` 

Checks if an object is an instance of a specific class or inherits from it. This is the preferred way to check types for objects.

```aegis
class Animal {}
class Dog extends Animal {}

var d = new Dog()
print is_instance(d, Dog)    // true
print is_instance(d, Animal) // true
```

## Utilities

### Len
**Syntax**: `len(iterable)`

Returns the length of a container. Supports:

- String: Number of characters (UTF-8 aware).
- List: Number of elements.
- Dict: Number of key-value pairs.
- Range: Number of steps.

```aegis
print len("Hello")   // 5
print len([1, 2, 3]) // 3
```

# Fmt
**Syntax**: `fmt(value, format_string)`

Formats a number (usually a float) into a string with specific precision.
- Format: ".Nf" where N is the number of decimal places.

```aegis
var pi = 3.14159265
print fmt(pi, ".2f") // "3.14"
print fmt(pi, ".4f") // "3.1416"
```

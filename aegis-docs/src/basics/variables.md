# Variables &amp; Data Types

Aegis allows you to store data using mutable variables or immutable constants.

It is a **dynamically typed** language by default. This means you do not need to specify the type of a variable when you declare it, and a variable can hold values of different types throughout its life (unless you use Gradual Typing).

## Mutable Variable

Use the `var` keyword when the value needs to change during the execution of the program.

```aegis
var username = "Admin"
var score = 100
var is_active = true
```

## Reassignment

You can change the value of a variable at any time:

```aegis
var data = 42
print data // 42

data = "Changed to string"
print data // Changed to string
```

## Immutable Constants (const)

Use `const` for values that must remain the same throughout the entire scope (e.g., configuration, math constants). This provides safety and clarifies intent.

```aegis
const PI = 3.14159
const MAX_RETRIES = 5

// Using the value works fine
var area = PI * 10 * 10
```

### Safety Check

Constants are protected at **Compile Time**. If you try to reassign a constant, Aegis will raise an error and refuse to run the script.

```aegis
const URL = "https://google.com"

// This causes a panic/crash at compile time:
URL = "https://bing.com" 
```

*Tip: It is considered good practice to name constants using UPPER_SNAKE_CASE (e.g., MAX_SPEED), although Aegis does not enforce it.*

## Primitive Types

Aegis supports the following primitive data types:

| Type | Description | Example |
|--- |--- |--- |
| Null | Represents the absence of value. | `null` |
| Boolean | Logical true or false. | `true`, `false` |
| Integer | 64-bit signed integer. | `42`, `-10`, `0` |
| Float | 64-bit floating point number. | `3.14`, `-0.01` |
| String | UTF-8 text sequence. | `"Hello World"` |

*Note: Lists and Dictionaries are complex types and are covered in the Data Structures section.*

## String Interpolation

You can inject variables directly into strings using the ${} syntax. This converts the value to a string automatically.

```aegis
var name = "Aegis"
var version = 0.2

print "Welcome to ${name} version ${version}!"
// Output: Welcome to Aegis version 0.2!
```
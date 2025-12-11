# Enums

Enums (Enumerations) allows you to define a set of named constants. They are useful for representing states, options, or error codes without using "magic numbers".

## Defining an Enum

Use the `enum` keyword followed by a name and a list of variants.

```aegis
enum Status {
    Idle,
    Running,
    Error
}
```

Under the hood, Aegis assigns an auto-incrementing integer to each variant, starting at 0.
- `Status.Idle` is `0`
- `Status.Running` is `1`
- `Status.Error` is `2`

## Usage

You access enum members using the dot notation.

```aegis
var current_state = Status.Running

if (current_state == Status.Error) {
    print "Something went wrong!"
} else {
    print "All systems go."
}
```

## Immutability & Safety

Unlike Dictionaries, Enums are read-only. You cannot add, remove, or modify variants at runtime. This ensures that your constants remain constant throughout the program's execution.

```aegis
try {
    // This will throw a Runtime Error
    Status.Idle = 100 
} catch (e) {
    print "Error: " + e
}
```

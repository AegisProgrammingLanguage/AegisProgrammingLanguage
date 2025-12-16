# Dictionaries

Dictionaries (or Maps) are collections of key-value pairs. They are useful for storing structured data or representing objects.

## Creation

Use curly braces `{}`. Keys must be strings (or identifiers which are converted to strings).

```aegis
var config = {
    host: "localhost",
    port: 8080,
    debug: true
}
```

## Operations

| Method | Description | Example |
|--- |--- |--- |
| `.get(key)` | Returns the value associated with the key. | `dict.get("host")` |
| `.contains(key)` | Returns `true` if the key exists in the dictionary. | `dict.contains("Content-Type")` |
| `.insert(key, val)` | Adds or updates a key-value pair. | `dict.insert("ssl", true)` |
| `.keys()` | Returns a List of all keys in the dictionary. | `dict.keys()` |
| `.len()` | Returns the number of entries. | `dict.len()` |
| `.is_empty()` | Returns true if the dict is empty, otherwise returns false. | `if (dict.is_empty()) { ... }` |
| `.remove(key)` | Removes a key and returns its value. | `dict.remove("ssl")` |
| `.values()` | Returns a List of all values in the dictionary. | `dict.values() // [true]` (true is the value of key "ssl") |

### Example

```aegis
var user = {}

// Adding data
user.insert("name", "Arthur")
user.insert("level", 5)

// Retrieving data
print "User: " + user.get("name")

// Listing keys
var fields = user.keys()
print fields // ["name", "level"]
```

*Note: Accessing a non-existent key with .get() returns null.*

# Reflection & Introspection

Reflection allows an instance to inspect and modify its own structure at runtime. This is a powerful feature used for building ORMs, serializers, and dependency injection systems.

## Methods

Every instance in Aegis has built-in native methods for reflection:

| Method | Description | Example |
| :--- | :--- | :--- |
| `.get_properties()` | Returns a list of all public property names. | `user.get_properties()` |
| `.get_property(name)` | returns the value of the property by its name (string). | `user.get_property("age")` |
| `.set_property(name, val)` | Sets the value of a property dynamically. | `user.set_property("age", 25)` |

## Example: Dynamic Hydration

A common use case is filling an object from a dictionary (e.g., JSON or Database result).

```aegis
class User {
    public username = ""
    public email = ""
    public age = 0
    
    // Fill the object with dictionary data
    public fill(data: dict) {
        var props = this.get_properties()
        
        foreach (key in data.keys()) {
            // Only set if the property exists in the class
            if (props.contains(key)) {
                this.set_property(key, data.get(key))
            }
        }
    }
}

var data = { "username": "Ikigami", "age": 22, "admin": true }
var u = new User()

u.fill(data) 

print u.username // "Ikigami"
print u.age      // 22
// "admin" was ignored safely
```

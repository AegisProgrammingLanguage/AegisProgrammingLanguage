# Classes &amp; Instances

Object-Oriented Programming allows you to structure your code by bundling data (attributes) and behavior (methods) into reusable blueprints called **Classes**.

## Defining a Class

Use the `class` keyword followed by the class name and a block `{ ... }`.

Unlike the previous version, Aegis v0.2.1 uses an explicit **Initializer method** named `init`. This method is automatically called when you create a new instance.

```aegis
class User {
    // The initializer (Constructor)
    init(name, email) {
        // Assign parameters to the instance fields using 'this'
        this.name = name
        this.email = email
        this.active = true
        
        print "Creating user: " + this.name
    }
    
    // Other methods...
}
```

## Creating Instances

To create an object (an instance of a class), use the `new` keyword. You pass the arguments expected by your `init` method here.

```aegis
var admin = new User("Alice", "alice@example.com")
var guest = new User("Bob", "bob@example.com")
```

## Fields (Attributes)

Aegis objects are dynamic. You typically define your fields inside `init`, but you can access, modify, or add new fields at any time using the dot notation.

```aegis
print admin.name // "Alice"

// Modifying a field
admin.name = "SuperAlice"

// Adding a new field dynamically
admin.role = "Administrator"
print admin.role // "Administrator"
```

## Checking Types and Inheritance

To verify if an object is an instance of a specific class, avoid comparing type names with strings. Instead, use the robust global function `is_instance()`.

### `is_instance(object, class)`

Returns `true` if the object is an instance of the class **or** if it inherits from it.

### Example

```aegis
class Animal {}
class Dog extends Animal {}

var dog = new Dog()

// Direct class check
print is_instance(dog, Dog)    // true

// Inheritance check (Polymorphism)
print is_instance(dog, Animal) // true

// Unrelated check
print is_instance(dog, String) // false
```

### Why use is_instance?

Unlike checking `type_of(obj) == "Dog"`, `is_instance` checks the actual structure in memory.
- **Supports Inheritance**: It returns true `for` parent classes.
- **Safety**: It works even if variables are renamed or aliases are used.

```aegis
var Pet = Dog
var my_pet = new Pet()

print is_instance(my_pet, Dog) // true
```

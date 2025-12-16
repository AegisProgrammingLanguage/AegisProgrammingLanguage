# Methods & this

Methods are functions defined inside a class. They define what an object can *do*.

## Defining Methods

Methods can have visibility modifiers (`public`, `private`, `protected`). If omitted, they are `public` by default.

```aegis
class Rectangle {
    // Public fields
    public width = 0
    public height = 0
    
    init(width, height) {
        this.width = width
        this.height = height
    }
    
    // Public method
    func area() {
        return this.width * this.height
    }
    
    // Private method (internal helper)
    private check_validity() {
        if (this.width < 0) throw "Invalid width"
    }
}
```

## The this Keyword

Inside a method, the special variable `this` refers to the current instance. It allows you to access or modify the object's fields and call other methods.

```aegis
var rect = new Rectangle(10, 20)

print "Area: " + rect.area() // 200
```

## Static Method Fallback

Aegis allows calling **Static Methods** from an **Instance**. 

If you try to call a method on an object (e.g., `user.table()`), and that method does not exist on the instance, the VM will look for a **static method** with the same name on the class.

When this happens, `this` inside the static method will refer to the **Class** itself, not the instance.

### Why is this useful?

This enables clean patterns where an instance needs to access configuration defined at the class level (like a table name in an ORM).

```aegis
class Model {
    public static table() {
        return "generic_table"
    }

    public save() {
        // 'this' is the instance here.
        // It calls .table(), which is not on the instance.
        // The VM finds static table() on the class and executes it.
        print "Saving to " + this.table() 
    }
}

class User extends Model {
    public static table() {
        return "users"
    }
}

var u = new User()
u.save() // Prints: "Saving to users"
```

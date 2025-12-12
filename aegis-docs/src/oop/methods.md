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

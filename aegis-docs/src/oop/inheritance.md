# Inheritance

Inheritance allows a class (the **Child**) to derive behavior and properties from another class (the **Parent**).

In Aegis, inheritance is achieved using the `extends` keyword.

## Basic Inheritance

Child classes inherit `public` and `protected` members from the parent. `private` members remain inaccessible to the child.

```aegis
// The Parent Class
class Animal {
    protected name // 'protected' allows children to access this field

    init(name) {
        this.name = name
    }

    public speak() {
        print this.name + " makes a noise."
    }
}

// The Child Class
class Dog extends Animal {
    public fetch() {
        // Can access 'this.name' because it is protected in Animal
        print this.name + " runs after the ball!"
    }
}

var d = new Dog("Rex")
d.fetch() // "Rex runs after the ball!"
d.speak() // "Rex makes a noise." (Inherited)
```

## Method Overriding

A child class can provide its own implementation of a method.

```aegis
class Cat extends Animal {
    // Overriding the 'speak' method
    speak() {
        print "Meow!"
    }
}

var c = new Cat("Luna")
c.speak() // "Meow!"
```

## Accessing Parent Methods

Use `super` to call a method from the parent class. This is essential in the `init` method to ensure the parent is correctly initialized.

```aegis
class Hero extends Entity {
    init(name, hp) {
        // 1. Call the parent constructor first
        super.init(name)
        
        // 2. Add child-specific logic
        this.hp = hp
    }
    
    speak() {
        return "Hero says: " + super.speak()
    }
}
```

## Polymorphism & Type Checking

`is_instance` checks the entire inheritance chain.

```aegis
var d = new Dog("Buddy")

print is_instance(d, Dog)    // true
print is_instance(d, Animal) // true (Dog is an Animal)
print is_instance(d, String) // false
```

## Final Classes & Methods

You can restrict inheritance and overriding using the `final` keyword. This is useful for security, creating immutable utilities, or enforcing a specific design.

### Final Classes

A `final` class **cannot be extended**. This effectively seals the class hierarchy at that point.

```aegis
final class Virus {
    init() {}
}

// ❌ Error: Class 'Corona' cannot inherit from 'Virus' because it is marked 'final'.
// class Corona extends Virus {}
```

### Final Methods

A `final` method cannot be overridden by subclasses. This ensures that the behavior defined in the parent class remains consistent across all children.

```aegis
class God {
    final func create_universe() {
        print "Big Bang!"
    }
}

class Human extends God {
    // ❌ Error: Cannot override final method 'create_universe' of class 'God'.
    /* func create_universe() {
        print "I made this."
    }
    */
}

var h = new Human()
h.create_universe() // "Big Bang!"
```

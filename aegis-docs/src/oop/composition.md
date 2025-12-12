# Composition Pattern

Aegis promotes flexible design patterns. While inheritance creates rigid hierarchies ("is-a" relationship), **Composition** allows you to build complex objects by combining simpler ones ("has-a" relationship).

## Example: Game Entity

Instead of a deep inheritance tree, you can compose an `Player` using a `Vector` and a `Stats` object.

```aegis
// Component 1: Position
class Vector {
    public x = 0
    public y = 0

    init(x, y) {
        this.x = x
        this.y = y
    }

    func str() { return "(" + this.x + ", " + this.y + ")" }
}

// Component 2: Stats
class Stats {
    public hp = 100
    public mana = 50

    init(hp, mana) {
        this.hp = hp
        this.mana = mana
    }
}

// Main Entity using Composition
class Player {
    public name
    public pos
    public stats

    init(name, x, y) {
        this.name = name
        
        // We initialize components inside the constructor
        this.pos = new Vector(x, y)
        this.stats = new Stats(100, 50)
    }
    
    func info() {
        print this.name + " is at " + this.pos.str()
    }
}
```

Usage:

```aegis
var p = new Player("Hero", 10, 10)
p.info() // Hero is at (10, 10)
```

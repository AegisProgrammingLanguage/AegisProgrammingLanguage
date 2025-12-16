# Building a Simple ORM

Using **Reflection** and **Static Fallback**, we can build an "Active Record" style ORM where models map directly to database tables.

## The Base Model

```aegis
import "packages/sqlite/sqlite.aeg"

class Model {
    public id = null

    // 1. Configure table name statically
    public static table() { 
        throw "Table not defined" 
    }

    // 2. Hydrate object dynamically
    public init(data) {
        if (data != null) {
            var props = this.get_properties()
            foreach (k in data.keys()) {
                if (props.contains(k)) {
                    this.set_property(k, data.get(k))
                }
            }
        }
    }

    // 3. Save to Database
    public save() {
        var props = this.get_properties()
        var cols = []
        var vals = []
        var placeholders = []

        foreach (p in props) {
            if (p != "id") {
                cols.push(p)
                vals.push(this.get_property(p))
                placeholders.push("?")
            }
        }

        // Access static table() from instance
        var query = "INSERT INTO ${this.table()} (" + cols.join(",") + ") VALUES (" + placeholders.join(",") + ")"
        Sqlite.execute(DB_CONN, query, vals)
    }
}
```

### Usage

```aegis
class User extends Model {
    public username = ""
    public email = ""

    public static table() { return "users" }
}

// Create and Save in one go!
var u = new User({
    "username": "admin", 
    "email": "admin@aegis.org"
})

u.save()
```

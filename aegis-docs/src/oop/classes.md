# Classes &amp; Instances

Object-Oriented Programming in Aegis allows you to structure your code by bundling data and behavior into reusable blueprints called **Classes**.

## Defining a Class

Use the `class` keyword followed by the class name and a block `{ ... }`.

Aegis now supports **Declared Fields** and **Visibility Modifiers**. You define your data structure at the top of the class.

```aegis
class User {
    // 1. Field Declaration
    public name
    private email
    protected role = "Guest" // Default value
    
    // 2. Initializer (Constructor)
    init(name, email) {
        this.name = name
        this.email = email
        print "User created: " + this.name
    }
    
    // 3. Methods
    public get_info() {
        return this.name + " (" + this.role + ")"
    }
}
```

## Visibility & Encapsulation

Aegis enforces strict encapsulation. There are three visibility modifiers available for both fields and methods:


| Keyword | Scope | Description |
|--- |--- |--- |
| public | Global | Accessible from anywhere. This is the default if no modifier is used. |
| protected | Hierarchy | Accessible only within the class and its subclasses. |
| private | Internal | Accessible only within the class itself. Strictly internal. |

### Example

```aegis
class BankAccount {
    private balance = 0
    public currency = "EUR"

    init(start_amount) {
        if (start_amount > 0) {
            this.balance = start_amount
        }
    }

    public deposit(amount) {
        this.balance += amount
        this.log_transaction()
    }

    // Private method: Internal use only
    private log_transaction() {
        print "Transaction logged."
    }
}

var acc = new BankAccount(100)

// ✅ Public access allowed
print acc.currency 
acc.deposit(50)

// ❌ Error: Access denied ('balance' is private)
// print acc.balance 

// ❌ Error: Access denied ('log_transaction' is private)
// acc.log_transaction()
```

## Creating Instances

To create an object, use the `new` keyword. This allocates memory, initializes fields with their default values, and then calls the `init` method.

```aegis
var admin = new User("Alice", "alice@aegis.lang")
```

## Type Checking

To verify if an object is an instance of a specific class, use the global function `is_instance()`.

```aegis
if (is_instance(admin, User)) {
    print "This is a user."
}
```

# Lists

Lists are ordered collections of values. In Aegis, lists are dynamic (they can grow or shrink) and can hold values of mixed types.

## Creation

Use square brackets `[]` to create a list.

```aegus =
// Empty list
var empty = []

// Mixed types
var numbers = [1, 2, 3, 4]
var user = ["Admin", 42, true]
```

## Accessing Elements

Lists are 0-indexed. Use the `.at(index)` method to retrieve an element.

```aegis
var fruits = ["Apple", "Banana", "Cherry"]

print fruits.at(0) // Apple
print fruits.at(2) // Cherry

// Accessing an out-of-bounds index returns null
print fruits.at(99) // null
```

## Modifying Lists

You can add and remove elements dynamically.

| Method | Description | Example |
|--- |--- |--- |
| `.push(value)` | Adds an element to the end of the list. | `list.push(5)` |
| `.pop()` | Removes and returns the last element. | `var last = list.pop()` |
| `.len()` | Returns the number of elements. | `list.len()` |
| `.reverse()` | Reverses elements in-place and returns the list itself. | `list.reverse()` |
| `.contains(val)` | Returns true if the value exists in the list. | `if (list.contains("admin")) { ... }` |
| `.index_of(val)` | Returns the index of the first occurrence of val, or -1 if not found. | `var idx = list.index_of("banana")` |
| `.join(sep)` | Joins all elements into a single string using a separator. | `list.join(", ")` |
| `.is_empty()` | Returns true if the list is empty. | `if (list.is_empty()) { ... }` |
| `.first()` | Returns the first element of the list. | `var first = list.first()` |
| `.last()` | Returns the last element of the list. | `var last = list.last()` |
| `.clear()` | Removes all items from the list. | `list.clear()` |
| `.slice(start, end)` | Returns a new sub-list from start to end (exclusive). | `var sub = items.slice(0, 10)` |
| `.sort(fn?)` | Sorts the list in-place. Accepts an optional comparison function func(a,b). | `list.sort(func(a,b) { return a - b })` |
| `.find(fn)` | Returns the first element where the callback returns true, or null. | `var u = users.find(func(u) { return u.id == 1 })` |
| `.map(fn)` | Creates a new list with the results of calling a function on every element. | `var squares = nums.map(func(n) { return n * n })` |
| `.filter(fn)` | Creates a new list with all elements that pass the test implemented by the function. | `var adults = users.filter(func(u) { return u.age >= 18 })` |
| `.reduce(fn, init)` | Reduces the list to a single value using an accumulator. | `var sum = nums.reduce(func(acc, n) { return acc + n }, 0)` |
| `.for_each(fn)` | Executes a provided function once for each array element. | `list.for_each(func(item) { print item })` |

### Example
```
var stack = []

stack.push("First")
stack.push("Second")

print stack.len() // 2

print stack.pop() // "Second"
print stack.len() // 1
```

*Note: Lists in Aegis are passed by reference. If you pass a list to a function and modify it there, the original list is affected.*

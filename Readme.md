Amethyst
===

good code always starts from scratch


here is some of Amethyst code

```amethyst
![img][]
print("Hello world")
```

its nothing that special. But can python do this?!
---
````
return 7 / (1 + 1 + 9 (80));
````
yes probably. but can it do this?!:
```
in fun fib(num: any[int, float]) -> any[int, float] {
    pub int a = 0, b = 1, c = undefined;
    if (num > 0) {
        c = a + b
        a = b
        b = c
        print(c)
        fib(num-1)
    }
}

// example usage of function fib is
var num = int[input("Enter number: ")];
fib(num)
```

partialy yes it can!

bad news is that it is `unable!` at doing things that are advanced like this:

it does'nt even have the structs

```

struct main {
    s: any[int, float, String]
}

var a = any[1, 2.9, ""]
var test = main[s: a]
print(test) // output: main[{a: [1, 2.9, ""]}]
```

it cannot even convert the radix of a number like this for example like this:

```
print(1, [r=b]) // output: 1
print(10, [r=x]) // output: a
print("a", [r=d]) // output: 10
```
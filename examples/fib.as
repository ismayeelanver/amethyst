in fun fib(num: any(int, float)) -> any(int, float) {
    pub int a = 0, b = 1, c = undefined;
    if (num > 0) {
        c = a + b
        a = b
        b = c
        print(c)
        fib(num-1)
    }
}

try {
    var num = float(input("Enter number: "));
} catch ConversionError {
    throw Err("Can't convert the input into a number");
}
fib(num)
# 函数

函数遍布于 Rust 代码中。你已经见过语言中最重要的函数之一：main 函数，它是很多程序的入口点。你也见过 fn 关键字，它用来声明新函数。Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。要声明一个函数，需要使用关键字 `fn`，后面跟上函数名，比如

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
```

其中函数参数的类型不能省略，可以有多个参数，但是最多只能返回一个值， 提前返回使用 `return` 关键字。Rust 编译器会对未使用的函数提出警告， 可以使用属性 `#[allow(dead_code)]` 禁用无效代码检查。

```rs
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

可以使用函数名后跟圆括号来调用我们定义过的任意函数。因为程序中已定义 another_function 函数，所以可以在 main 函数中调用它。注意，源码中 another_function 定义在 main 函数 之后；也可以定义在之前。Rust 不关心函数定义于何处，只要定义了就行。

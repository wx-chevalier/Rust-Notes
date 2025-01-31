# Rust 基本数据类型

在 Rust 中，每一个值都属于某一个数据类型（data type），这告诉 Rust 它被指定为何种数据，以便明确数据处理方式。我们将看到两类数据类型子集：标量（scalar）和复合（compound）。记住，Rust 是 静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型。根据值及其使用方式，编译器通常可以推断出我们想要用的类型。当多种类型均有可能时，必须增加类型注解，像这样：

```rs
let guess: u32 = "42".parse().expect("Not a number!");
```

这里如果不添加类型注解，Rust 会显示如下错误，这说明编译器需要我们提供更多信息，来了解我们想要的类型：

```rs
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^
  |         |
  |         cannot infer type for `_`
  |         consider giving `guess` a type
```

复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

## Copy types

Rust 中的某些类型非常简单。它们称为复制类型。这些简单类型都在堆栈上，并且编译器知道它们的大小。这意味着它们非常容易复制，因此编译器在将其发送到函数时始终会进行复制。因此，您无需担心所有权。这些简单类型包括：integers, floats, booleans (true and false), 以及 char.

```rs
fn main() {
    let my_number = 8;
    prints_number(my_number); // Prints 8. prints_number gets a copy of my_number
    prints_number(my_number); // Prints 8 again.
                              // No problem, because my_number is copy type!
}

fn prints_number(number: i32) { // No return with ->
                             // If number was not copy type, it would take it
                             // and we couldn't use it again
    println!("{}", number);
}
```

# 布尔型

正如其他大部分编程语言一样，Rust 中的布尔类型有两个可能的值：true 和 false。Rust 中的布尔类型使用 bool 表示。例如：

```rs
fn main() {
    let t = true;

    let f: bool = false; // 显式指定类型注解
}
```

使用布尔值的主要场景是条件表达式，例如 if 表达式。

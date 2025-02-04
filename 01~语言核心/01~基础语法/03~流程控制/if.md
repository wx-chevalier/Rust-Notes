# if

根据条件是否为真来决定是否执行某些代码，以及根据条件是否为真来重复运行一段代码是大部分编程语言的基本组成部分。Rust 代码中最常见的用来控制执行流的结构是 if 表达式和循环。if 是分支 (branch) 的一种特殊形式，也可以使用 else 和 else if。与 C 语言不同的是，逻辑条件不需要用小括号括起来，但是条件后面必须跟一个代码块。Rust 中的 if 是一个表达式 (expression)，可以赋给一个变量：

```rs
let x = 5;

let y = if x == 5 { 10 } else { 15 };

fn main() {
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    }
}
```

请注意，我们写的是 `my_number == 7` 而不是 `if(my_number == 7)`，在 Rust 中不需要使用 ()。

```rs
fn main() {
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }
}
```

# 使用 else if 处理多重条件

可以将 else if 表达式与 if 和 else 组合来实现多重条件。例如：

```rs
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
number is divisible by 3
```

当执行这个程序时，它按顺序检查每个 `if` 表达式并执行第一个条件为真的代码块。注意即使 6 可以被 2 整除，也不会输出 `number is divisible by 2`，更不会输出 `else` 块中的 `number is not divisible by 4, 3, or 2`。原因是 Rust 只会执行第一个条件为真的代码块，并且一旦它找到一个以后，甚至都不会检查剩下的条件了。

您可以使用 &&（和）和 ||（或）添加更多条件。

# 在 let 语句中使用 if

此外，Rust 还引入了 if let 和 while let 进行模式匹配；因为 if 是一个表达式，我们可以在 let 语句的右侧使用它：

```rs
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

number 变量将会绑定到表示 if 表达式结果的值上。记住，代码块的值是其最后一个表达式的值，而数字本身就是一个表达式。在这个例子中，整个 if 表达式的值取决于哪个代码块被执行。这意味着 if 的每个分支的可能的返回值都必须是相同类型；if 分支和 else 分支的结果都是 i32 整型。如果它们的类型不匹配，如下面这个例子，则会出现一个错误：

```rs
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}

error[E0308]: if and else have incompatible types
 --> src/main.rs:4:18
  |
4 |       let number = if condition {
  |  __________________^
5 | |         5
6 | |     } else {
7 | |         "six"
8 | |     };
  | |_____^ expected integer, found &str
  |
  = note: expected type `{integer}`
             found type `&str`
```

if 代码块中的表达式返回一个整数，而 else 代码块中的表达式返回一个字符串。这不可行，因为变量必须只有一个类型。Rust 需要在编译时就确切的知道 number 变量的类型，这样它就可以在编译时验证在每处使用的 number 变量的类型是有效的。Rust 并不能够在 number 的类型只能在运行时确定的情况下工作；这样会使编译器变得更复杂而且只能为代码提供更少的保障，因为它不得不记录所有变量的多种可能的类型。

```rs
let number = Some(7);
let mut optional = Some(0);

// If `let` destructures `number` into `Some(i)`, evaluate the block.
if let Some(i) = number {
    println!("Matched {:?}!", i);
} else {
    println!("Didn't match a number!");
}

// While `let` destructures `optional` into `Some(i)`, evaluate the block.
while let Some(i) = optional {
    if i > 9 {
        println!("Greater than 9, quit!");
        optional = None;
    } else {
        println!("`i` is `{:?}`. Try again.", i);
        optional = Some(i + 1);
    }
}

```

if let 获取通过等号分隔的一个模式和一个表达式。它的工作方式与 match 相同，这里的表达式对应 match 而模式则对应第一个分支。使用 if let 意味着编写更少代码，更少的缩进和更少的样板代码。然而，这样会失去 match 强制要求的穷尽性检查。match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。
换句话说，可以认为 `if let` 是 `match` 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。可以在 `if let` 中包含一个 `else`。`else` 块中的代码与 `match` 表达式中的 `_` 分支块中的代码相同，这样的 `match` 表达式就等同于 `if let` 和 `else`。回忆一下示例 6-4 中 `Coin` 枚举的定义，其 `Quarter` 成员也包含一个 `UsState` 值。如果想要计数所有不是 25 美分的硬币的同时也报告 25 美分硬币所属的州，可以使用这样一个 `match` 表达式：

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

或者可以使用这样的 `if let` 和 `else` 表达式：

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

如果你的程序遇到一个使用 `match` 表达起来过于啰嗦的逻辑，记住 `if let` 也在你的 Rust 工具箱中。

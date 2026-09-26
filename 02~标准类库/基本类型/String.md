# Rust 字符串

# String 和 ＆str

Rust 有两种主要的字符串类型：String 和 ＆str。有什么区别？

- &str 是一个简单的字符串。当你写 `let my_variable = "Hello, world!"`, 你创建了 &str，它的速度非常快。
- String 字符串是更复杂的字符串。它有点慢，并且功能更多。字符串是一个指针，在堆上有数据。

&str 和 String 都是 UTF-8，譬如：

```rs
// Rust 的 string 被表达为：&'static str，所有的用""包裹起来的字符串，都被声明成了一个不可变，静态的字符串。
let x = "Hello";
let x:&'static str = "Hello";

fn main() {
    let name = "서태지"; // This is a Korean name. No problem, because a &str is UTF-8.
    let other_name = String::from("Adrian Fahrenheit Țepeș"); // Ț and ș are no problem in UTF-8.
}
```rust
您甚至可以借助 UTF-8 编写表情符号。

```rs
fn main() {
    let name = "😂";
    println!("My name is actually {}", name);
}
```rust
那么为什么我们在 str 前面需要一个 ＆ 而不是 String 呢？因为，str 是动态类型，例如，名字 "서태지" 以及 "Adrian Fahrenheit Țepeș" 堆栈上的大小不同:

```rs
fn main() {
    println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>()); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
    println!("But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.", std::mem::size_of_val("서태지")); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!("And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Adrian Fahrenheit Țepeș"));
}
```rust
这就是为什么我们需要一个 ＆ 的原因，因为 ＆ 会创建一个指针，而 Rust 知道指针的大小。因此，指针进入堆栈。如果我们写了 str，Rust 将不知道要做什么，因为它不知道大小。有很多方法可以制作字符串。

## 类型互转

这里有一些：

- `String::from("This is the string text");` 这是用于 String 的方法，该方法采用文本并创建 String。
- `"This is the string text".to_string()`，这是 ＆str 的方法，使它成为 String。
- format! 宏。这就像 println！除了它创建一个 String 而不是打印。因此，您可以执行以下操作：

```rs
fn main() {
    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";

    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );
}
```rust
现在我们有了一个在一起命名的字符串，但尚未打印出来。此外，我们还可以使用 `.into()` 来创建字符串，某些类型可以使用 from 和 .into() 轻松地与其他类型进行转换；如果您有 From，那么您也有.into()。from 更清晰，因为您已经知道类型：您知道 `String::from("Some str"）` 是来自 ＆str 的 String。但是使用 .into()，有时编译器不知道：

```rs
fn main() {
    let my_string = "Try to make this a String".into(); // ⚠️
}

// Rust doesn't know what type you want, because many types can be made from a &str.

error[E0282]: type annotations needed
 --> src\main.rs:2:9
  |
2 |     let my_string = "Try to make this a String".into();
  |         ^^^^^^^^^ consider giving `my_string` a type

fn main() {
    let my_string: String = "Try to make this a String".into();
}
```rust
我们也可以使用 `&*` 符号将 String 转化为 &str 类型：

```rs
fn use_str(s: &str) {
    println!("I am: {}", s);
}

fn main() {
    let s = "Hello".to_string();
    use_str(&*s);
}
```rust
首先呢，`&*` 是两个符号 `&` 和 `*` 的组合，按照 Rust 的运算顺序，先对 String 进行 Deref,也就是 `*` 操作。由于 String 实现了 `impl Deref<Target=str> for String`，这相当于一个运算符重载，所以你就能通过 `*` 获得一个 str 类型。但是我们知道，单独的 str 是不能在 Rust 里直接存在的，因此，我们需要先给他进行 & 操作取得 &str 这个结果。

# 索引访问

有人会把 Rust 中的字符串和其惯用的字符串等同起来，于是就出现了如下代码

```rust
let x = "hello".to_string();
x[1]; //编译错误！
```rust
Rust 的字符串实际上是不支持通过下标访问的，但是呢，我们可以通过将其转变成数组的方式访问

```rust
let x = "哎哟我去".to_string();
for i in x.as_bytes() {
    print!("{} ", i);
}

println!("");

for i in x.chars() {
    print!("{}", i);
}

x.chars().nth(2);
```rust
# 格式化字符串

Rust 采取了一种类似 Python 里面 format 的用法，其核心组成是五个宏和两个 trait:`format!`、`format_arg!`、`print!`、`println!`、`write!`;`Debug`、`Display`。相信你们在写 Rust 版本的 Hello World 的时候用到了`print!`或者`println!`这两个宏，但是其实最核心的是`format!`，前两个宏只不过将`format!`的结果输出到了 console 而已。

```rs
fn main() {
    let s = format!("{1}是个有着{0:>0width$}KG重，{height:?}cm高的大胖子",
                    81, "wayslog", width=4, height=178);
    // 我被逼的牺牲了自己了……
    print!("{}", s);
}
```rust
```rs
format_string := <text> [ format <text> ] *
format := '{' [ argument ] [ ':' format_spec ] '}'
argument := integer | identifier

format_spec := [[fill]align][sign]['#'][0][width]['.' precision][type]
fill := character
align := '<' | '^' | '>'
sign := '+' | '-'
width := count
precision := count | '*'
type := identifier | ''
count := parameter | integer
parameter := integer '$'
```rust
# 字符串切片

另一个没有所有权的数据类型是 slice。slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。字符串 slice（string slice）是 String 中一部分值的引用，它看起来像这样：

```rs
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```rust
这类似于引用整个 String 不过带有额外的 [0..5] 部分。它不是对整个 String 的引用，而是对部分 String 的引用。可以使用一个由中括号中的 [starting_index..ending_index] 指定的 range 创建一个 slice，其中 starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值。在其内部，slice 的数据结构存储了 slice 的开始位置和长度，长度对应于 ending_index 减去 starting_index 的值。所以对于 `let world = &s[6..11];` 的情况，world 将是一个包含指向 s 第 7 个字节（从 1 开始）的指针和长度值 5 的 slice。

![引用了部分 String 的字符串 Slice](https://s1.ax1x.com/2020/09/14/wDKG9S.png)

对于 Rust 的 .. range 语法，如果想要从第一个索引（0）开始，可以不写两个点号之前的值。换句话说，如下两个语句是相同的：

```rs
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```rust
依此类推，如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字。这意味着如下也是相同的：

```rs
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```rust
也可以同时舍弃这两个值来获取整个字符串的 slice。所以如下亦是相同的：

```rs
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```rust
字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。

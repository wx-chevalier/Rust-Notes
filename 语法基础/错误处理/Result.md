# Result

Result 类似于 Option，但是区别在于：Option 为 Some 或 None（值或无值），Result 为 Ok 或 Err（好的结果或错误结果）。因此，Option 的语义是：也许会有东西，也许不会。”但是 Result 的语义是：也许它将失败。”

```rs
enum Option<T> {
    None,
    Some(T),
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() { }
```

因此 Result 的值在 Ok 内，而值在 Err 内。这是因为错误通常（并且应该具有）内部具有信息。`Result<T, E>` 意味着您需要考虑要为 Ok 返回的内容以及要为 Err 返回的内容。实际上，您可以决定任何事情。即使这样也可以：

```rs
fn check_error() -> Result<(), ()> {
    Ok(())
}

fn main() {
    check_error();
}
```

check_error 表示“如果确定可以返回 ()，如果得到错误则返回 ()”。然后我们用 () 返回 Ok。编译器给我们一个有趣的警告：

```rs
warning: unused `std::result::Result` that must be used
 --> src\main.rs:6:5
  |
6 |     check_error();
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: this `Result` may be an `Err` variant, which should be handled
```

的确如此：我们只返回了 Result，但可能是 Err。因此，即使我们仍然没有真正做任何事情，让我们稍微处理一下错误。

```rs
fn give_result(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        return Ok(())
    } else {
        return Err(())
    }
}

fn main() {
    if give_result(5).is_ok() {
        println!("It's okay, guys")
    } else {
        println!("It's an error, guys")
    }
}

// It's an error, guys
```

请记住，四种易于检查的方法是 .is_some()，is_none()，is_ok() 和 is_err()。有时带有 Result 的函数将使用 String 作为 Err 值。这不是最好的方法，但是比我们到目前为止做的要好。

```rs
fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_string()), // This is our error message
    }
}

fn main() {
    let mut result_vec = Vec::new(); // Create a new vec for the results

    for number in 2..7 {
        result_vec.push(check_if_five(number)); // push each result into the vec
    }

    println!("{:?}", result_vec);
}

[Err("Sorry, the number wasn\'t five."), Err("Sorry, the number wasn\'t five."), Err("Sorry, the number wasn\'t five."), Ok(5),
Err("Sorry, the number wasn\'t five.")]
```

## Result 别名

在 Rust 的标准库中会经常出现 Result 的别名，用来默认确认其中 Ok(T)或者 Err(E)的类型，这能减少重复编码。比如 io::Result

```rs
use std::num::ParseIntError;
use std::result;

type Result<T> = result::Result<T, ParseIntError>;

fn double_number(number_str: &str) -> Result<i32> {
unimplemented!();
}
```

# 组合 Option 和 Result

`Option` 的方法 `ok_or`：

```rust
fn ok_or<T, E>(option: Option<T>, err: E) -> Result<T, E> {
    match option {
        Some(val) => Ok(val),
        None => Err(err),
    }
}
```

可以在值为`None`的时候返回一个`Result::Err(E)`，值为`Some(T)`的时候返回`Ok(T)`，利用它我们可以组合`Option`和`Result`：

```rust
use std::env;

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

fn main() {
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

`double_arg` 将传入的命令行参数转化为数字并翻倍，`ok_or` 将 `Option` 类型转换成 `Result`，`map_err` 当值为 `Err(E)` 时调用作为参数的函数处理错误

## 复杂的例子

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
         .map_err(|err| err.to_string())
         .and_then(|mut file| {
              let mut contents = String::new();
              file.read_to_string(&mut contents)
                  .map_err(|err| err.to_string())
                  .map(|_| contents)
         })
         .and_then(|contents| {
              contents.trim().parse::<i32>()
                      .map_err(|err| err.to_string())
         })
         .map(|n| 2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

`file_double`从文件中读取内容并将其转化成`i32`类型再翻倍。这个例子看起来已经很复杂了，它使用了多个组合方法，我们可以使用传统的`match`和`if let`来改写它：

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    }
    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

这两种方法个人认为都是可以的，依具体情况来取舍。

## try!

```rust
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(::std::convert::From::from(err)),
    });
}
```

`try!` 事实上就是 `match Result` 的封装，当遇到 `Err(E)` 时会提早返回， `::std::convert::From::from(err)`可以将不同的错误类型返回成最终需要的错误类型，因为所有的错误都能通过 `From` 转化成 `Box`，所以下面的代码是正确的：

```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<Error>> {
    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    let n = try!(contents.trim().parse::<i32>());
    Ok(2 * n)
}
```

## 组合自定义错误类型

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num;
use std::io;
use std::path::Path;

// We derive `Debug` because all types should probably derive `Debug`.
// This gives us a reasonable human readable description of `CliError` values.
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

fn file_double_verbose<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path).map_err(CliError::Io));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(CliError::Io));
    let n: i32 = try!(contents.trim().parse().map_err(CliError::Parse));
    Ok(2 * n)
}
```

`CliError` 分别为 `io::Error` 和 `num::ParseIntError` 实现了 `From` 这个 trait，所有调用 `try!` 的时候这两种错误类型都能转化成 `CliError`。

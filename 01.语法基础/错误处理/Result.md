# Result

大部分错误并没有严重到需要程序完全停止执行。有时，一个函数会因为一个容易理解并做出反应的原因失败。例如，如果因为打开一个并不存在的文件而失败，此时我们可能想要创建这个文件，而不是终止进程。Result 类似于 Option，但是区别在于：Option 为 Some 或 None（值或无值），Result 为 Ok 或 Err（好的结果或错误结果）。因此，Option 的语义是：也许会有东西，也许不会。但是 Result 的语义是：也许它将失败。

```rs
enum Option<T> {
    None,
    Some(T),
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

```

因此 Result 的值在 Ok 内，而值在 Err 内。这是因为错误通常（并且应该具有）内部具有信息。`Result<T, E>` 意味着您需要考虑要为 Ok 返回的内容以及要为 Err 返回的内容。对于 Result 最常见的使用场景就是文件读写：

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}
```

注意与 Option 枚举一样，Result 枚举和其成员也被导入到了 prelude 中，所以就不需要在 match 分支中的 Ok 和 Err 之前指定 Result::。这里我们告诉 Rust 当结果是 Ok 时，返回 Ok 成员中的 file 值，然后将这个文件句柄赋值给变量 f。match 之后，我们可以利用这个文件句柄来进行读写。match 的另一个分支处理从 File::open 得到 Err 值的情况。在这种情况下，我们选择调用 panic! 宏。如果当前目录没有一个叫做 hello.txt 的文件，当运行这段代码时会看到如下来自 panic! 宏的输出：

```rs
thread 'main' panicked at 'Problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
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

可以在值为 `None` 的时候返回一个 `Result::Err(E)`，值为 `Some(T)` 的时候返回 `Ok(T)`，利用它我们可以组合 `Option` 和 `Result`：

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

`try!` 事实上就是 `match Result` 的封装，当遇到 `Err(E)` 时会提早返回，`::std::convert::From::from(err)`可以将不同的错误类型返回成最终需要的错误类型，因为所有的错误都能通过 `From` 转化成 `Box`，所以下面的代码是正确的：

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

# 匹配不同的错误

我们真正希望的是对不同的错误原因采取不同的行为：如果 File::open 因为文件不存在而失败，我们希望创建这个文件并返回新文件的句柄。如果 File::open 因为任何其他原因失败，例如没有打开文件的权限，我们则希望是 panic!。

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

File::open 返回的 Err 成员中的值类型 io::Error，它是一个标准库中提供的结构体。这个结构体有一个返回 io::ErrorKind 值的 kind 方法可供调用。io::ErrorKind 是一个标准库提供的枚举，它的成员对应 io 操作可能导致的不同错误类型。我们感兴趣的成员是 ErrorKind::NotFound，它代表尝试打开的文件并不存在。这样，match 就匹配完 f 了，不过对于 error.kind() 还有一个内层 match。

我们希望在内层 match 中检查的条件是 error.kind() 的返回值是否为 ErrorKind 的 NotFound 成员。如果是，则尝试通过 File::create 创建文件。然而因为 File::create 也可能会失败，还需要增加一个内层 match 语句。当文件不能被打开，会打印出一个不同的错误信息。外层 match 的最后一个分支保持不变，这样对任何除了文件不存在的错误会使程序 panic。

如果与闭包相结合，我们可以写成如下的形式：

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

或者结合 try! 宏我们可以使用如下的写法：

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

# 错误传播

当编写一个其实现会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理。这被称为 传播（propagating）错误，这样能更好的控制代码调用，因为比起你代码所拥有的上下文，调用者可能拥有更多信息或逻辑来决定应该如何处理错误。下例展示了一个从文件中读取用户名的函数。如果文件不存在或不能读取，这个函数会将这些错误返回给调用它的代码：

```rs
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

首先让我们看看函数的返回值：`Result`。这意味着函数返回一个 `Result` 类型的值，其中泛型参数 `T` 的具体类型是 `String`，而 `E` 的具体类型是 `io::Error`。如果这个函数没有出任何错误成功返回，函数的调用者会收到一个包含 `String` 的 `Ok` 值 —— 函数从文件中读取到的用户名。如果函数遇到任何错误，函数的调用者会收到一个 `Err` 值，它储存了一个包含更多这个问题相关信息的 `io::Error` 实例。这里选择 `io::Error` 作为函数的返回值是因为它正好是函数体中那两个可能会失败的操作的错误返回值：`File::open` 函数和 `read_to_string` 方法。函数体以 `File::open` 函数开头。接着使用 `match` 处理返回值 `Result`，类似于示例 9-4 中的 `match`，唯一的区别是当 `Err` 时不再调用 `panic!`，而是提早返回并将 `File::open` 返回的错误值作为函数的错误返回值传递给调用者。如果 `File::open` 成功了，我们将文件句柄储存在变量 `f` 中并继续。

接着我们在变量 `s` 中创建了一个新 `String` 并调用文件句柄 `f` 的 `read_to_string` 方法来将文件的内容读取到 `s` 中。`read_to_string` 方法也返回一个 `Result` 因为它也可能会失败：哪怕是 `File::open` 已经成功了。所以我们需要另一个 `match` 来处理这个 `Result`：如果 `read_to_string` 成功了，那么这个函数就成功了，并返回文件中的用户名，它现在位于被封装进 `Ok` 的 `s` 中。如果`read_to_string` 失败了，则像之前处理 `File::open` 的返回值的 `match` 那样返回错误值。不过并不需要显式的调用 `return`，因为这是函数的最后一个表达式。调用这个函数的代码最终会得到一个包含用户名的 `Ok` 值，或者一个包含 `io::Error` 的 `Err` 值。我们无从得知调用者会如何处理这些值。例如，如果他们得到了一个 `Err` 值，他们可能会选择 `panic!` 并使程序崩溃、使用一个默认的用户名或者从文件之外的地方寻找用户名。我们没有足够的信息知晓调用者具体会如何尝试，所以将所有的成功或失败信息向上传播，让他们选择合适的处理方法。

## 简写 ? 运算符

```rs

use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

Result 值之后的 ? 被定义为与上例中定义的处理 Result 值的 match 表达式有着完全相同的工作方式。如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。? 运算符所使用的错误值被传递给了 from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型。这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败。只要每一个错误类型都实现了 from 函数来定义如何将自身转换为返回的错误类型，? 运算符会自动处理这些转换。

? 运算符消除了大量样板代码并使得函数的实现更简单。我们甚至可以在 ? 之后直接使用链式方法调用来进一步缩短代码，如下例所示：

```rs
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

在 s 中创建新的 String 被放到了函数开头；这一部分没有变化。我们对 File::open("hello.txt")? 的结果直接链式调用了 read_to_string，而不再创建变量 f。仍然需要 read_to_string 调用结尾的 ?，而且当 File::open 和 read_to_string 都成功没有失败时返回包含用户名 s 的 Ok 值。其

# Option

Option 是标准库定义的另一个枚举。Option 类型应用广泛因为它编码了一个非常普遍的场景，即一个值要么有值要么没值。从类型系统的角度来表达这个概念就意味着编译器需要检查是否处理了所有应该处理的情况，这样就可以避免在其他编程语言中非常常见的问题。编程语言的设计经常要考虑包含哪些功能，但考虑排除哪些功能也很重要。Rust 并没有很多其他语言中有的空值功能。空值（Null ）是一个值，它代表没有值。在有空值的语言中，变量总是这两种状态之一：空值和非空值。

空值的问题在于当你尝试像一个非空值那样使用一个空值，会出现某种形式的错误。因为空和非空的属性无处不在，非常容易出现这类错误。然而，空值尝试表达的概念仍然是有意义的：空值是一个因为某种原因目前无效或缺失的值。问题不在于概念而在于具体的实现。为此，Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是 Option<T>，而且它定义于标准库中，如下:

```rs
enum Option<T> {
    None,
    Some(T),
}
```

当您拥有一个可能存在或不存在的值时，请使用 Option。当值存在时为 `Some(value)`，当值不存在时仅为 None，这是一个错误代码示例，可以使用 Option 进行改进。

```rs
    // ⚠️
fn take_fifth(value: Vec<i32>) -> i32 {
    value[4]
}

fn main() {
    let new_vec = vec![1, 2];
    let index = take_fifth(new_vec);
}

// thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 4', src\main.rs:34:5
```

Panic 表示程序在问题发生之前已停止。Rust 看到该函数想要一些不可能的事情，然后停止。它“展开堆栈”（从堆栈中取出值），并告诉您“对不起，我不能这样做”。所以现在我们将返回类型从 i32 更改为`Option<i32>`。这意味着“如果有，请给我 Some（i32），如果没有，请给我任何内容”。我们说 i32 是“包装”在 Option 中的，这意味着它在 Option 内部。您必须做一些事情才能获得价值。

```rs
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 { // .len() gives the length of the vec.
                         // It must be at least 5.
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    println!("{:?}, {:?}", take_fifth(new_vec), take_fifth(bigger_vec));
}

None, Some(5)
```

一个完整的例子如下：

```rust
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}
```

`find`在字符串`haystack`中查找`needle`字符，事实上结果会出现两种可能，有（`Some(usize)`)或无（`None`）。

```rust
fn main() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i+1..]),
    }
}
```

Rust 使用模式匹配来处理返回值，调用者必须处理结果为`None`的情况。这往往是一个好的编程习惯，可以减少潜在的 bug。Option 包含一些方法来简化模式匹配，毕竟过多的`match`会使代码变得臃肿，这也是滋生 bug 的原因之一。

# 属性方法

## unwrap

我们可以使用 `.unwrap()` 获取选项中的值，但使用 `.unwrap()` 时要小心。就像打开礼物一样：也许里面好东西，或者里面有一条愤怒的蛇。如果您确定的话，只想 `.unwrap()` 即可。如果解开的值是 None，则程序将崩溃。

```rs
// ⚠️
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 4 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    println!("{:?}, {:?}",
        take_fifth(new_vec).unwrap(), // this one is None. .unwrap() will panic!
        take_fifth(bigger_vec).unwrap()
    );
}
```

unwrap 的源码如下：

```rust
impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None =>
              panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

`unwrap` 当遇到 `None` 值时会 panic，如前面所说这不是一个好的工程实践。不过有些时候却非常有用：

- **在例子和简单快速的编码中** 有的时候你只是需要一个小例子或者一个简单的小程序，输入输出已经确定，你根本没必要花太多时间考虑错误处理，使用 `unwrap` 变得非常合适。
- **当程序遇到了致命的 bug，panic 是最优选择**

很多时候我们并不需要 unwrap，可以使用 match 来进行值匹配：

```rs
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 4 {
        None
    } else {
        Some(value[4])
    }
}

fn handle_option(my_option: Vec<Option<i32>>) {
  for item in my_option {
    match item {
      Some(number) => println!("Found a {}!", number),
      None => println!("Found a None!"),
    }
  }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let mut option_vec = Vec::new(); // Make a new vec to hold our options
                                     // The vec is type: Vec<Option<i32>>. That means a vec of Option<i32>.

    option_vec.push(take_fifth(new_vec)); // This pushes "None" into the vec
    option_vec.push(take_fifth(bigger_vec)); // This pushes "Some(5)" into the vec

    handle_option(option_vec); // handle_option looks at every option in the vec.
                               // It prints the value if it is Some. It doesn't touch it if it is None.
}

Found a None!
Found a 5!
```

当然，我们也可以直接去判断 Option 的属性，譬如它提供了 .is_some() 方法来判断是否为 Some 类型，包括 .is_none() 来判断是否为 None 类型：

```rs
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 4 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
        let inside_number = take_fifth(vec);
        if inside_number.is_some() {
            // .is_some() returns true if we get Some, false if we get None
            println!("We got: {}", inside_number.unwrap()); // now it is safe to use .unwrap() because we already checked
        } else {
            println!("We got nothing.");
        }
    }
}

We got nothing.
We got: 5
```

## unwrap_or

```rust
fn unwrap_or<T>(option: Option<T>, default: T) -> T {
    match option {
        None => default,
        Some(value) => value,
    }
}
```

`unwrap_or`提供了一个默认值`default`，当值为`None`时返回`default`：

```rust
fn main() {
    assert_eq!(extension("foo.rs").unwrap_or("rs"), "rs");
    assert_eq!(extension("foo").unwrap_or("rs"), "rs");
}
```

## map

假如我们要在一个字符串中找到文件的扩展名，比如 foo.rs 中的 rs，我们可以这样：

```rust
fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..]),
    }
}

fn main() {
    match extension_explicit("foo.rs") {
        None => println!("no extension"),
        Some(ext) =>  assert_eq!(ext, "rs"),
    }
}
```

我们可以使用 `map` 简化：

```rust
// map 是标准库中的方法
fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}

// 使用 map 去掉 match
fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}
```

`map`如果有值`Some(T)`会执行`f`，反之直接返回`None`。

## and_then

```rust
fn and_then<F, T, A>(option: Option<T>, f: F) -> Option<A>
        where F: FnOnce(T) -> Option<A> {
    match option {
        None => None,
        Some(value) => f(value),
    }
}
```

看起来 `and_then` 和 `map` 差不多，不过 `map` 只是把值为 `Some(t)` 重新映射了一遍，`and_then` 则会返回另一个 `Option`。如果我们在一个文件路径中找到它的扩展名，这时候就会变得尤为重要：

```rust
use std::path::Path;
fn file_name(file_path: &str) -> Option<&str> {
    let path = Path::new(file_path);
    path.file_name().to_str()
}
fn file_path_ext(file_path: &str) -> Option<&str> {
    file_name(file_path).and_then(extension)
}
```

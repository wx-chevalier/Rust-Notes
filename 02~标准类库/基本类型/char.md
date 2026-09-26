# char

Rust 的 char 类型是语言中最原生的字母类型，如下代码展示了如何使用它。（注意 char 由单引号指定，不同于字符串使用双引号。）。您看到一个 char 始终是一个字符，并使用 `''` 而不是 `""`。所有字符均为 4 个字节。它们是 4 个字节，因为字符串中的某些字符超过一个字节。一直在计算机上使用的基本字母是 1 个字节，后面的字符是 2 个字节，其他字符是 3 和 4。一个 char 是 4 个字节，因此可以容纳所有这些字符。

```rust
fn main() {
    println!("{}", "a".len()); // .len() gives the size in bytes
    println!("{}", "ß".len());
    println!("{}", "国".len());
    println!("{}", "𓅱".len());
}

1
2
3
4
```
您可以看到 `a` 是一个字节，德语 `ß` 是两个字节，日本 `国` 字是三个字节，古埃及语 `𓅱` 是 4 个字节。

```rust
fn main() {
    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!"; // Korean for "hi"
    println!("Slice2 is {} bytes.", slice2.len());
}
```
slice 的长度为六个字符和六个字节，而 slice2 的长度为三个字符和七个字节。char 需要以任何语言适合任何字符，因此它是 4 个字节长。如果 `.len()` 以字节为单位提供大小，那么以字符为单位的大小呢？稍后我们将学习这些方法，但是您只需记住 `.chars().count()` 即可完成。

```rust
fn main() {
    let slice = "Hello!";
    println!("Slice is {} characters.", slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} characters.", slice2.chars().count());
}

Slice is 6 characters.
Slice2 is 3 characters.
```
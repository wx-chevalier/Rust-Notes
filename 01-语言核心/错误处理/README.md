# Rust 中空与错误处理

错误处理是保证程序健壮性的前提，在编程语言中错误处理的方式大致分为两种：抛出异常（exceptions）和作为值返回。Rust 对可靠性的执着也延伸到了错误处理。错误对于软件来说是不可避免的，所以 Rust 有很多特性来处理出现错误的情况。在很多情况下，Rust 要求你承认出错的可能性，并在编译代码之前就采取行动。这些要求使得程序更为健壮，它们确保了你会在将代码部署到生产环境之前就发现错误并正确地处理它们。

Rust 将错误组合成两个主要类别：可恢复错误（recoverable）和 不可恢复错误（unrecoverable）。可恢复错误通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件。不可恢复错误通常是 bug 的同义词，比如尝试访问超过数组结尾的位置。大部分语言并不区分这两类错误，并采用类似异常这样方式统一处理他们。Rust 并没有异常，但是，有可恢复错误 `Result`，和不可恢复(遇到错误时停止程序执行)错误 `panic!`。panic! 宏代表一个程序无法处理的状态，并停止执行而不是使用无效或不正确的值继续处理。Rust 类型系统的 Result 枚举代表操作可能会在一种可以恢复的情况下失败。可以使用 Result 来告诉代码调用者他需要处理潜在的成功或失败。在适当的场景使用 panic! 和 Result 将会使你的代码在面对不可避免的错误时显得更加可靠。

# Links

- https://nrc.github.io/error-docs/ A guide to error handling in Rust. The first half introduces Rust's language features and libraries for error handling. The second half should help you make good error handling code in your Rust programs.

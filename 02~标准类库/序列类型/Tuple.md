# 元组

元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。Rust 中的元组使用 ()。我们已经看到了许多空元组。`fn do_something() {}` 有一个空的元组。

```rs
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // tup 变量绑定到整个元组上，因为元组是一个单独的复合元素。为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}

// 当您在函数中不返回任何内容时，实际上会返回一个空的元组。
fn just_prints() {
    println!("I am printing"); // Adding ; means we return an empty tuple
}
```

程序首先创建了一个元组并绑定到 tup 变量上。接着使用了 let 和一个模式将 tup 分成了三个不同的变量，x、y 和 z。这叫做 解构（destructuring），因为它将一个元组拆成了三个部分。最后，程序打印出了 y 的值，也就是 6.4。

```rs
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

这个程序创建了一个元组，x，并接着使用索引为每个元素创建新变量。跟大多数编程语言一样，元组的第一个索引值是 0。元组可以容纳很多东西，也可以容纳不同的类型。要访问元组中的项目，请不要使用 `[]`，而应使用 `.`。除了使用模式匹配解构外，也可以使用点号（.）后跟值的索引来直接访问它们。例如：

```rs
fn main() {
    let mut new_vec = Vec::new();
    new_vec.push('a');
    let random_tuple = ("Here is a name", 8, new_vec, 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}
Second item: {:?}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    )
}
```

您可以使用元组创建多个变量。

```rs
fn main() {
    let str_vec = vec!["one", "two", "three"];

    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("{:?}", b);
}

```

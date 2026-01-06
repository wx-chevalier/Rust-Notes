# 常用 Trait

# Copy

Copy 特性定义在标准库 std::marker::Copy 中：

```rs
pub trait Copy: Clone { }
```

一旦一种类型实现了 Copy 特性，这就意味着这种类型可以通过的简单的位(bits)拷贝实现拷贝。从前面知识我们知道“绑定”存在 move 语义（所有权转移），但是，一旦这种类型实现了 Copy 特性，会先拷贝内容到新内存区域，然后把新内存区域和这个标识符做绑定。

哪些情况下我们自定义的类型（如某个 Struct 等）可以实现 Copy 特性？ 只要这种类型的属性类型都实现了 Copy 特性，那么这个类型就可以实现 Copy 特性。例如：

```rs
struct Foo {  //可实现Copy特性
    a: i32,
    b: bool,
}

struct Bar {  //不可实现Copy特性
    l: Vec<i32>,
}
```

因为 Foo 的属性 a 和 b 的类型 i32 和 bool 均实现了 Copy 特性，所以 Foo 也是可以实现 Copy 特性的。但对于 Bar 来说，它的属性 l 是 Vec<T>类型，这种类型并没有实现 Copy 特性，所以 Bar 也是无法实现 Copy 特性的。

有两种方式可以实现 Copy 特性：

- 通过 derive 让 Rust 编译器自动实现

```rs
 #[derive(Copy, Clone)]
 struct Foo {
     a: i32,
     b: bool,
 }
```

编译器会自动检查 Foo 的所有属性是否实现了 Copy 特性，一旦检查通过，便会为 Foo 自动实现 Copy 特性。

- 手动实现 Clone 和 Copy trait

```rs
#[derive(Debug)]
 struct Foo {
     a: i32,
     b: bool,
 }
 impl Copy for Foo {}
 impl Clone for Foo {
     fn clone(&self) -> Foo {
         Foo{a: self.a, b: self.b}
     }
 }
 fn main() {
     let x = Foo{ a: 100, b: true};
     let mut y = x;
     y.b = false;

     println!("{:?}", x);  //打印：Foo { a: 100, b: true }
     println!("{:?}", y);  //打印：Foo { a: 100, b: false }
 }
```

从结果我们发现 let mut y = x 后，x 并没有因为所有权 move 而出现不可访问错误。因为 Foo 继承了 Copy 特性和 Clone 特性，所以例子中我们实现了这两个特性。

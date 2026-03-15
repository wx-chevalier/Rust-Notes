# Box

C 里面是通过 malloc/free 手动管理堆上内存空间的，而 Rust 则有多种方式，其中最常用的一种就是 Box,通过 Box::new() 可以在堆上申请一块内存空间，不像 C 里面一样堆上空间需要手动调用 free 释放，Rust 中是在编译期编译器借助 lifetime 对堆内存生命期进行分析，在生命期结束时自动插入 free。当前 Rust 底层即 Box 背后是调用 jemalloc 来做内存管理的，所以堆上空间是不需要程序员手动去管理释放的。很多时候你被编译器虐得死去活来时，那些 borrow, move, lifetime 错误其实就是编译器在教你认识内存布局，教你用 lifetime 规则去控制内存。大多数带 GC 的面向对象语言里面的对象都是借助 box 来实现的，比如常见的动态语言 Python/Ruby/JavaScript 等，其宣称的"一切皆对象(Everything is an object)"，里面所谓的对象基本上都是 boxed value。

boxed 值相对于 unboxed，内存占用空间会大些，同时访问值的时候也需要先进行 unbox，即对指针进行解引用再获取真正存储的值，所以内存访问开销也会大些。既然 boxed 值既费空间又费时间，为什么还要这么做呢？因为通过 box，所有对象看起来就像是以相同大小存储的，因为只需要存储一个指针就够了，应用程序可以同等看待各种值，而不用去管实际存储是多大的值，如何申请和释放相应资源。

Box 是堆上分配的内存，通过 Box::new() 会创建一个堆空间并返回一个指向堆空间的指针，nightly 版本中引入 box 关键词，可以用来取代 Box::new() 申请一个堆空间，也可以用在模式匹配上面：

```rs
#![feature(box_syntax, box_patterns)]
fn main() {
   let boxed = Some(box 5);
   match boxed {
       Some(box unboxed) => println!("Some {}", unboxed),
       None => println!("None"),
   }
}
```

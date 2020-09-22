# 泛型

泛型程序设计是程序设计语言的一种风格或范式。允许程序员在强类型程序设计语言中编写代码时使用一些以后才指定的类型，在实例化时（instantiate）作为参数指明这些类型（在 Rust 中，有的时候类型还可以被编译器推导出来）。各种程序设计语言和其编译器、运行环境对泛型的支持均不一样。Ada, Delphi, Eiffel, Java, C#, F#, Swift, and Visual Basic .NET 称之为泛型（generics）；ML, Scala and Haskell 称之为参数多态（parametric polymorphism）；C++与 D 称之为模板。具有广泛影响的 1994 年版的《Design Patterns》一书称之为参数化类型（parameterized type）。

在编程的时候，我们经常利用多态。通俗的讲，多态就是好比坦克的炮管，既可以发射普通弹药，也可以发射制导炮弹（导弹），也可以发射贫铀穿甲弹，甚至发射子母弹，大家都不想为每一种炮弹都在坦克上分别安装一个专用炮管，即使生产商愿意，炮手也不愿意，累死人啊。所以在编程开发中，我们也需要这样“通用的炮管”，这个“通用的炮管”就是多态。需要知道的是，泛型就是一种多态。

泛型主要目的是为程序员提供了编程的便利，减少代码的臃肿,同时极大丰富了语言本身的表达能力, 为程序员提供了一个合适的炮管。想想，一个函数，代替了几十个，甚至数百个函数，是一件多么让人兴奋的事情。泛型，可以理解为具有某些功能共性的集合类型，如 i8、i16、u8、f32 等都可以支持 add，甚至两个 struct Point 类型也可以 add 形成一个新的 Point。

# 提取函数来减少重复

在函数中，您写什么类型作为输入：

```rs
fn return_number(number: i32) -> i32 {
    println!("Here is your number.");
    number
}

fn main() {
    let number = return_number(5);
}
```

但是，如果您要使用的不仅仅是 i32，该怎么办？您可以为此使用泛型。泛型的意思是“也许是一种类型，也许是另一种类型”。对于泛型，可以使用内部类型的尖括号，如下所示：`<T>` 表示“放入函数中的任何类型”。通常，泛型使用带有一个大写字母（T，U，V 等）的类型，尽管您不必只使用一个字母。

```rs
fn return_number<T>(number: T) -> T {
    println!("Here is your number.");
    number
}

fn main() {
    let number = return_number(5);
}
```

重要的部分是函数名称后面的 `<T>`。没有这个，Rust 会认为 T 是一个具体的（具体=非泛型）类型，例如 String 或 i8。如果我们写出一个类型名，这更容易理解。看看将 T 更改为 MyType 时会发生什么：

```rs
fn return_number(number: MyType) -> MyType { // ⚠️
    println!("Here is your number.");
    number
}
```

如您所见，MyType 是具体的，而不是通用的。因此，我们需要编写此代码，这样它现在可以工作了：

```rs
fn return_number<MyType>(number: MyType) -> MyType {
    println!("Here is your number.");
    number
}

fn main() {
    let number = return_number(5);
}
```

因此，单个字母 T 用于人类的眼睛，但函数名后面的部分用于编译器的“眼睛”。没有它，它不是通用的。

## 最大值提取

当熟悉了这个技术以后，我们将使用相同的机制来提取一个泛型函数！如同你识别出可以提取到函数中重复代码那样，你也会开始识别出能够使用泛型的重复代码。

```rs
// 考虑一下这个寻找列表中最大值的小程序
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

这段代码获取一个整型列表，存放在变量 number_list 中。它将列表的第一项放入了变量 largest 中。接着遍历了列表中的所有数字，如果当前值大于 largest 中储存的值，将 largest 替换为这个值。如果当前值小于或者等于目前为止的最大值，largest 保持不变。当列表中所有值都被考虑到之后，largest 将会是最大值，在这里也就是 100。

如果需要在两个不同的列表中寻找最大值，我们可以重复上例中的代码，这样程序中就会存在两段相同逻辑的代码，虽然代码能够执行，但是重复的代码是冗余且容易出错的，并且意味着当更新逻辑时需要修改多处地方的代码。为了消除重复，我们可以创建一层抽象，在这个例子中将表现为一个获取任意整型列表作为参数并对其进行处理的函数。这将增加代码的简洁性并让我们将表达和推导寻找列表中最大值的这个概念与使用这个概念的特定位置相互独立。

```rs
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

largest 函数有一个参数 list，它代表会传递给函数的任何具体的 i32 值的 slice。函数定义中的 list 代表任何 &[i32]。当调用 largest 函数时，其代码实际上运行于我们传递的特定值上。如果我们有两个函数，一个寻找一个 i32 值的 slice 中的最大项而另一个寻找 char 值的 slice 中的最大项该怎么办？

```rs
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

如果想要定义一个 x 和 y 可以有不同类型且仍然是泛型的 Point 结构体，我们可以使用多个泛型类型参数。我们修改 Point 的定义为拥有两个泛型类型 T 和 U。其中字段 x 是 T 类型的，而字段 y 是 U 类型的：

```rs

```

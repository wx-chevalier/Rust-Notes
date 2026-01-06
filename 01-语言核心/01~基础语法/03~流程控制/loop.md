# 循环

多次执行同一段代码是很常用的，Rust 为此提供了多种 循环（loops）。一个循环执行循环体中的代码直到结尾并紧接着回到开头继续执行。Rust 有三种循环：loop、while 和 for。我们每一个都试试。

# loop

loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止。

```rs
fn main() { // This program will never stop
    loop {

    }
}
```

因此，让我们告诉编译器何时会中断。

```rs
fn main() {
    let mut counter = 0; // set a counter to 0
    loop {
        counter +=1; // increase the counter by 1
        println!("The counter is now: {}", counter);
        if counter == 5 { // stop when counter == 5
            break;
        }
    }
}

The counter is now: 1
The counter is now: 2
The counter is now: 3
The counter is now: 4
The counter is now: 5
```

## loop 命名

如果循环中有一个循环，则可以给它们命名。使用名称，您可以告诉 Rust 中断哪个循环。使用 `'`（称为“刻度”）和 `:` 为其命名：

```rs
fn main() {
    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop.");

    'first_loop: loop { // Give the first loop a name
        counter +=1;
        println!("The counter is now: {}", counter);
        if counter > 9 { // Starts a second loop inside this loop
            println!("Now entering the second loop.");

            'second_loop: loop { // now we are inside `second_loop
                println!("The second counter is now: {}", counter2);
                counter2 +=1;
                if counter2 == 3 {
                    break 'first_loop; // Break out of 'first_loop so we can exit the program
                }
            }
        }
    }
}
```

## 从循环返回

您还可以使用 break 返回值。您可以在 break 后立即写该值，并使用 `;`。这是一个带有循环和中断的示例，为 my_number 提供值。

```rs
fn main() {
    let mut counter = 5;
    let my_number = loop {
        counter +=1;
        if counter % 53 == 3 {
            break counter;
        }
    };
    println!("{}", my_number);
}
```

在循环之前，我们声明了一个名为 counter 的变量并初始化为 0。接着声明了一个名为 result 来存放循环的返回值。在循环的每一次迭代中，我们将 counter 变量加 1，接着检查计数是否等于 10。当相等时，使用 break 关键字返回值 `counter * 2`。循环之后，我们通过分号结束赋值给 result 的语句。最后打印出 result 的值，也就是 20。

# while

while 循环是在某些情况仍然成立时继续进行的循环。在每个循环中，Rust 都会检查它是否仍然为真。如果为假，Rust 将停止循环。

```rs
fn main() {
    let mut counter = 0;

    while counter < 5 {
        counter +=1;
        println!("The counter is now: {}", counter);
    }
}
```

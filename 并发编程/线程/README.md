# 线程创建与结束

Rust 对于线程的支持，和 C++11 一样，都是放在标准库中来实现的，详情请参见 `std::thread`，好在 Rust 从一开始就这样做了，不用像 C++那样等呀等。在语言层面支持后，开发者就不用那么苦兮兮地处理各平台的移植问题。通过 Rust 的源码可以看到，`std::thread` 其实就是对不同平台的线程操作的封装，相关 API 的实现都是调用操作系统的 API 来实现的，从而提供了线程操作的统一接口。

# 创建线程

首先，我们看一下在 Rust 中如何创建一个原生线程(native thread)。`std::thread` 提供了两种创建方式，都非常简单，第一种方式是通过 spawn 函数来创建，参见下面的示例代码：

```rs
use std::thread;

fn main() {
    // 创建一个线程
    let new_thread = thread::spawn(move || {
        println!("I am a new thread.");
    });
    // 等待新建线程执行完成
    new_thread.join().unwrap();
}

I am a new thread.
```

使用 spawn 这个函数，记得要先 use std::thread。注意 spawn 函数需要一个函数作为参数，且是 FnOnce 类型。接下来我们使用第二种方式创建线程，它比第一种方式稍微复杂一点，因为功能强大一点，可以在创建之前设置线程的名称和堆栈大小，参见下面的代码：

```rs
use std::thread;

fn main() {
    // 创建一个线程，线程名称为 thread1, 堆栈大小为4k
    let new_thread_result = thread::Builder::new()
                            .name("thread1".to_string())
                            .stack_size(4*1024*1024).spawn(move || {
        println!("I am thread1.");
    });
    // 等待新创建的线程执行完成
    new_thread_result.unwrap().join().unwrap();
}

// I am thread1.
```

通过和第一种方式的实现代码比较可以发现，这种方式借助了一个 Builder 类来设置线程名称和堆栈大小，除此之外，Builder 的 spawn 函数的返回值是一个 Result，在正式的代码编写中，可不能像上面这样直接 unwrap.join，应该判定一下。后面也会有很多类似的演示代码，为了简单说明不会做的很严谨。

# 线程结束

Rust 中不建议直接强行终止某个线程，如果深入接触过并发编程或多线程编程，就会知道强制终止一个运行中的线程，会出现诸多问题。比如资源没有释放，引起状态混乱，结果不可预期。强制干掉那一刻，貌似很爽地解决问题了，然而可能后患无穷。Rust 语言的一大特性就是安全，是绝对不允许这样不负责任的做法的。即使在其他语言提供了类似的接口，也不应该滥用。

那么在 Rust 中，新建的线程就只能让它自身自灭了吗？其实也有两种方式，首先介绍大家都知道的自生自灭的方式，线程执行体执行完成，线程就结束了。比如上面创建线程的第一种方式，代码执行完 println!("I am a new thread.");就结束了。如果像下面这样：

```rs
use std::thread;

fn main() {
    // 创建一个线程
    let new_thread = thread::spawn(move || {
        loop {
            println!("I am a new thread.");
        }
    });
    // 等待新创建的线程执行完成
    new_thread.join().unwrap();
}
```

线程就永远都不会结束，如果你用的还是古董电脑，运行上面的代码之前，请做好心理准备。在实际代码中，要时刻警惕该情况的出现（单核情况下，CPU 占用率会飙升到 100%），除非你是故意为之。线程结束的另一种方式就是，线程所在进程结束了。我们把上面这个例子稍作修改：

```rs
use std::thread;

fn main() {
    // 创建一个线程
    thread::spawn(move || {
        loop {
            println!("I am a new thread.");
        }
    });

    // 不等待新创建的线程执行完成
    // new_thread.join().unwrap();
}
```

同上面的代码相比，唯一的差别在于 main 函数的最后一行代码被注释了，这样主线程就不用等待新建线程了，在创建线程之后就执行完了，其所在进程也就结束了，从而新建的线程也就结束了。此处，你可能有疑问：为什么一定是进程结束导致新建线程结束？也可能是创建新线程的主线程结束而导致的？事实到底如何，我们不妨验证一下：

```rs
use std::thread;

fn main() {
    // 创建一个线程
    let new_thread = thread::spawn(move || {
        // 再创建一个线程
        thread::spawn(move || {
            loop {
                println!("I am a new thread.");
            }
        })
    });

    // 等待新创建的线程执行完成
    new_thread.join().unwrap();
    println!("Child thread is finish!");

    // 睡眠一段时间，看子线程创建的子线程是否还在运行
    thread::sleep_ms(100);
}

Child thread is finish!
I am a new thread.
I am a new thread.
......
```

这次我们在新建线程中还创建了一个线程，从而第一个新建线程是父线程，主线程在等待该父线程结束后，主动睡眠一段时间。这样做有两个目的，一是确保整个程序不会马上结束；二是如果子线程还存在，应该会获得执行机会，以此来检验子线程是否还在运行。

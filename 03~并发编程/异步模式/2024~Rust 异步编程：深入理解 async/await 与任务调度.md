# Rust 异步编程：深入理解 async/await 与任务调度

## 1. 异步编程基础

### 1.1 什么是异步编程

异步编程允许程序在等待某些操作完成时继续执行其他任务，而不是一直等待。在 Rust 中，这通过 `async/await` 语法实现。

### 1.2 基本语法示例

```rust
#[tokio::main]
async fn main() {
    // 异步函数声明
    async fn say_hello() {
        println!("Hello");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("World");
    }

    // 调用异步函数
    say_hello().await;
}
```

## 2. await 关键字详解

### 2.1 await 的本质

`await` 是 Rust 的内置关键字，不是函数。它用于暂停当前任务的执行，直到异步操作完成。

### 2.2 执行权的转移

```rust
async fn example() {
    println!("开始");
    let result = async_operation().await;  // 在这里交出执行权
    println!("完成");
}
```

当代码遇到 `.await` 时：

1. 如果操作未完成，当前任务暂停执行
2. 执行权返回给调度器
3. 调度器可以运行其他任务
4. 当操作完成时，任务恢复执行

## 3. 并发与并行

### 3.1 并发执行示例

```rust
#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(async {
        println!("任务1开始");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("任务1完成");
    });

    let task2 = tokio::spawn(async {
        println!("任务2开始");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("任务2完成");
    });

    // 等待所有任务完成
    let _ = tokio::join!(task1, task2);
}
```

### 3.2 CPU 执行原理

实际上，单个 CPU 核心在同一时刻只能执行一个任务。"并发"是通过快速切换任务实现的：

1. CPU 执行任务 1
2. 任务 1 需要等待时，切换到任务 2
3. 任务 2 需要等待时，可能切换回任务 1 或其他任务

## 4. CPU 密集型任务处理

### 4.1 什么是 CPU 密集型任务

CPU 密集型任务主要进行计算，很少有 I/O 操作，例如：

- 复杂数学计算
- 图像处理
- 密码哈希
- 大数据排序

### 4.2 处理示例

```rust
#[tokio::main]
async fn main() {
    // 使用 spawn_blocking 处理 CPU 密集型任务
    let cpu_task = tokio::task::spawn_blocking(|| {
        // 计算密集型示例：计算斐波那契数列
        fn fibonacci(n: u32) -> u64 {
            match n {
                0 => 0,
                1 => 1,
                n => fibonacci(n - 1) + fibonacci(n - 2)
            }
        }

        fibonacci(40)
    });

    // 等待计算结果
    let result = cpu_task.await.unwrap();
    println!("计算结果: {}", result);
}
```

## 5. 错误处理最佳实践

### 5.1 避免使用 unwrap()

```rust
// 不推荐
let result = some_async_operation().await.unwrap();

// 推荐
let result = match some_async_operation().await {
    Ok(value) => value,
    Err(e) => {
        eprintln!("操作失败: {}", e);
        return Err(e.into());
    }
};
```

### 5.2 使用 ? 操作符

```rust
async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let text = response.text().await?;
    Ok(text)
}
```

## 6. 实际应用示例

### 6.1 并发处理多个用户请求

```rust
async fn handle_multiple_users() {
    let user_tasks: Vec<_> = (1..=3).map(|user_id| {
        tokio::spawn(async move {
            // 模拟处理用户请求
            println!("处理用户 {} 的请求", user_id);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("用户 {} 的请求处理完成", user_id);
        })
    }).collect();

    // 等待所有用户请求处理完成
    for task in user_tasks {
        task.await.unwrap();
    }
}
```

## 7. 性能优化建议

1. **正确区分任务类型**

   - I/O 密集型任务使用 async/await
   - CPU 密集型任务使用 spawn_blocking

2. **避免阻塞异步运行时**

   - 不要在异步函数中执行长时间的 CPU 计算
   - 使用适当的线程池处理计算密集型任务

3. **合理使用资源**
   - 控制并发任务数量
   - 适当设置超时机制
   - 正确处理错误情况

## 结论

Rust 的异步编程模型提供了高效处理并发任务的能力。通过正确使用 async/await、合理区分任务类型、采用适当的错误处理策略，可以构建出高性能、可靠的异步应用程序。理解执行权的转移和 CPU 调度原理，有助于编写更好的异步代码。

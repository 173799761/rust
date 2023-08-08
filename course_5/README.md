# Course 5

实现：一个简单的声明宏并理解其代码结构，和编译过程。

#### 宏的分类

##### **声明式宏( \*declarative macros\* )** `macro_rules!` 



##### **过程宏( \*procedural macros\* )**

- `#[derive]`，派生宏，可以为目标结构体或枚举派生指定的代码，例如 `Debug` 特征
- 类属性宏(Attribute-like macro)，用于为目标添加自定义的属性
- 类函数宏(Function-like macro)，看上去就像是函数调用



#### 作业思路: 打印一条带有前缀的消息

声明式宏定义如下

```rust
//打印一条带有前缀的消息的声明式宏
macro_rules! print_with_prefix {
    ($prefix:expr, $($msg:expr),*) => {
        $(
            println!("{} {}", $prefix, $msg);
        )*
    };
}
```

在上面的示例中，我们使用 `macro_rules!` 定义了一个名为 `print_with_prefix` 的宏。该宏接受一个前缀表达式 `$prefix:expr`，后面是一个或多个消息表达式 `$($msg:expr),*`。

宏的定义采用了模式匹配的方式。当宏被调用时，它会根据传入的参数模式匹配，并根据模式执行相应的代码。

下面是如何使用该宏的示例：

```rust
fn main() {
    let prefix = "Message:";
    let msg1 = "Hello";
    let msg2 = "World";

    print_with_prefix!(prefix, msg1);

    println!("");
    print_with_prefix!(prefix, msg1, msg2);
}
```

在上面的示例中，我们定义了一个前缀字符串 `prefix` 和两个消息字符串 `msg1` 和 `msg2`。然后，我们使用 `print_with_prefix!` 宏来打印带有前缀的消息。第一个调用只打印一个消息，而第二个调用打印两个消息。



运行上述代码将输出以下内容：

```bash
Message: Hello

Message: Hello
Message: World
```



### 其他资料

- [Rust在线编译器](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)
- [Rust标准库文档](https://doc.rust-lang.org/std/index.html)
- [rust编译套件和中文手册](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html)
- [知乎教程](https://zhuanlan.zhihu.com/p/410291415)
- [Rust语言圣经https://course.rs/advance/macro.html](https://course.rs/advance/macro.html)
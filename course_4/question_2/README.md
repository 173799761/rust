# Question 2

搜索相关文档，为你自己定义的一个类型或多个类型实现加法运算（用符号 +），并构思使用Trait Object实现类型方法的调用

提示：

- 实现 Add trait

- 实现一个函数，接受Trait Object作为参数

  

思路 ：实现一个二维坐标的加法



标准库源码实现方式

```rust
#[doc(alias = "+")]
pub trait Add<Rhs = Self> {
    /// The resulting type after applying the `+` operator.
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12 + 1, 13);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[rustc_diagnostic_item = "add"]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

##### 1. 实现 Add trait

```rust
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

##### 2. 实现一个函数，接受Trait Object作为参数

```rust
fn point_add<T: Add<Output = T>>(num1: T, num2: T) -> T {
    num1 + num2
}
```

##### 3.工程运行结果

```rust
Point { x: 1.1, y: 2.2 } + Point { x: 3.3, y: 1.1 } = Point { x: 4.4, y: 3.3000000000000003 }
```





### 其他资料

- [Rust在线编译器](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)
- [Rust标准库文档](https://doc.rust-lang.org/std/index.html)
- [rust编译套件和中文手册](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html)
- [知乎教程](https://zhuanlan.zhihu.com/p/410291415)
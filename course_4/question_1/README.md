# Question 1

- 使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。

- 定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。

- 同时，说明其上两种不同实现方法的区别。

##### 1.使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。

```rust
struct Cat ;
impl Cat {
    fn catTalk(&self) {
        println!("catTalk:miao~");
    }
}


struct Dog;
impl Dog {
    fn dogTalk(&self) {
        println!("dogTalk:wang!");
    }
}

struct Cattle ;
impl Cattle {
    fn cattleTalk(&self) {
        println!("cattleTalk:mou~");
    }
}

enum EnumWrappedType {
    One(Cat),
    Two(Dog),
    Three(Cattle),
}

fn enumMethodDispatchTest() {
    let vec: Vec<EnumWrappedType> = vec![
        EnumWrappedType::One(Cat),
        EnumWrappedType::Two(Dog),
        EnumWrappedType::Three(Cattle),
    ];

    for enumWrappedType in vec.iter() {
        match enumWrappedType {
            EnumWrappedType::One(enum1) => enum1.catTalk(),
            EnumWrappedType::Two(enum2) => enum2.dogTalk(),
            EnumWrappedType::Three(enum3) => enum3.cattleTalk(),
        }
    }
}
```



输出

```bash
catTalk:miao~
dogTalk:wang!
cattleTalk:mou~
```

##### 2.定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。

```rust
struct Cat ;
struct Dog ;
struct Cattle ;
trait TraitMethodAnimal {
    fn talk(&self);
}

impl TraitMethodAnimal for Cat {
    fn talk(&self) {
        println!("TraitMethodAnimal cat talk : miao ~~~~~~~~~");
    }
}
impl TraitMethodAnimal for Dog {
    fn talk(&self) {
        println!("TraitMethodAnimal Dog talk : wang ~~~~~~~~~");
    }
}
impl TraitMethodAnimal for Cattle {
    fn talk(&self) {
        println!("TraitMethodAnimal Cattle talk : mou ~~~~~~~~~");
    }
}

fn traitTest() {
    let vec: Vec<Box<dyn TraitMethodAnimal>> = vec![
        Box::new(Cat),
        Box::new(Dog),
        Box::new(Cattle),
    ];

    for item in vec.iter() {
        item.talk();
    }
}
```

##### 3.说明其上两种不同实现方法的区别

- enum 实现允许调用不同类型上的不同方法，这些不同的方法完全独立，互不相关；而 trait 实现则只能调用 trait 里定义的方法，这些方法是不同类型共同拥有的方法，只是具体实现有可能有差异
- enum 实现中 Vec 里的元素可能有的类型只能是 enum 可变项中包含的类型，如果要增加新的类型，必须改动 enum 本身；而 trait 实现中 Vec 里的元素类型可以通过增加实现了该 trait 的类型来增加，不用改动任何已有的代码
- enum 实现中 Vec 里的元素即可以是所有者也可以是引用；而 trait 实现中 Vec 里的元素必须是动态引用`&dyn Trait`， `&mut dyn Trait`或`Box<dyn Trait>`，不能是所有者
- 在调用不同类型的方法时，enum 实现的版本通过`branch`实现静态分发；而 trait 实现的版本通过`vtable`实现动态分发，因此 enum 实现的调用效率更高一些



##### 4.工程运行结果

```bash
***************EnumMethodDispatchTest *******************************
catTalk:miao~
dogTalk:wang!
cattleTalk:mou~

***************traitTest *******************************
TraitMethodAnimal cat talk : miao ~~~~~~~~~
TraitMethodAnimal Dog talk : wang ~~~~~~~~~
TraitMethodAnimal Cattle talk : mou ~~~~~~~~~
```



### 其他资料

- [Rust在线编译器](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)
- [Rust标准库文档](https://doc.rust-lang.org/std/index.html)
- [rust编译套件和中文手册](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html)
- [知乎教程](https://zhuanlan.zhihu.com/p/410291415)
//打印一条带有前缀的消息的声明式宏
macro_rules! print_with_prefix {
    ($prefix:expr, $($msg:expr),*) => {
        $(
            println!("{} {}", $prefix, $msg);
        )*
    };
}

fn main() {
    let prefix = "Message:";
    let msg1 = "Hello";
    let msg2 = "World";

    print_with_prefix!(prefix, msg1);

    println!("");
    print_with_prefix!(prefix, msg1, msg2);
}

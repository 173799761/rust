# 学生管理系统

在学生管理系统中，分别实现学生，班级，课程，社团CRUD操作

### 1.Cargo常用管理工程命令

```bash
cargo --version
```



返回

```
cargo 1.71.1 (7f1d04c00 2023-07-29)
```



- ##### 初始化项目--生成`Cargo.toml`文件和`src/main.rs`文件


```bash
cargo new XXX
cargo init


src/
|  ├── main.rs
└── Cargo.toml
```

- ##### 构建项目


```rust
cargo build
```

- ##### 构建并运行项目


```rust
cargo run
```

- ##### 在不生成二进制文件的情况下构建项目来检查错误


```rust
cargo check
```



### 2.学生的CRUD

```rust
    pub fn addStu(&mut self, stu: Student) {
        self.stuMap.insert(stu.id, stu);
    }
    pub fn delStuById(&mut self, stuId: u32) {
        self.stuMap.remove(&stuId);
    }
    pub fn updateStu(&mut self, stu: Student) {
        self.stuMap.insert(stu.id, stu);
    }
    pub fn getStuById(&self, stuId: u32) -> Option<&Student> {
        self.stuMap.get(&stuId)
    }
```



### 3.班级的CRUD

```rust
    pub fn addClass(&mut self, class: Class) {
        self.classMap.insert(class.id, class);
    }

    pub fn delClassById(&mut self, classId: u32) {
        self.classMap.remove(&classId);
    }
    pub fn updateClass(&mut self, class: Class) {
        self.classMap.insert(class.id, class);
    }
    pub fn getClassById(&self, classId: u32) -> Option<&Class> {
        self.classMap.get(&classId)
    }
```



### 4.课程的CRUD

```rust
    pub fn addCourse(&mut self, course: Course) {
        self.courseMap.insert(course.id, course);
    }
    pub fn delCourseById(&mut self, courseId: u32) {
        self.courseMap.remove(&courseId);
    }
    pub fn updateCourse(&mut self, course: Course) {
        self.courseMap.insert(course.id, course);
    }
    pub fn getCourseById(&self, courseId: u32) -> Option<&Course> {
        self.courseMap.get(&courseId)
    }
```



### 5.社团的CRUD

```rust
    pub fn addClub(&mut self, club: Club) {
        self.clubMap.insert(club.id, club);
    }
    pub fn delClubById(&mut self, clubId: u32) {
        self.clubMap.remove(&clubId);
    }
    pub fn updateClub(&mut self, club: Club) {
        self.clubMap.insert(club.id, club);
    }
    pub fn getClubById(&self, clubId: u32) -> Option<&Club> {
        self.clubMap.get(&clubId)
    }
```



6.测试

```rust
    // 添加一些学生
    stuMgrSys.addStu(Student {
        id: 1,
        name: "Stu1".to_string(),
        classId: 1,
    });

    stuMgrSys.addStu(Student {
        id: 2,
        name: "Stu2".to_string(),
        classId: 1,
    });
    
        // 加载数据
    let loaded_sms = StuMgrSys::load_from_file("data.json");
    match loaded_sms {
        Ok(sms) => {
            // 打印学生列表
            for student in sms.stuMap.values() {
                println!("Student ID: {}, Name: {}", student.id, student.name);
            }
        }
        Err(e) => {
            println!("Error loading data: {}", e);
        }
    }
```

输出

```bash
Student ID: 1, Name: Stu1
Student ID: 2, Name: Stu2
```



### 其他资料

- [Rust在线编译器](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)
- [Rust标准库文档](https://doc.rust-lang.org/std/index.html)
- [rust编译套件和中文手册](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html)
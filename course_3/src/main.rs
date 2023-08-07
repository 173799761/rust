use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

// 学生结构体
#[derive(Clone, Serialize, Deserialize)]
pub struct Student {
    id: u32,
    name: String,
    classId: u32,
}

// 班级结构体
#[derive(Clone, Serialize, Deserialize)]
pub struct Class {
    id: u32,
    name: String,
    stuIds: Vec<u32>,
}


// 课程结构体
#[derive(Clone, Serialize, Deserialize)]
pub struct Course {
    id: u32,
    name: String,
    classId: u32,
}

// 社团结构体
#[derive(Clone, Serialize, Deserialize)]
pub struct Club {
    id: u32,
    name: String,
    stuIds: Vec<u32>,
}

// 学生管理系统
#[derive(Serialize, Deserialize)]
pub struct StuMgrSys {
    stuMap: HashMap<u32, Student>,
    classMap: HashMap<u32, Class>,
    courseMap: HashMap<u32, Course>,
    clubMap: HashMap<u32, Club>,
}

impl StuMgrSys {
    // 创建一个新的学生管理系统
    pub fn new() -> Self {
        Self {
            stuMap: HashMap::new(),
            classMap: HashMap::new(),
            courseMap: HashMap::new(),
            clubMap: HashMap::new(),
        }
    }

    // 学生的 CRUD 操作
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

    // 班级的 CRUD 操作
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

    // 课程的 CRUD 操作
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

    // 社团的 CRUD 操作
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


    // 保存数据到文件
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(self)?;
        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    // 从文件加载数据
    pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let system: StuMgrSys = serde_json::from_str(&contents)?;
        Ok(system)
    }
}


fn main() {
    println!("Hello, world!******************************");

    // 创建一个新的学生管理系统
    let mut stuMgrSys = StuMgrSys::new();

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

    // 将数据保存到文件
    let result = stuMgrSys.save_to_file("data.json");
    if let Err(e) = result {
        println!("Error saving data: {}", e);
    }

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
}

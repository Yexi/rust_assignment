#[derive(Debug, Eq, Clone)]
struct Student {
    id: usize,
    name: String,
    organizations: Option<String>,
    class: String,
    course: Option<String>,
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}


impl Student {
    pub fn new(id: usize, name: String, organizations: Option<String>, class: String,course: Option<String>) -> Self {
        Self { id, name, organizations, class, course }
    }
    pub fn print_student(&self) {
        println!("{}", self.id);
        println!("{}", self.name);
        println!("{:?}", self.organizations);
        println!("{}", self.class);
        println!("{:?}", self.course);
    }
}

struct StudentSystem {
    students: Vec<Student>,
}

impl StudentSystem {

    fn new() -> StudentSystem {
        Self { students: vec![] }
    }

    fn read_students(&self) {
        println!("{:?}", self.students);
    }

    fn add_student(&mut self, student: &Student) {
        self.students.push(student.clone());
    }
    
    fn update_student(&mut self, student: &Student) {
       match self.students.iter().position(|stu| stu == student) {
              Some(index) => {
               self.students[index] = student.clone(); 
            },
              None => {
                println!("student not found");
              },
          }  
    }
    
    fn delete_student(&mut self, student: &Student) {
          match self.students.iter().position(|stu| stu == student) {
              Some(index) => {
                self.students.remove(index);
            },
              None => {
                println!("student not found");
              },
          } 

        // let index = self.students.iter().position(|stu| *stu == student).unwrap();
        // self.students.remove(index);
    }
}



fn main() {
    //init
    let mut student_system: StudentSystem = StudentSystem::new();
    let mut s1: Student = Student { id: 1, name: String::from("Tom"), organizations: Some(String::from("A")), class: String::from("c3g2"), course: None };

    //create    
    student_system.add_student(&s1);
    student_system.read_students();

    //update
    s1.class = String::from("c2g3");
    s1.organizations = Some(String::from("Zero"));
    student_system.update_student(&s1);
    student_system.read_students();
    

    //delete
    student_system.delete_student(&s1);
    student_system.read_students(); 

}

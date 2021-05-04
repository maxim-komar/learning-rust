trait Person {
    fn name(&self) -> String;
}

// `Person` is a supertrait of `Student`
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// `CompSciStudent` is a subtrait of both `Programmer` and `Student`
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

#[allow(dead_code)]
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username(),
    )
}

fn main() {}

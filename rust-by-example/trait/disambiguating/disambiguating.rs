// A type can implement many different traits. What if two traits both 
// require the same name? For example, many traits might have a method
// name `get()`. They might event have different return types!
//
// Good news: because each trait implementation gets its own `impl` block,
//            it's clear which trait's `get` method you're implementing
//
// What about when it comes time to call those methods? To disambiguate 
// between them, we have to use Fully Qualified Syntax
trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);

    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}

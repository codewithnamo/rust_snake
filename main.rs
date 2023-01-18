
use snake_game::learning_rust::{Person, PersonId};


fn main() {
  let person = Person::new();
  // person.display_info();

  let id = PersonId::Passport(432);

  println!("{:?}", id);

  println!("{}", person.name())
} 
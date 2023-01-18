
use snake_game::learning_rust::{Person, Animal, log_info, log_info_2};


fn main() {
  let mut person = Person::new();
  let animal = Animal(String::from("dog"));

  person.change_age(38);

  log_info(person);
  log_info_2(&animal);
}
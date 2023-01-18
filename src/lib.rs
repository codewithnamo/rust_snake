
fn outsider() {
    println!("outsider fn!");
}

pub mod learning_rust {

    mod top_level {
        pub fn hi_there() {
            println!("hi there!");
        }

        pub mod low_level {
            pub fn hello_world() {
                println!("hello world");
            }
        }
    }

    pub trait Log {
        fn display_info(&self);
        fn alert_something(&self){
            println!("Default implementation")
        }
    }

    #[derive(Debug)]
    pub enum PersonId {
        Passport(u32),
        IdentityCard(u32, u32, u32),
    }

    pub struct Person {
        name: String,
        last_name: String,
        age: u32,
        id: PersonId,
    }

    pub struct Animal(pub String);

    impl Log for Animal {
        fn display_info(&self) {
            println!("{}", self.0)
        }

        fn alert_something(&self) {
            println!("Animal impelmentation")
        }
    }

    impl Log for Person {
        fn display_info(&self) {

            // absolute import in lib.rs
            // crate::outsider();

            // super -> accessing outside of the current module
            // it's like going ".."
            super::outsider();
           
            println!("{} {} {} {:?}", self.name, self.last_name, self.age, self.id)
        }
    }

    impl Person {
        pub fn new() -> Person {
            Person {
                name: "Default".to_string(),
                last_name: "Default".to_string(),
                age: 0,
                id: PersonId::IdentityCard(540, 320, 100),
            }
        }

        pub fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
            Person {
                name,
                last_name,
                age, 
                id
            }
        }

        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn change_age(&mut self, new_age: u32) {
            self.age = new_age;
        }
    }

    pub fn log_info(val: impl Log) {
        val.alert_something();
    }

    pub fn log_info_2(val: &dyn Log) {
        val.alert_something();
    }
}
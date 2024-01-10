use static_init::dynamic;

mod first_tests;
mod second_tests;

struct SharedResources {
    something: u64
}

impl SharedResources {
    fn init() -> Self {
        println!("Initializing shared resources");
        Self {
            something: 11
        }
    }

    pub fn get_something(&self) -> u64 {
        self.something
    }
}

impl Drop for SharedResources {
    fn drop(&mut self) {
        println!("Dropping shared resources");
    }
}

#[dynamic(drop)]
pub static mut SHARED: SharedResources =SharedResources::init();


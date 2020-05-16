#[derive(Clone)]
pub struct __move_capture_block__ {
    pub name: String,
}

impl<'a> FnOnce<(&'a str,)> for __move_capture_block__ {
    type Output = ();
    extern "rust-call" fn call_once(self, (message,): (&'a str,)) -> () {
        println!("{}, {}", self.name, message);
    }
}

impl<'a> FnMut<(&'a str,)> for __move_capture_block__ {
    extern "rust-call" fn call_mut(&mut self, (message,): (&'a str,)) -> () {
        println!("{}, {}", self.name, message);
    }
}

impl<'a> Fn<(&'a str,)> for __move_capture_block__ {
    extern "rust-call" fn call(&self, (message,): (&'a str,)) -> () {
        println!("{}, {}", self.name, message);
    }
}

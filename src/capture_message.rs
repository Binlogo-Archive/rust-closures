#[derive(Clone, Copy)]
pub struct __capture_message_block__<'a> {
    pub message: &'a String,
}

impl<'a> FnOnce<()> for __capture_message_block__<'a> {
    type Output = ();
    extern "rust-call" fn call_once(self, _: ()) -> () {
        println!("{}", *self.message)
    }
}

impl<'a> FnMut<()> for __capture_message_block__<'a> {
    extern "rust-call" fn call_mut(&mut self, _: ()) -> () {
        println!("{}", *self.message)
    }
}

impl<'a> Fn<()> for __capture_message_block__<'a> {
    extern "rust-call" fn call(&self, _: ()) -> () {
        println!("{}", *self.message)
    }
}

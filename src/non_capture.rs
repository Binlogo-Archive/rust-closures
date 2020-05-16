#[derive(Clone, Copy)]
pub struct __none_capture_block__ {}

impl FnOnce<()> for __none_capture_block__ {
    type Output = ();
    extern "rust-call" fn call_once(self, _: ()) -> () {}
}

impl FnMut<()> for __none_capture_block__ {
    extern "rust-call" fn call_mut(&mut self, _: ()) -> () {}
}

impl Fn<()> for __none_capture_block__ {
    extern "rust-call" fn call(&self, _: ()) -> () {}
}

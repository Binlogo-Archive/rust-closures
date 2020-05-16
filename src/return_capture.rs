pub struct __return__caputure_block__<'a, 'b> {
    pub counter: &'a mut u32,
    pub delta: &'b u32,
}

impl<'a, 'b> FnOnce<()> for __return__caputure_block__<'a, 'b> {
    type Output = u32;
    extern "rust-call" fn call_once(self, _: ()) -> u32 {
        *self.counter += self.delta;
        *self.counter
    }
}

impl<'a, 'b> FnMut<()> for __return__caputure_block__<'a, 'b> {
    extern "rust-call" fn call_mut(&mut self, _: ()) -> u32 {
        *self.counter += *self.delta;
        *self.counter
    }
}

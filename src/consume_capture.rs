pub struct __consume_capture_block__ {
    pub a: Vec<u32>,
}

impl FnOnce<()> for __consume_capture_block__ {
    type Output = u32;
    extern "rust-call" fn call_once(self, _: ()) -> u32 {
        let a = self.a.into_iter().map(|x| x * 2);
        a.sum::<u32>()
    }
}

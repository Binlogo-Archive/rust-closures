#![feature(unboxed_closures, fn_traits)]

mod capture_message;
mod consume_capture;
mod move_capture;
mod non_capture;
mod return_capture;

use capture_message::*;
use consume_capture::*;
use move_capture::*;
use non_capture::*;
use return_capture::*;

fn main() {
    // non capture
    let non_capture = __none_capture_block__ {};
    non_capture(); // Fn::call(&non_capture, ())

    // caputure maessage
    let message = "Hi".to_string();
    let capture_message = __capture_message_block__ { message: &message };
    capture_message();

    // move caputure
    let name = "Binboy".to_string();
    let move_capture = __move_capture_block__ { name };
    move_capture("You are awessome!");

    // returen capture
    let mut counter: u32 = 0;
    let delta: u32 = 2;
    let mut next = __return__caputure_block__ {
        counter: &mut counter,
        delta: &delta,
    };
    assert_eq!(FnMut::call_mut(&mut next, ()), 2);
    assert_eq!(FnMut::call_mut(&mut next, ()), 4);
    assert_eq!(FnMut::call_mut(&mut next, ()), 6);

    // consume capture
    let a = vec![0, 1, 2, 3, 4, 5];
    let transform = __consume_capture_block__ { a };
    println!("{}", transform());
    // transform(); // error[E0382]: use of moved value: `transform`
}

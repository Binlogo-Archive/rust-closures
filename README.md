# 神奇的闭包

通过拆解 Rust 中的闭包语法糖，来理解编程语言中闭包（Closure）的本质。

首先理解闭包（Closures）的含义：

> In programming languages, a closure is **a function or reference to a function** together with **a referencing environment**--a table storing a reference to each of the non-local variables (also called free variables or upvalues) of that function.

即，闭包就是**一个函数指针**和**该函数所处上下文环境**。其中，函数指针不能理解，最关键也是闭包和常规函数最大的区别就是，闭包捕获上下文环境的特性了。

```rust
let hi = "Hi!".to_string();
let closure = || println!("{}", hi);
closure(); // Hi!

// 以下代码会报错：无法动态捕获上下文
// error[E0434]: can't capture dynamic environment in a fn item
fn normal_function() {
  println!("{}", hi)
}
```

那么闭包是如何做到捕获上下文的呢？要回答这个问题，一是要理解闭包的定义，二是要知道闭包的使用是函数+上下文环境捕获的语法糖。

## 三种不同特性的函数声明 `trait Fn*`

```rust
pub trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

pub trait FnMut<Args>: FnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait Fn<Args>: FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}
```

> 更详细的文档参考：[FnOnce](https://doc.rust-lang.org/std/ops/trait.FnOnce.html), [FnMut](https://doc.rust-lang.org/std/ops/trait.FnMut.html), [Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html).

- `Args`：泛型的传入参数，对于闭包的实现，`Args`都是元组，以便同时传入多个参数

```rust
let zero_args_closure = || "hi";              // type Args = ()
let one_args_closure = |a: u32| ();           // type Args = (u32,)
let two_args_closure = |a: u32, b: String| a; // type Args = (u32, String)
```

- `Output`：泛型的返回参数

```rust
let zero_args_closure = || "hi";              // type Output = &`static str
let one_args_closure = |a: u32| ();           // type Output = ()
let two_args_closure = |a: u32, b: String| a; // type OUtput = u32
```

- `fn call*`：最终实际执行的函数实现

  - `FnOnce`：第一个参数声明是`self`，会移交调用者的所有权，因此编译器保证仅能被执行一次。
  - `FnMut`：第一个参数声明是`&mut self`，可变的引用，可修改调用者环境。
  - `Fn`：第一个参数声明是`&self`，不可变引用。

## 卸下闭包的糖衣

针对不同类型的闭包，试着由简单到复杂初步采用 [`fn_traits`](https://doc.rust-lang.org/unstable-book/library-features/fn-traits.html) 进行去语法糖实现。

### 无参数无返回值

先来看看无参数传入也无参数返回的一个空闭包：

```rust
let non_capture = || ();
non_capture();
```

采用 `trait Fn*` 的去糖实现：

```rust
#![feature(unboxed_closures, fn_traits)]

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

/// 
let non_capture = __none_capture_block__ {};
non_capture();
```

- `#![feature(unboxed_closures, fn_traits)]` 使用这两个 unstable 特性，编译器才允许手动实现闭包

  - [`unboxed_closures`](https://doc.rust-lang.org/unstable-book/language-features/unboxed-closures.html#unboxed_closures)
  - [`fn_traits`](https://doc.rust-lang.org/unstable-book/library-features/fn-traits.html)

--------------------------------------------------------------------------------

### 捕获上下文环境变量的引用

接下来，试试捕获1个上下文环境变量

```rust
let message = "Hi".to_string();
let capture_message = || println!("{}", message);
capture_message();
```

采用 `trait Fn*` 的去糖实现：

```rust
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

///
let message = "Hi".to_string();
let capture_message = __capture_message_block__ { message: &message };
capture_message()
```

- `__capture_message_block__` 捕获 `message`，并维护一致的生命周期

--------------------------------------------------------------------------------

### 捕获上下文环境变量并取得所有权

采用 `move` 的方式捕获环境上下文

```rust
let name = "Dylan".to_string();
let move_capture = move |message| println!("{}, {}", name, message);
move_capture("You are awessome!");
```

去糖实现：

```rust
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

let name = "Binboy".to_string();
let move_capture = __move_capture_block__ { name };
move_capture("You are awessome!");
```

- `name: String` 的声明会在捕获环境变量时，取得变量的所有权，即 `move` 语义
- `call*` 的输入参数为 `(message,): (&'a str,)`，对应闭包接受一个与闭包同声明周期的 `str` 类型参数，并使用元组做一层封包

--------------------------------------------------------------------------------

### 返回值

继续尝试如何返回值

```rust
let mut counter: u32 = 0;
let delta: u32 = 0;

let mut next = || {
  counter += delta;
  counter
}

assert_eq!(next(), 2);
assert_eq!(next(), 4);
assert_eq!(next(), 6);
```

去糖实现：

```rust
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

/// returen capture
let mut counter: u32 = 0;
let delta: u32 = 2;
let mut next = __return__caputure_block__ {
    counter: &mut counter,
    delta: δ,
};
assert_eq!(FnMut::call_mut(&mut next, ()), 2);
assert_eq!(FnMut::call_mut(&mut next, ()), 4);
assert_eq!(FnMut::call_mut(&mut next, ()), 6);
```

- `type Output` 指定泛型参数最终类型，并在实现 `call*` 方法时实际返回即可

### 获取上下文环境变量所有权并返回值

```rust
let a = vec![0, 1, 2, 3, 4, 5];
let transform = || {
  let a = a.into_iter().map(|x| x * 2);
  a.sum::<u32>()
}
println!("{}", transform());
// transform(); // error[E0382]: use of moved value: `transform`
```

去糖实现：

```rust
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
```

- 由于获取了环境变量所有权进行消费，因此仅实现`FnOnce`以符合语意

## 更多参考

- [Closures: Magic Functions](https://rustyyato.github.io/rust/syntactic/sugar/2019/01/17/Closures-Magic-Functions.html)

- [谈Objective-C block的实现](https://blog.devtang.com/2013/07/28/a-look-inside-blocks/)

- [Closures Capture Semantics: Catch them all!](https://alisoftware.github.io/swift/closures/2016/07/25/closure-capture-1/)

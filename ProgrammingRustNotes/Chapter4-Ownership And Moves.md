> Rust’s radical wager, the claim on which it stakes its success, and that forms the root of the language, is that even with these restrictions in place, you’ll find the language more than flexible enough for almost every task, and that the benefits—the elimination of broad classes of memory management and concurrency bugs—will justify the adaptations you’ll need to make to your style.

##### Ownership

> In Rust, however, the concept of ownership is built into the language itself, and enforced by compile-time checks. Every value has a single owner that determines its lifetime. When the owner is freed—*dropped*, in Rust terminology—the owned value is dropped too. 

##### Moves

默认move语义

> Moving values around like this may sound inefficient, but there are two things to keep in mind. First, the moves always apply to the value proper, not the heap storage they own. For vectors and strings, the *value proper* is the three-word header alone; the potentially large element arrays and text buffers sit where they are in the heap. Second, the Rust compiler’s code generation is good at “seeing through” all these moves; in practice, the machine code often stores the value directly where it belongs.

##### Moves and Indexed Content

```rust
let mut v = Vec::new();
for i in 101 .. 106 {
    v.push(i.to_string());
}
// let third = v[2]; // error: Cannot move out of index of Vec
// let fifth = v[4]; // here too

// Here are three possibilities:
// 1. Pop a value off the end of the vector:
let fifth = v.pop().expect("vector empty!");
assert_eq!(fifth, "105");

// 2. Move a value out of a given index in the vector,
// and move the last element into its spot:
let second = v.swap_remove(1);
assert_eq!(second, "102");

// 3. Swap in another value for the one we're taking out:
let third = std::mem::replace(&mut v[2], "substitute".to_string());
assert_eq!(third, "103");

// Let's see what's left of our vector.
assert_eq!(v, vec!["101", "104", "substitute"]);

let v = vec!["liberté".to_string(),
             "égalité".to_string(),
             "fraternité".to_string()];
// moves the vector out of v, leaving v uninitialized. 
for mut s in v {
    s.push('!');
    println!("{}", s);
}


struct Person { name: Option<String>, birth: i32 }

let mut composers = Vec::new();
composers.push(Person { name: Some("Palestrina".to_string()),
                        birth: 1525 });

// let first_name = composers[0].name; // cannot move out of index
// fix
let first_name = std::mem::replace(&mut composers[0].name, None);
assert_eq!(first_name, Some("Palestrina".to_string()));
assert_eq!(composers[0].name, None);
// or
let first_name = composers[0].name.take();
```

> The standard `Copy` types include all the machine integer and floating-point numeric types, the `char` and `bool` types, and a few others. A tuple or fixed-size array of `Copy`types is itself a `Copy` type.
>
> Only types for which a simple bit-for-bit copy suffices can be `Copy`.
>
> One of Rust’s principles is that costs should be apparent to the programmer. Basic operations must remain simple. Potentially expensive operations should be explicit, like the calls to `clone` in the earlier example that make deep copies of vectors and the strings they contain.

##### Rc and Arc: Shared Ownership

- > Rust provides the reference-counted pointer types `Rc` and `Arc`.

- > For any type `T`, an `Rc<T>` value is a pointer to a heap-allocated `T` that has had a reference count affixed to it. Cloning an `Rc<T>` value does not copy the `T`; instead, it simply creates another pointer to it, and increments the reference count.

- > A value owned by an `Rc` pointer is immutable. 

- 循环引用导致内存泄漏仍然存在，只是场景少，首先Rc指针指向的内容是不变的，正常情况下没办法构造，除非使用RefCell，内部可变

- 避免创建Rc引用循环可以使用weak pointer

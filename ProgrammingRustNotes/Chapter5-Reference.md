#### C++和Rust的引用区别
1. C++中引用都是隐式创造和解引用的；
2. Rust中引用是通过&显式创造和*解引用的，唯一的例外是.操作符，.可以隐式解引用，也可以在方法调用时隐式发生借用；
3. 底层两者都是地址
#### 引用比较
1. 类型相同则会一直找到引用对象，并比较
2. 类型不同会报错，比如&i32和&&i32或者i32比较
#### Slice和trait object的引用是胖指针
> “modifying collections while pointing into them is delicate territory in many languages. ”
```rust
extend(&mut wave, &wave);
```
Java中，可能会发生ConcurrentModificationException
 > If the Hashtable is structurally modified at any time after the iterator is created, in any way except through the iterator’s own remove method, the iterator will throw a ConcurrentModificationException.

 
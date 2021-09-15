#### Fixed-Width Numeric Types

##### Integer Types

- 数字类型和字符类型是不同的，但是，Rust提供了byte literals

  > *byte literals*, character-like literals for `u8` values: `b'X'` represents the ASCII code for the character `X`, as a `u8` value. For example, since the ASCII code for `A` is 65, the literals `b'A'` and `65u8`are exactly equivalent. Only ASCII characters may appear in byte literals.

- 整数算术运算overflow在debug构建中回panic，但是在release构建中，产生的值为数学上正确的结果对整数范围取模，如果要指定算术运算的行为，Rust提供了以下四种方法：

  1. Checked operations，返回Option，即如果数学上正确的结果落在目标类型的范围内则Some(v)，否则None；
  2. Wrapping operations，返回数学上正确的结果对目标类型范围取模；
  3. Saturating operations，返回最接近数学上正确结果的可表示值。换句话说，结果被 "钳制 "在该类型可以表示的最大和最小值上；
  4. Overflowing operations，返回元组(数学上正确的结果对目标类型范围取模, 是否溢出)。
     - overflowing_shl和overflowing_shr和上述描述有些偏离，是否溢出取决于偏移量是否大于类型的比特数，且偏移量会对类型的比特数取模

##### Floating-Point Types

> Unlike C and C++, Rust performs almost no numeric conversions implicitly

##### The bool Type

> Although a `bool` only needs a single bit to represent it, Rust uses an entire byte for a `bool` value in memory, so you can create a pointer to it.

##### Characters

- char代表单个Unicode字符，使用4个字节，但是String使用UTF8编码，内部使用vec\<u8\>
  - ascii字符可以使用'\xHH'表示
  - Unicode字符可以使用'\u{HHHHHH}'表示
  - char总是有效的Unicode code point，0x0000到0xD7FF，或者0xE000到0x10FFFF
  - char不会隐式转为其他类型，使用as转换操作符

##### Tuples

- () unit tuple被Rust用来表示携带无意义的值，比如使用Map实现Set

##### Pointer Types

- References

  - > At run time, a reference to an `i32` is a single machine word holding the address of the `i32`, which may be on the stack or in the heap.The expression `&x` produces a reference to `x`; in Rust terminology, we say that it *borrows a reference to `x`*. Given a reference `r`, the expression `*r` refers to the value `r` points to. These are very much like the `&` and `*`operators in C and C++. And like a C pointer, a reference does not automatically free any resources when it goes out of scope.

  - > Unlike C pointers, however, Rust references are never null: there is simply no way to produce a null reference in safe Rust.

  - > Rust uses this dichotomy between shared and mutable references to enforce a “single writer *or* multiple readers” rule. &T和&mut T

- Boxes

  - > The call to `Box::new`allocates enough memory to contain the tuple on the heap. When `b` goes out of scope, the memory is freed immediately, unless `b` has been *moved*—by returning it, for example. Moves are essential to the way Rust handles heap-allocated values

- Raw Pointers

  \*mut T和\*const T

##### Arrays

> The type `[T; N]` represents an array of `N` values, each of type `T`. An array’s size is a constant determined at compile time, and is part of the type; you can’t append new elements, or shrink an array.
>
>  Rust has no notation for an uninitialized array. (In general, Rust ensures that code can never access any sort of uninitialized value.)
>
>  Rust implicitly converts a reference to an array to a slice when searching for methods, so you can call any slice method on an array directly

##### Vectors

> The type `Vec<T>`, called a *vector of `T`s*, is a dynamically allocated, growable sequence of values of type `T`. 

##### Slices

> The types `&[T]` and `&mut [T]`, called a *shared slice of `T`s* and *mutable slice of `T`s*, are references to a series of elements that are a part of some other value, like an array or vector.

> A slice, written `[T]` without specifying the length, is a region of an array or vector. Since a slice can be any length, slices can’t be stored directly in variables or passed as function arguments. Slices are always passed by reference.
>
> A reference to a slice is a *fat pointer*: a two-word value comprising a pointer to the slice’s first element, and the number of elements in the slice.

##### String Types

- String Literals

  - > String literals are enclosed in double quotes.

    -  ```let speech = "\"Ouch!\" said the well.\n";```

  - > A raw string is tagged with the lowercase letter `r`. All backslashes and whitespace characters inside a raw string are included verbatim in the string. No escape sequences are recognized.  

    - ```let default_win_install_path = r"C:\Program Files\Gorillas";```
    - ```let pattern = Regex::new(r"\d+(\.\d+)*");```

- Byte Strings

  - > A string literal with the `b` prefix is a *byte string*. Such a string is a slice of `u8` values—that is, bytes—rather than Unicode text:

    - ```let method = b"GET";```
    - ```assert_eq!(method, &[b'G', b'E', b'T']);```

  - > Raw byte strings start with `br"`.

  - > Byte strings can’t contain arbitrary Unicode characters. They must make do with ASCII and `\xHH` escape sequences.

- Strings in Memory

> Rust strings are sequences of Unicode characters, but they are not stored in memory as arrays of `char`s. Instead, they are stored using UTF-8, a variable-width encoding. Each ASCII character in a string is stored in one byte. Other characters take up multiple bytes.

- Other String-Like Types

  - > Stick to `String` and `&str` for Unicode text.

  - > When working with filenames, use `std::path::PathBuf` and `&Path` instead.

  - > When working with binary data that isn’t UTF-8 encoded at all, use `Vec<u8>`and `&[u8]`.

  - > When working with environment variable names and command-line arguments in the native form presented by the operating system, use `OsString`and `&OsStr`.

  - > When interoperating with C libraries that use null-terminated strings, use `std::ffi::CString` and `&CStr`. 

##### Type aliases

```type Bytes = Vec<u8>;```


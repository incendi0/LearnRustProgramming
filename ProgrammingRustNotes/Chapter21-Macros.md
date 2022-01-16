#### Repetition
```
macro_rules! vec {
    ($elem:expr ; $n:expr) => {
        ::std::vec::from_elem($elem, $n)
    };
    ( $( $x:expr ),* ) => {
        <[_]>::into_vec(Box::new([ $( $x ),* ]))
    };
    ( $( $x:expr ),+ ,) => {
        vec![ $( $x ),* ]
    };
}
```
重复的pattern-$( PATTERN ),*，表示逗号分隔的重复PATTERN，类似正则，*表示0个或多个，可以换成？或者+，分隔符可以没有，也可以是;。
上例第三个pattern是为了匹配结尾的,。


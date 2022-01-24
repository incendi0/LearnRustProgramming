// 示例宏
// 1. 宏调用可以使用{}, [], ()
macro_rules! avec1 {
    () => {};
}
avec1!();
avec1![];
avec1!{}
// 2. 左侧pattern必须是语法正确，右侧必须编译通过
macro_rules! avec2 {
    ($arg1:ty, $arg2:expr, $arg3:path) => {};
    ($arg1:ty => $arg2:expr; $arg3:path) => {};
}
avec2!(u32, 1 + 1, ::std::collections);
avec2!(u32 => 1 + 1; ::std::collections);
// 3. cargo install cargo-expand
macro_rules! avec3 {
    ($arg1:ty => $arg2:ident) => {
        type $arg2 = $arg1;
    };
}
avec3!(u32 => AliasOfU32);
// 4. 宏内变量标识符和代码中的不在同一作用域
macro_rules! avec4 {
    ($x:ident) => {
        $x += 1;
    };
}
fn foo() {
    let mut x = 43;
    // 并没有传递ownership，只是名字
    avec4!(x);
}

macro_rules! avec5 {
    () => {
        Vec::new()
    };
    // double curly parenthesis
    // ($($element:expr),* $(,)?) => {{
    ($($element:expr),*) => {{
        let mut xs = Vec::new();
        // repeat pattern中对应次数
        $(xs.push($element);)*
        xs
    }};
    ($($element:expr,)*) => {{
        let mut xs = Vec::new();
        // repeat pattern中对应次数
        $(xs.push($element);)*
        xs
    }};
    ($element: expr; $count: expr) => {{
        // 5. 示例宏是替换的，注意副作用的表达式，可能会有意想不到的问题，类似c中的i++
        // xs.extend(::std::iter::repeat($element).take($count));
        let count = $count;
        let mut xs = Vec::with_capacity(count);
        xs.resize(count, $element);
        // xs.extend(::std::iter::repeat($element).take($count));
        // with bound check
        // for _ in 0..$count {
        //     xs.push(x.clone());
        // }
        xs
    }}
}

pub trait MaxValue {
    fn max_value() -> Self;
}

macro_rules! MAX_IMPL {
    ($t:ty) => {
        impl $crate::macros::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

MAX_IMPL!(i32);
MAX_IMPL!(u32);
MAX_IMPL!(u8);


#[cfg(test)]
mod tests {
    #[test]
    fn avec5_works() {
        let x: Vec<u32> = avec5![];
        assert!(x.is_empty());

        let xs = avec5!(42,1,);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 42);

        let xs = avec5!(42,2);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 42);

        let xs = avec5!(42; 2);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 42);
    }
}

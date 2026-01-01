//! 关于栈的库

///只是对Vec进行了一个包装的stack
#[derive(Debug, Clone)]
pub struct Stack<T> {
    pub data: Vec<T>,
}
impl<T> Stack<T> {
    ///初始化Stack
    pub fn new() -> Self {
        Self {
            data: Vec::<T>::new(),
        }
    }
}

///静态的栈
#[derive(Debug, Clone, Copy)]
pub struct StackStatic<T, const N: usize> {
    pub data: [T; N],
    pub len: usize,
}

/// Application of stack:bracket matching
///
/// Use Vec to directly act as stack.
/// # Examples
/// ```Rust
/// let s="{}{[()]}";
/// let result=check_bracket(s);
/// ```
pub fn check_bracket(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s
        .chars()
        .filter(|c| matches!(c, '(' | ')' | '[' | ']' | '{' | '}'))
    {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    stack.is_empty()
}

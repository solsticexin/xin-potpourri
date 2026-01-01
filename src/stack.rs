//! 关于栈的库

use std::ptr::null_mut;

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

///链栈:此链栈采用裸指针的方式
///
/// 此栈没有实现Drop trait 必须自己实现，或者手动管理内存
#[derive(Debug)]
pub struct LinkStack<T> {
    pub data: T,
    pub next: *mut LinkStack<T>,
}
impl<T> LinkStack<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            next: null_mut(),
        }
    }
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

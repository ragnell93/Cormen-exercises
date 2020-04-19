//! # Elementary data structures
//! Elementary data structures from part III of *"Algorithm Design"* by Cormen

///Basic stack with a defined capacity
pub struct Stack<T> {
    stack: Vec<T>,
    top: usize,
    capacity: usize,
}

impl<T> Stack<T> {
    ///Creates a new Stack with the given capacity
    pub fn new(capacity: usize) -> Stack<T> {
        let stack: Vec<T> = Vec::with_capacity(capacity);
        Stack {
            stack,
            top: 0,
            capacity,
        }
    }

    ///Checks if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    ///Push a new element to the stack
    ///## Panic
    /// If there is a stack overflow
    ///## Example
    /// ```
    /// let mut st: Stack<i32>  = Stack::new(3);
    /// st.push(66);
    /// ```
    pub fn push(&mut self, element: T) -> Result<(), &str> {
        if self.top + 1 == self.capacity {
            Err("Stack overflow")
        } else {
            self.stack.push(element);
            self.top += 1;
            Ok(())
        }
    }

    ///Pops the newest element in the stack
    ///## Panic
    /// If there is a stack undeflow
    ///## Example
    /// ```
    /// let mut st: Stack<i32>  = Stack::new(3);
    /// st.push(66);
    /// st.pop();
    /// ```
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            Err("Stack Underflow")
        } else {
            let popped = self.stack.pop().unwrap();
            self.top -= 1;
            Ok(popped)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_capacity() {
        let st: Stack<i32> = Stack::new(3);
        assert_eq!(st.capacity, 3);
    }

    #[test]
    fn new_top_is_zero() {
        let st: Stack<i32> = Stack::new(3);
        assert_eq!(st.top, 0);
    }

    #[test]
    fn stack_is_empty() {
        let st: Stack<i32> = Stack::new(3);
        assert!(st.is_empty());
    }

    #[test]
    #[should_panic]
    fn push_overflow() {
        let mut st: Stack<i32> = Stack::new(3);
        st.push(1).unwrap();
        st.push(2).unwrap();
        st.push(3).unwrap();
        st.push(4).unwrap();
    }

    #[test]
    #[should_panic]
    fn pop_underflow() {
        let mut st: Stack<i32> = Stack::new(3);
        st.pop().unwrap();
    }

    #[test]
    fn correct_push_pop() {
        let mut st: Stack<i32> = Stack::new(3);
        st.push(1).unwrap();
        assert_eq!(st.pop().unwrap(), 1);
    }

    #[test]
    fn stack_isnt_empty() {
        let mut st: Stack<i32> = Stack::new(3);
        st.push(1).unwrap();
        assert!(!st.is_empty())
    }
}

fn main() {}

#[test]
fn test_stack() {
    let mut s = Stack::new();
    for i in 1..20 {
        s.push(i);
    }

    s.show_info();

    {
        println!("elements: {:?}", s.get_all_elements());
    }

    for _i in 1..19 {
        let val: i32 = s.pop();
    }

    s.show_info();
    s.pop();

    {
        println!("empty: {:?}", s.empty());
        println!("elements: {:?}", s.get_all_elements());
    }
}

#[derive(Debug)]
pub struct Stack<T> {
    top: usize,             // 用于跟踪栈顶元素的索引
    elements: Vec<T>,       // 存储栈中元素的向量
}

impl<T: std::fmt::Debug> Stack<T> {
    /// 创建一个新的空栈，初始容量为16
    pub fn new() -> Self {
        Stack {
            top: 0,
            elements: Vec::with_capacity(16),
        }
    }

    /// 将元素压入栈中
    pub fn push(&mut self, item: T) {
        self.add_element(item);
        self.top += 1;
    }

    /// 添加元素到栈中（内部方法）
    fn add_element(&mut self, item: T) {
        self.elements.push(item);
    }

    /// 返回栈的容量
    pub fn capacity(&self) -> usize {
        self.elements.capacity()
    }

    /// 返回栈中元素的数量
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /// 检查栈是否为空
    pub fn empty(&self) -> bool {
        self.size() == 0
    }

    /// 弹出栈顶元素
    /// 如果栈为空，则引发 panic
    pub fn pop(&mut self) -> T {
        if self.empty() {
            panic!("Empty Stack");
        }
        self.top -= 1;
        self.elements.pop().unwrap()
    }

    /// 查看栈顶元素但不弹出
    /// 如果栈为空，则引发 panic
    pub fn peek(&self) -> &T {
        if self.empty() {
            panic!("Empty Stack");
        }
        let peek_idx: usize = self.size() - 1;
        self.get_item_by_idx(peek_idx)
    }

    /// 根据索引获取栈中的元素（内部方法）
    fn get_item_by_idx(&self, idx: usize) -> &T {
        self.check_range(idx);
        self.elements.get(idx).unwrap()
    }

    /// 清空栈中的所有元素
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// 获取栈中所有元素的引用
    pub fn get_all_elements(&self) -> &Vec<T> {
        &self.elements
    }

    /// 检查索引是否在有效范围内（内部方法）
    fn check_range(&self, idx: usize) {
        if idx >= self.size() {
            panic!("range must be between {} and  {}", 0, self.size());
        }
    }

    /// 显示栈的信息，包括是否为空、大小、容量和栈顶元素
    pub fn show_info(&self) {
        println!(
            "empty: {:?}, size: {}, capacity: {}, peek item: {:?}",
            self.empty(),
            self.size(),
            self.capacity(),
            self.peek()
        );
    }
}


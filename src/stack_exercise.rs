use std::collections::HashMap;
use crate::stack::Stack;

fn bracket_matching(expression: &str) -> bool {
    let mut char_lst: Vec<char> = Vec::new();
    for c in expression.chars() {
        char_lst.push(c);
    }
    let mut idx: usize = 0_usize;
    let mut balance: bool = true;
    let mut stack: Stack<char> = Stack::new();
    while idx < char_lst.len() && balance {
        let current_char = char_lst[idx];
        if is_left_bracket(current_char) {
            stack.push(current_char);
        } else if is_right_bracket(current_char) {
            if stack.empty() {
                return false;
            }
            let require_bracket: char = get_require_left_bracket(current_char);
            if *stack.peek() == require_bracket {
                stack.pop();
            } else {
                balance = false;
            }
        }

        idx += 1;
    }

    return balance;
}

fn is_left_bracket(c: char) -> bool {
    return c == '(' || c == '[' || c == '{';
}

fn is_right_bracket(c: char) -> bool {
    return c == ')' || c == ']' || c == '}';
}

fn get_require_left_bracket(right_bracket: char) -> char {
    return if ')' == right_bracket {
        '('
    } else if ']' == right_bracket {
        '['
    } else {
        '{'
    };
}

fn get_require_right_bracket(left_bracket: char) -> char {
    return if '(' == left_bracket {
        ')'
    } else if '[' == left_bracket {
        ']'
    } else {
        '}'
    };
}

fn divide_by_n(mut num: i32, n: i32) -> String {
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    let mut stack: Stack<i32> = Stack::new();
    while num > 0 {
        let rem = num % n;
        stack.push(rem);
        num /= n;
    }
    let mut bin_str = "".to_string();
    while !stack.empty() {
        let rem = stack.pop() as usize;
        bin_str += &digits[rem].to_string();
    }

    bin_str
}


fn calculate_from_postfix(postfix: &str) -> Option<i32> {
    // 少 于 五 个 字 符 ， 不 是 有 效 的 后 缀 表 达 式 ， 因 为表 达 式
    // 至 少 两 个 操 作 数 加 一 个 操 作 符 ， 还 需 要 两 个 空 格 隔 开
    if postfix.len() < 5 { return None; }
    let mut op_stack = Stack::new();
    for token in postfix.split_whitespace() {
        if "0" <= token && token <= "9" {
            op_stack.push(token.parse::<i32>().unwrap());
        } else {
            // 对 于 减 法 和 除 法 ， 顺 序 有要 求
            // 所 以 先 出 栈 的 是 第 二 个 操 作 数
            let op2 = op_stack.pop();
            let op1 = op_stack.pop();
            let res = do_calc(token, op1, op2);
            op_stack.push(res);
        }
    }
    Some(op_stack.pop())
}

// 执 行 四 则 数 学 运 算
fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    if "+" == op {
        op1 + op2
    } else if "-" == op {
        op1 - op2
    } else if "*" == op {
        op1 * op2
    } else {
        if 0 == op2 {
            panic!("ZeroDivisionError: Invalid operation!");
        }
        op1 / op2
    }
}

fn convert_infix_to_postfix(infix: &str) -> Option<String> {
    // 括 号 匹 配 检 验
    if !bracket_matching(infix) { return None; }
    // 设 置 各 个 符 号 的 优 先 级
    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);
    // ops 保 存 操 作 符 号 、postfix 保 存 后 缀 表 达 式
    let mut op_stack = Stack::new();
    let mut postfix = Vec::new();
    for token in infix.split_whitespace() {
        // 0 - 9和 A-Z 范 围 字 符 入 栈
        if ("A" <= token && token <= "Z") ||
            ("0" <= token && token <= "9") {
            postfix.push(token);
        } else if "(" == token {
            // 遇到 开符 号 ， 将 操 作 符 入 栈
            op_stack.push(token);
        } else if ")" == token {
            // 遇到 闭符 号 ， 将 操 作 数 入 栈
            let mut top = op_stack.pop();
            while top != "(" {
                postfix.push(top);
                top = op_stack.pop();
            }
        } else {
            // 比 较 符 号 优 先 级 来 决 定 操 作 符 是 否 加 入 postfix
            while (!op_stack.empty()) &&
                (prec[op_stack.peek()] >= prec[token]) {
                postfix.push(op_stack.pop());
            }
            op_stack.push(token);
        }
    }
    // 剩 下 的 操 作 数 入 栈
    while !op_stack.empty() {
        postfix.push(op_stack.pop())
    }
    // 出 栈 并 组 成 字 符 串
    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }
    Some(postfix_str)
}

#[test]
fn test_bracket_matching() {
    assert!(bracket_matching("(a+b)(c*d)func()"));
    assert!(!bracket_matching("()][()"));
    assert!(bracket_matching("(){}[]"));
    assert!(!bracket_matching("(){)[}"));
    assert!(bracket_matching("()"));
    assert!(bracket_matching("()[]{}"));
    assert!(!bracket_matching("(]"));
    assert!(!bracket_matching("([)]"));
    assert!(bracket_matching("{[]}"));
}

#[test]
fn test_divide_by_n() {
    assert_eq!(divide_by_n(3, 2), "11");
    assert_eq!(divide_by_n(7, 2), "111");
    assert_eq!(divide_by_n(8, 8), "10");
    assert_eq!(divide_by_n(15, 8), "17");
    assert_eq!(divide_by_n(31, 16), "1F");
    assert_eq!(divide_by_n(11, 16), "B");
}

#[test]
fn test_postfix_eval() {
    let infix = "( 1 + 2 ) * ( 3 + 4 )";
    let postfix = convert_infix_to_postfix(infix);
    match postfix {
        Some(val) => {
            println!("infix: {infix} -> postfix: {val}");
            let val = calculate_from_postfix(&val);
            match val {
                Some(val) => {
                    assert_eq!(val, 21);
                }
                None => {}
            }
        }
        None => {
            println!("{infix} is not a corret infix string");
        }
    }
}
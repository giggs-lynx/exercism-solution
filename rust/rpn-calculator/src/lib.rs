#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for o in inputs {
        match o {
            CalculatorInput::Value(x) => stack.push(*x),
            _ => {
                if stack.len() < 2 || !calc(&mut stack, o) {
                    return None;
                }
            }
        }
    }

    if stack.len() != 1 {
        None
    } else {
        stack.pop()
    }
}

pub fn calc(stack: &mut Vec<i32>, op: &CalculatorInput) -> bool {
    let second = stack.pop();
    let first = stack.pop();

    if let (Some(f), Some(s)) = (first, second) {
        let o = match *op {
            CalculatorInput::Add => f + s,
            CalculatorInput::Subtract => f - s,
            CalculatorInput::Multiply => f * s,
            CalculatorInput::Divide => f / s,
            _ => return false,
        };

        stack.push(o);
        return true;
    }

    false
}

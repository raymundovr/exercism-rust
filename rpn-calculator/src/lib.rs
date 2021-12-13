#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn apply(operator: &CalculatorInput, stack: &Vec<i32>) -> Option<i32> {
    if stack.len() < 2 {
        return None;
    }

    let (last, before_last) = (stack[stack.len() - 1], stack[stack.len() - 2]);

    match operator {
        CalculatorInput::Add => Some(before_last + last),
        CalculatorInput::Subtract => Some(before_last - last),
        CalculatorInput::Divide => Some(before_last / last),
        CalculatorInput::Multiply => Some(before_last * last),
        _ => None
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for input in inputs.iter() {
        match input {
            CalculatorInput::Value(number) => stack.push(*number),
            _ => {
                if let Some(result) = apply(input, &stack) {
                    stack.drain((stack.len() - 2)..);
                    stack.push(result);
                } else {
                    return None;
                }
            }
        }
    }

    match stack.len() {
        1 => Some(stack[0]),
        _ => None
    }
}

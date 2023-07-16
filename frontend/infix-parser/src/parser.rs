use std::collections::VecDeque;

#[derive(Clone)]
struct Operator {
    symbol: char,
    precedence: i8,
    left_assoc: bool,
}

fn all_operators() -> Vec<Operator> {
    return vec![
        Operator {
            symbol: '+',
            precedence: 2,
            left_assoc: true,
        },
        Operator {
            symbol: '-',
            precedence: 2,
            left_assoc: true,
        },
        Operator {
            symbol: '^',
            precedence: 4,
            left_assoc: false,
        },
        Operator {
            symbol: '*',
            precedence: 3,
            left_assoc: true,
        },
        Operator {
            symbol: '/',
            precedence: 3,
            left_assoc: true,
        },
    ];
}

fn is_operator(c: char) -> bool {
    let vec: Vec<char> = all_operators().iter().map(|x| x.symbol).collect();
    return vec.contains(&c);
}

fn get_operator(c: char) -> Option<Operator> {
    return all_operators().iter().find(|x| x.symbol == c).cloned();
}

pub fn shunting_yard(tokens: String) -> String {
    let vec_tokens: Vec<_> = tokens.chars().collect();
    let mut output: VecDeque<char> = VecDeque::new();
    let mut operators: VecDeque<char> = VecDeque::new();

    for token in vec_tokens.iter() {
        if token.is_numeric() {
            output.push_front(*token);
        } else if is_operator(*token) {
            loop {
                match &mut operators.front() {
                    Some('(') => break,
                    Some(operator) => {
                        let o1 = get_operator(*token).unwrap();
                        let o2 = get_operator(**operator).unwrap();
                        if o2.precedence > o1.precedence
                            || (o2.precedence == o1.precedence && o1.left_assoc)
                        {
                            operators.pop_front();
                            output.push_front(o2.symbol);
                        } else {
                            break;
                        }
                    }
                    None => break,
                }
            }

            operators.push_front(*token);
        } else if *token == '(' {
            operators.push_front('(');
        } else if *token == ')' {
            loop {
                match &mut operators.pop_front() {
                    Some('(') => break,
                    Some(operator) => output.push_front(*operator),
                    None => return "Mismatched parentheses".to_string(),
                }
            }
        }
    }

    loop {
        match operators.pop_front() {
            Some(operator) => output.push_front(operator),
            None => break,
        }
    }

    let mut answer = String::new();

    loop {
        match output.pop_back() {
            Some(token) => answer.push(token),
            None => break,
        }
    }

    return answer;
}

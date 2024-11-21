use std::{fmt::Display, marker::PhantomData, rc::Rc};

use gh_workflow_macros::Expr;

pub struct Expr<A> {
    marker: PhantomData<A>,
    step: Step,
}

#[derive(Default, Clone)]
enum Step {
    #[default]
    Root,
    Select {
        name: Rc<String>,
        object: Box<Step>,
    },
}

impl Step {
    fn select(name: impl Into<String>) -> Step {
        Step::Select { name: Rc::new(name.into()), object: Box::new(Step::Root) }
    }
}

impl<A> Expr<A> {
    fn new() -> Self {
        Expr { marker: PhantomData, step: Step::Root }
    }

    fn select<B>(&self, path: impl Into<String>) -> Expr<B> {
        Expr {
            marker: PhantomData,
            step: Step::Select {
                name: Rc::new(path.into()),
                object: Box::new(self.step.clone()),
            },
        }
    }
}

impl<A> Display for Expr<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack: Vec<Step> = vec![self.step.clone()];

        write!(f, "{{{{ ")?;

        loop {
            match stack.pop() {
                None => break,
                Some(step) => match step {
                    Step::Root => break,
                    Step::Select { name, object } => {
                        if matches!(*object, Step::Root) {
                            write!(f, "{}", name.replace('"', ""))?;
                        } else {
                            stack.push(Step::select(name.as_str()));
                            stack.push(Step::select("."));
                            stack.push(*object);
                        }
                    }
                },
            }
        }

        write!(f, " }}}}")
    }
}

#[derive(Expr)]
#[allow(dead_code)]
pub struct Github {
    action: String,
    action_path: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_expr() {
        let github = Expr::github(); // Expr<Github>

        assert_eq!(github.to_string(), "{{ github }}");

        let action = github.action(); // Expr<String>
        assert_eq!(action.to_string(), "{{ github.action }}");

        let action_path = github.action_path(); // Expr<String>
        assert_eq!(action_path.to_string(), "{{ github.action_path }}");
    }
}

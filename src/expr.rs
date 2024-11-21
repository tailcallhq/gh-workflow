use std::marker::PhantomData;

use gh_workflow_macros::Expr;

#[derive(Default, Clone)]
enum Step {
    #[default]
    Root,
    Get {
        name: String,
        object: Box<Step>,
    },
}

pub struct Expr<A> {
    marker: PhantomData<A>,
    step: Step,
}

impl<A> Expr<A> {
    fn new() -> Self {
        Expr { marker: PhantomData, step: Step::Root }
    }

    fn get<B>(&self, path: impl Into<String>) -> Expr<B> {
        Expr {
            marker: PhantomData,
            step: Step::Get { name: path.into(), object: Box::new(self.step.clone()) },
        }
    }
}
impl<A> ToString for Expr<A> {
    fn to_string(&self) -> String {
        let mut step = &self.step;
        let mut parts = Vec::new();

        loop {
            match step {
                Step::Root => break,
                Step::Get { name, object } => {
                    parts.push(name.clone());
                    step = object;
                }
            }
        }

        parts.reverse();
        let path = parts.join(".").replace('"', "");

        format!("{{{{ {} }}}}", path)
    }
}

#[derive(Expr)]
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

use crate::Substitutions;

#[derive(Debug, Clone)]
pub enum Ty {
    Named(String),
    Variable(String),
    Func { from: Box<Ty>, to: Box<Ty> },
}

impl Ty {
    pub(crate) fn apply_subs(self, subs: &Substitutions) -> Ty {
        match self {
            Ty::Named(_) => self,
            Ty::Variable(ref name) => subs.0.get(name).cloned().unwrap_or(self),
            Ty::Func { from, to } => Ty::Func {
                from: Box::new(from.apply_subs(subs)),
                to: Box::new(to.apply_subs(subs)),
            },
        }
    }

    pub(crate) fn unify(self, y: Ty) -> Substitutions {
        match (self, y) {
            (Ty::Named(x), Ty::Named(y)) if x == y => Substitutions::default(),
            (Ty::Variable(x), y) => y.var_bind(x),
            (x, Ty::Variable(y)) => x.var_bind(y),
            (
                Ty::Func {
                    from: x_from,
                    to: x_to,
                },
                Ty::Func {
                    from: y_from,
                    to: y_to,
                },
            ) => {
                let mut subs = x_from.unify(*y_from);

                subs += x_to.apply_subs(&subs).unify(y_to.apply_subs(&subs));

                subs
            }
            _ => panic!("Type mismatch"),
        }
    }

    fn var_bind(self, name: String) -> Substitutions {
        if matches!(self, Ty::Variable(ref ty_name) if *ty_name == name) {
            Substitutions::default()
        } else if self.contains(&name) {
            panic!("Type contains self reference")
        } else {
            let mut subs = Substitutions::default();
            subs.0.insert(name, self);
            subs
        }
    }

    fn contains(&self, name: &str) -> bool {
        match self {
            Ty::Named(_) => false,
            Ty::Variable(var_name) => var_name == name,
            Ty::Func { from, to } => from.contains(name) || to.contains(name),
        }
    }
}

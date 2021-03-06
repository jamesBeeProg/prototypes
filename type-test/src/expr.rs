use crate::{
    builder,
    data::{Context, Substitutions},
    error::Result,
    ty::Ty,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Func(Box<FuncExpr>),
    Call(Box<CallExpr>),
    If(Box<IfExpr>),
    Let(Box<LetExpr>),
}

impl Expr {
    pub fn infer(self, ctx: &mut Context) -> Result<(Ty, Substitutions)> {
        match self {
            Expr::Number(_) => Ok((Ty::Named("Number".to_string()), Substitutions::default())),
            Expr::Variable(name) => Ok((ctx.get(&name)?.clone(), Substitutions::default())),
            Expr::Func(it) => it.infer(ctx),
            Expr::Call(it) => it.infer(ctx),
            Expr::If(it) => it.infer(ctx),
            Expr::Let(it) => it.infer(ctx),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncExpr {
    pub param: String,
    pub body: Expr,
}

impl FuncExpr {
    fn infer(self, ctx: &mut Context) -> Result<(Ty, Substitutions)> {
        let param_ty = ctx.new_ty_variable();
        let mut ctx = ctx.with(self.param, param_ty.clone());
        let (body_ty, subs) = self.body.infer(&mut ctx)?;
        let param_ty = param_ty.substitute(&subs);

        Ok((builder::ty_func(param_ty, body_ty), subs))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub func: Expr,
    pub arg: Expr,
}

impl CallExpr {
    fn infer(self, ctx: &mut Context) -> Result<(Ty, Substitutions)> {
        let (func_ty, mut subs) = self.func.infer(ctx)?;
        let (arg_ty, new_subs) = self.arg.infer(&mut ctx.substitute(&subs))?;

        let new_var = ctx.new_ty_variable();
        subs += new_subs;

        let new_subs = builder::ty_func(arg_ty.clone(), new_var).unify(func_ty.clone())?;
        let func_ty = func_ty
            .substitute(&new_subs)
            .try_into_func()
            .expect("Should still be a func type here");

        subs += new_subs;
        subs += func_ty.from.substitute(&subs).unify(arg_ty)?;

        Ok((func_ty.to.substitute(&subs), subs))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub condition: Expr,
    pub true_branch: Expr,
    pub false_branch: Expr,
}

impl IfExpr {
    fn infer(self, ctx: &mut Context) -> Result<(Ty, Substitutions)> {
        let (condition_ty, mut condition_subs) = self.condition.infer(ctx)?;
        let mut subs = condition_ty.unify(Ty::Named("Bool".to_string()))?;
        condition_subs += subs.clone();

        let mut ctx = ctx.substitute(&condition_subs);
        let (true_branch_ty, new_subs) = self.true_branch.infer(&mut ctx)?;
        subs += new_subs.clone();

        let mut ctx = ctx.substitute(&new_subs);
        let (false_branch_ty, new_subs) = self.false_branch.infer(&mut ctx)?;
        subs += new_subs;

        let true_branch_ty = true_branch_ty.substitute(&subs);
        let false_branch_ty = false_branch_ty.substitute(&subs);

        let new_subs = true_branch_ty.clone().unify(false_branch_ty)?;
        subs += new_subs.clone();

        Ok((true_branch_ty.substitute(&new_subs), subs))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetExpr {
    pub name: String,
    pub expr: Expr,
    pub body: Expr,
}

impl LetExpr {
    fn infer(self, ctx: &mut Context) -> Result<(Ty, Substitutions)> {
        let (expr_ty, mut subs) = self.expr.infer(ctx)?;
        let mut ctx = ctx.substitute(&subs).with(self.name, expr_ty);
        let (body_ty, new_subs) = self.body.infer(&mut ctx)?;
        subs += new_subs;
        Ok((body_ty, subs))
    }
}

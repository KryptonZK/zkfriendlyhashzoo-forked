use std::{
    ops::{Index, IndexMut},
    usize,
};

use bellman_ce::pairing::ff::PrimeField;

#[derive(Clone, Debug)]
pub struct Constraint<S: PrimeField> {
    lc: Vec<S>,
}

impl<S: PrimeField> Constraint<S> {
    pub fn new(vars: usize, var_index: usize) -> Self {
        let mut lc = vec![S::zero(); vars];
        lc[var_index] = S::one();

        Constraint { lc }
    }

    pub fn one(vars: usize) -> Self {
        let mut lc = vec![S::zero(); vars];
        lc[0] = S::one();

        Constraint { lc }
    }

    pub fn zeros(vars: usize) -> Self {
        let lc = vec![S::zero(); vars];

        Constraint { lc }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, S> {
        self.lc.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, S> {
        self.lc.iter_mut()
    }
}

impl<'s, S: PrimeField> IntoIterator for &'s Constraint<S> {
    type Item = &'s S;

    type IntoIter = ::std::slice::Iter<'s, S>;
    fn into_iter(self) -> Self::IntoIter {
        self.lc.iter()
    }
}

impl<'s, S: PrimeField> IntoIterator for &'s mut Constraint<S> {
    type Item = &'s mut S;

    type IntoIter = ::std::slice::IterMut<'s, S>;
    fn into_iter(self) -> Self::IntoIter {
        self.lc.iter_mut()
    }
}

impl<S: PrimeField> Index<usize> for Constraint<S> {
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.lc[index]
    }
}

impl<S: PrimeField> IndexMut<usize> for Constraint<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.lc[index]
    }
}

#[derive(Clone, Debug)]
pub struct R1CS<S: PrimeField> {
    vars: usize,
    var_index: usize,
    lhs: Vec<Constraint<S>>,
    rhs: Vec<Constraint<S>>,
    res: Vec<Constraint<S>>,
}

impl<S: PrimeField> R1CS<S> {
    pub fn new(vars: usize) -> Self {
        R1CS {
            vars,
            var_index: 1, // 0 is reserved for 1
            lhs: Vec::new(),
            rhs: Vec::new(),
            res: Vec::new(),
        }
    }

    pub fn new_variable(&mut self) -> Constraint<S> {
        let var = Constraint::new(self.vars, self.var_index);
        self.var_index += 1;
        var
    }

    pub fn register_constraints(
        &mut self,
        lhs: &Constraint<S>,
        rhs: &Constraint<S>,
        res: &Constraint<S>,
    ) {
        self.lhs.push(lhs.to_owned());
        self.rhs.push(rhs.to_owned());
        self.res.push(res.to_owned());
    }

    pub fn linear_constraints(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let num = constraints.len();
        let mut result = Vec::with_capacity(num);
        let one = Constraint::one(self.vars);

        for el in constraints {
            let var = self.new_variable();
            self.register_constraints(el, &one, &var);
            result.push(var);
        }
        result
    }

    pub fn scalar_multiplication(
        &self,
        constraints: &[Constraint<S>],
        vector: &[S],
    ) -> Constraint<S> {
        let mut new_constraint = Constraint::zeros(self.vars);
        for (con, vec) in constraints.iter().zip(vector.iter()) {
            let tmp = self.scale(con, vec);
            new_constraint = self.addition(&new_constraint, &tmp);
        }
        new_constraint
    }

    pub fn scale(&self, constraint: &Constraint<S>, scalar: &S) -> Constraint<S> {
        let mut result = constraint.to_owned();
        for el in &mut result {
            el.mul_assign(scalar);
        }
        result
    }

    pub fn multiplication_new(
        &mut self,
        constraint_lhs: &Constraint<S>,
        constraint_rhs: &Constraint<S>,
    ) -> Constraint<S> {
        let result = self.new_variable();
        self.register_constraints(constraint_lhs, constraint_rhs, &result);
        result
    }

    pub fn multiplication(
        &mut self,
        constraint_lhs: &Constraint<S>,
        constraint_rhs: &Constraint<S>,
        constraint_res: &Constraint<S>,
    ) {
        self.register_constraints(constraint_lhs, constraint_rhs, constraint_res);
    }

    pub fn addition(
        &self,
        constraint1: &Constraint<S>,
        constraint2: &Constraint<S>,
    ) -> Constraint<S> {
        let mut result = constraint1.to_owned();
        for (a, b) in result.iter_mut().zip(constraint2.iter()) {
            a.add_assign(b);
        }
        result
    }

    pub fn subtraction(
        &self,
        constraint1: &Constraint<S>,
        constraint2: &Constraint<S>,
    ) -> Constraint<S> {
        let mut result = constraint1.to_owned();
        for (a, b) in result.iter_mut().zip(constraint2.iter()) {
            a.sub_assign(b);
        }
        result
    }

    pub fn add_const(&self, constraint: &Constraint<S>, rc: &S) -> Constraint<S> {
        let mut res = constraint.clone();
        res[0].add_assign(rc);
        res
    }

    pub fn sub_const(&self, constraint: &Constraint<S>, rc: &S) -> Constraint<S> {
        let mut res = constraint.clone();
        res[0].sub_assign(rc);
        res
    }

    pub fn add_rc(&self, constraints: &[Constraint<S>], rc: &[S]) -> Vec<Constraint<S>> {
        let mut result = constraints.to_owned();
        for (res, rc) in result.iter_mut().zip(rc.iter()) {
            res[0].add_assign(rc);
        }
        result
    }

    pub fn matrix_mul(
        &self,
        constraints: &[Constraint<S>],
        matrix: &[Vec<S>],
    ) -> Vec<Constraint<S>> {
        let t = matrix.len();
        let mut result: Vec<Constraint<S>> = Vec::with_capacity(t);
        for row in matrix.iter() {
            let tmp = self.scalar_multiplication(constraints, row);
            result.push(tmp);
        }
        result
    }

    pub fn get_lhs(&self) -> &Vec<Constraint<S>> {
        &self.lhs
    }

    pub fn get_rhs(&self) -> &Vec<Constraint<S>> {
        &self.rhs
    }

    pub fn get_res(&self) -> &Vec<Constraint<S>> {
        &self.res
    }

    pub fn get_num_vars(&self) -> usize {
        self.vars
    }
}

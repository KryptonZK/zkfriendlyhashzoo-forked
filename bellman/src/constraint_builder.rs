use std::collections::HashMap;

use bellman_ce::{
    pairing::ff::Field, pairing::Engine, ConstraintSystem, LinearCombination, SynthesisError,
    Variable,
};

use crate::utils;

#[derive(Clone)]
pub struct ProofVar<E: Engine> {
    pub(crate) value: Option<E::Fr>,
    pub(crate) lc: LinearCombination<E>,
}

impl<E: Engine> ProofVar<E> {
    pub fn new_input<CS: ConstraintSystem<E>>(value: Option<E::Fr>, cs: &mut CS) -> Self {
        let var = cs
            .alloc_input(
                || "ProofVar Input",
                || value.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        ProofVar {
            value,
            lc: Self::lc_from_var(var),
        }
    }

    pub fn new_var<CS: ConstraintSystem<E>>(value: Option<E::Fr>, cs: &mut CS) -> Self {
        let var = cs
            .alloc(
                || "ProofVar",
                || value.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        ProofVar {
            value,
            lc: Self::lc_from_var(var),
        }
    }

    pub fn zero() -> Self {
        let value = Some(E::Fr::zero());
        let lc = LinearCombination::<E>::zero();
        ProofVar { value, lc }
    }

    pub fn lc_from_var(var: Variable) -> LinearCombination<E> {
        LinearCombination::zero() + var
    }

    pub fn get_value(&self) -> Option<E::Fr> {
        self.value
    }
}

pub struct ConstraintBuilder {}

impl ConstraintBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<E: Engine, CS: ConstraintSystem<E>>(
        val: Option<E::Fr>,
        is_input: bool,
        cs: &mut CS,
    ) -> ProofVar<E> {
        if is_input {
            ProofVar::new_input(val, cs)
        } else {
            ProofVar::new_var(val, cs)
        }
    }

    pub fn new_input<E: Engine, CS: ConstraintSystem<E>>(
        val: Option<E::Fr>,
        cs: &mut CS,
    ) -> ProofVar<E> {
        ProofVar::new_input(val, cs)
    }

    pub fn new_variable<E: Engine, CS: ConstraintSystem<E>>(
        val: Option<E::Fr>,
        cs: &mut CS,
    ) -> ProofVar<E> {
        ProofVar::new_var(val, cs)
    }

    pub fn enforce<E: Engine, CS: ConstraintSystem<E>>(
        rhs: &ProofVar<E>,
        lhs: &ProofVar<E>,
        res: &ProofVar<E>,
        cs: &mut CS,
    ) {
        cs.enforce(
            || "Enforce",
            |_| lhs.clone().lc,
            |_| rhs.clone().lc,
            |_| res.clone().lc,
        );
    }

    pub fn enforce_zero<E: Engine, CS: ConstraintSystem<E>>(
        lhs: &ProofVar<E>,
        rhs: &ProofVar<E>,
        cs: &mut CS,
    ) {
        cs.enforce(
            || "Enforce Zero",
            |_| lhs.clone().lc,
            |_| rhs.clone().lc,
            |lc| lc,
        );
    }

    pub fn enforce_is_zero<E: Engine, CS: ConstraintSystem<E>>(val: &ProofVar<E>, cs: &mut CS) {
        cs.enforce(
            || "Enforce is Zero",
            |_| val.clone().lc,
            |lc| lc + CS::one(),
            |lc| lc,
        );
    }

    // value != 0 => inverse must exist
    pub fn enforce_non_zero<E: Engine, CS: ConstraintSystem<E>>(
        val: &ProofVar<E>,
        cs: &mut CS,
    ) -> Result<ProofVar<E>, SynthesisError> {
        let inv = match val.value {
            Some(x) => {
                if x.is_zero() {
                    Err(SynthesisError::DivisionByZero)
                } else {
                    Ok(Some(x.inverse().unwrap()))
                }
            }
            None => Ok(None),
        }?;

        let inv_var = Self::new_variable(inv, cs);

        cs.enforce(
            || "Enforce is not Zero",
            |_| val.clone().lc,
            |_| inv_var.clone().lc,
            |lc| lc + CS::one(),
        );

        Ok(inv_var)
    }

    pub fn enforce_distinct<E: Engine, CS: ConstraintSystem<E>>(
        lhs: &ProofVar<E>,
        rhs: &ProofVar<E>,
        cs: &mut CS,
    ) -> Result<ProofVar<E>, SynthesisError> {
        let diff = Self::subtraction(lhs, rhs);
        Self::enforce_non_zero(&diff, cs)
    }

    // enforces low <= val <= high by enforcing (val - low) * (val - (low + 1))* ... (val -high) = 0
    pub fn enforce_range<E: Engine, CS: ConstraintSystem<E>>(
        val: &ProofVar<E>,
        low: usize,
        high: usize,
        cs: &mut CS,
    ) {
        let low_f = utils::from_u64(low as u64);
        if low == high {
            cs.enforce(
                || "Enforce is value",
                |_| val.clone().lc,
                |lc| lc + CS::one(),
                |lc| lc + (low_f, CS::one()),
            );
            return;
        }

        let mut acc = Self::sub_constant::<E, CS>(val, &low_f);
        for i in low + 1..high {
            let i_f = utils::from_u64(i as u64);
            let sub_var = Self::sub_constant::<E, CS>(val, &i_f);
            acc = Self::multiplication_new(&acc, &sub_var, cs);
        }
        let high_f = utils::from_u64(high as u64);
        let sub_var = Self::sub_constant::<E, CS>(val, &high_f);
        Self::enforce_zero(&acc, &sub_var, cs);
    }

    pub fn enforce_var<E: Engine, CS: ConstraintSystem<E>>(
        lhs: &ProofVar<E>,
        rhs: &ProofVar<E>,
        res: &Variable,
        cs: &mut CS,
    ) {
        cs.enforce(
            || "Enforce",
            |_| lhs.clone().lc,
            |_| rhs.clone().lc,
            |lc| lc + *res,
        );
    }

    pub fn enforce_linear<E: Engine, CS: ConstraintSystem<E>>(
        lhs: &ProofVar<E>,
        cs: &mut CS,
    ) -> ProofVar<E> {
        let var = Self::new_variable(lhs.value, cs);
        cs.enforce(
            || "Enforce Linear",
            |_| lhs.clone().lc,
            |lc| lc + CS::one(),
            |_| var.clone().lc,
        );
        var
    }

    pub fn enforce_final_linear<E: Engine, CS: ConstraintSystem<E>>(
        lhs: &ProofVar<E>,
        cs: &mut CS,
    ) -> ProofVar<E> {
        let var = Self::new_input(lhs.value, cs);
        cs.enforce(
            || "Enforce Final Linear",
            |_| lhs.clone().lc,
            |lc| lc + CS::one(),
            |_| var.clone().lc,
        );
        var
    }

    pub fn enforce_bool<E: Engine, CS: ConstraintSystem<E>>(
        var: &ProofVar<E>,
        cs: &mut CS,
    ) -> ProofVar<E> {
        let invert_var = Self::one_minus_rhs::<E, CS>(var);
        Self::enforce_zero(var, &invert_var, cs);
        invert_var
    }

    pub fn scalar_multiplication<E: Engine>(vars: &[ProofVar<E>], vector: &[E::Fr]) -> ProofVar<E> {
        let mut new_var = ProofVar::zero();
        for (con, vec) in vars.iter().zip(vector.iter()) {
            let tmp = Self::scale(con, vec);
            new_var = Self::addition(&new_var, &tmp);
        }
        new_var
    }

    pub fn scale<E: Engine>(var: &ProofVar<E>, scalar: &E::Fr) -> ProofVar<E> {
        let mut result = ProofVar::zero();
        for (variable, val) in var.lc.as_ref() {
            let mut tmp = *val;
            tmp.mul_assign(scalar);
            result.lc = result.lc + (tmp, *variable);
        }
        match var.value {
            Some(x) => {
                let mut tmp = x;
                tmp.mul_assign(scalar);
                result.value = Some(tmp);
            }
            None => result.value = None,
        }

        result
    }

    pub fn multiplication_new<E: Engine, CS: ConstraintSystem<E>>(
        lhs: &ProofVar<E>,
        rhs: &ProofVar<E>,
        cs: &mut CS,
    ) -> ProofVar<E> {
        let val = match (lhs.value, rhs.value) {
            (Some(a), Some(b)) => {
                let mut tmp = a;
                tmp.mul_assign(&b);
                Some(tmp)
            }
            (_, _) => None,
        };

        let var = Self::new_variable(val, cs);
        Self::enforce(rhs, lhs, &var, cs);
        var
    }

    pub fn multiplication<E: Engine, CS: ConstraintSystem<E>>(
        lhs: &ProofVar<E>,
        rhs: &ProofVar<E>,
        res: &ProofVar<E>,
        cs: &mut CS,
    ) {
        #[cfg(debug_assertions)]
        {
            if let (Some(a), Some(b), Some(c)) = (lhs.value, rhs.value, res.value) {
                let mut tmp = a;
                tmp.mul_assign(&b);
                assert_eq!(tmp, c);
            }
        }

        Self::enforce(lhs, rhs, res, cs);
    }

    pub fn addition<E: Engine>(lhs: &ProofVar<E>, rhs: &ProofVar<E>) -> ProofVar<E> {
        let mut coeffs: HashMap<Variable, E::Fr> = HashMap::new();
        for (var, val) in lhs.lc.as_ref() {
            debug_assert_eq!(coeffs.get_mut(var), None);
            coeffs.insert(*var, *val);
        }

        for (var, val) in rhs.lc.as_ref() {
            if let Some(x) = coeffs.get_mut(var) {
                x.add_assign(val);
            } else {
                coeffs.insert(*var, *val);
            }
        }

        let mut res = ProofVar::zero();
        for (var, val) in coeffs {
            res.lc = res.lc + (val, var);
        }

        let val = match (lhs.value, rhs.value) {
            (Some(a), Some(b)) => {
                let mut tmp = a;
                tmp.add_assign(&b);
                Some(tmp)
            }
            (_, _) => None,
        };
        res.value = val;
        res
    }

    pub fn subtraction<E: Engine>(lhs: &ProofVar<E>, rhs: &ProofVar<E>) -> ProofVar<E> {
        let mut coeffs: HashMap<Variable, E::Fr> = HashMap::new();
        for (var, val) in lhs.lc.as_ref() {
            debug_assert_eq!(coeffs.get_mut(var), None);
            coeffs.insert(*var, *val);
        }
        for (var, val) in rhs.lc.as_ref() {
            if let Some(x) = coeffs.get_mut(var) {
                x.sub_assign(val);
            } else {
                let mut tmp = *val;
                tmp.negate();
                coeffs.insert(*var, tmp);
            }
        }

        let mut res = ProofVar::zero();
        for (var, val) in coeffs {
            res.lc = res.lc + (val, var);
        }

        let val = match (lhs.value, rhs.value) {
            (Some(a), Some(b)) => {
                let mut tmp = a;
                tmp.sub_assign(&b);
                Some(tmp)
            }
            (_, _) => None,
        };
        res.value = val;
        res
    }

    pub fn one_minus_rhs<E: Engine, CS: ConstraintSystem<E>>(rhs: &ProofVar<E>) -> ProofVar<E> {
        let one = CS::one();
        let mut result = ProofVar::zero();
        let mut found = false;
        for (var, val) in rhs.lc.as_ref() {
            if !found && *var == one {
                let mut tmp = E::Fr::one();
                tmp.sub_assign(val);
                result.lc = result.lc + (tmp, *var);
                found = true;
            } else {
                result.lc = result.lc - (*val, *var);
            }
        }
        if !found {
            result.lc = result.lc + (E::Fr::one(), one);
        }

        let val = match rhs.value {
            Some(a) => {
                let mut tmp = E::Fr::one();
                tmp.sub_assign(&a);
                Some(tmp)
            }
            None => None,
        };
        result.value = val;
        result
    }

    pub fn sub_constant<E: Engine, CS: ConstraintSystem<E>>(
        variable: &ProofVar<E>,
        constant: &E::Fr,
    ) -> ProofVar<E> {
        let one = CS::one();
        let mut result = ProofVar::zero();
        let mut found = false;
        for (var, val) in variable.lc.as_ref() {
            if !found && *var == one {
                let mut tmp = *val;
                tmp.sub_assign(constant);
                result.lc = result.lc + (tmp, *var);
                found = true;
            } else {
                result.lc = result.lc + (*val, *var);
            }
        }
        if !found {
            result.lc = result.lc - (*constant, one);
        }

        match variable.value {
            Some(x) => {
                let mut tmp = x;
                tmp.sub_assign(constant);
                result.value = Some(tmp);
            }
            None => result.value = None,
        }

        result
    }

    pub fn add_constant<E: Engine, CS: ConstraintSystem<E>>(
        variable: &ProofVar<E>,
        constant: &E::Fr,
    ) -> ProofVar<E> {
        let one = CS::one();
        let mut result = ProofVar::zero();
        let mut found = false;
        for (var, val) in variable.lc.as_ref() {
            if !found && *var == one {
                let mut tmp = *val;
                tmp.add_assign(constant);
                result.lc = result.lc + (tmp, *var);
                found = true;
            } else {
                result.lc = result.lc + (*val, *var);
            }
        }
        if !found {
            result.lc = result.lc + (*constant, one);
        }

        match variable.value {
            Some(x) => {
                let mut tmp = x;
                tmp.add_assign(constant);
                result.value = Some(tmp);
            }
            None => result.value = None,
        }

        result
    }

    pub fn add_rc<E: Engine, CS: ConstraintSystem<E>>(
        constraints: &[ProofVar<E>],
        rc: &[E::Fr],
    ) -> Vec<ProofVar<E>> {
        constraints
            .iter()
            .zip(rc)
            .map(|(c, rc)| Self::add_constant::<E, CS>(c, rc))
            .collect()
    }

    pub fn matrix_mul<E: Engine>(
        constraints: &[ProofVar<E>],
        matrix: &[Vec<E::Fr>],
    ) -> Vec<ProofVar<E>> {
        let t = matrix.len();
        let mut result: Vec<ProofVar<E>> = Vec::with_capacity(t);
        for row in matrix.iter() {
            let tmp = Self::scalar_multiplication(constraints, row);
            result.push(tmp);
        }
        result
    }

    // Different arrangements of node for values of p
    // p=0 => [N, N1]
    // p=1 => [N1, N]
    pub fn mt_perm2<E: Engine, CS: ConstraintSystem<E>>(
        state: &mut [ProofVar<E>],
        pos: &Option<usize>,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        debug_assert!(state.len() >= 2);

        let pos_ = match pos {
            Some(x) => match x {
                0 => Some(E::Fr::zero()),
                1 => Some(E::Fr::one()),
                _ => return Err(SynthesisError::Unsatisfiable),
            },
            None => None,
        };
        let pos_node = Self::new_variable(pos_, cs);

        // enforce 0 or 1
        Self::enforce_bool(&pos_node, cs);

        let s0_p = Self::multiplication_new(&state[0], &pos_node, cs);
        let s1_p = Self::multiplication_new(&state[1], &pos_node, cs);

        // c0 = N - N * p + N1 * p
        let tmp = Self::addition(&state[0], &s1_p);
        state[0] = Self::subtraction(&tmp, &s0_p);

        // c1 = N1 - N1 * p + N * p
        let tmp = Self::addition(&state[1], &s0_p);
        state[1] = Self::subtraction(&tmp, &s1_p);
        Ok(())
    }

    // Different arrangements of node for values of p
    // p=0=00 => [N, N1, N2, N3]
    // p=1=10 => [N1, N, N2, N3]
    // p=2=01 => [N1, N2, N, N3]
    // p=3=11 => [N1, N2, N3, N]
    pub fn mt_perm4<E: Engine, CS: ConstraintSystem<E>>(
        state: &mut [ProofVar<E>],
        pos: &Option<usize>,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        debug_assert!(state.len() >= 4);

        let pos_ = match pos {
            Some(x) => match x {
                0 => [Some(E::Fr::zero()), Some(E::Fr::zero())],
                1 => [Some(E::Fr::one()), Some(E::Fr::zero())],
                2 => [Some(E::Fr::zero()), Some(E::Fr::one())],
                3 => [Some(E::Fr::one()), Some(E::Fr::one())],
                _ => return Err(SynthesisError::Unsatisfiable),
            },
            None => [None; 2],
        };

        let pos_nodes: Vec<ProofVar<E>> = pos_.iter().map(|p| Self::new_variable(*p, cs)).collect();

        // enforce 0 or 1
        let inv_pos_nodes: Vec<ProofVar<E>> = pos_nodes
            .iter()
            .map(|p| Self::enforce_bool(p, cs))
            .collect();

        // precompute bit products
        let p0_p1 = Self::multiplication_new(&pos_nodes[0], &pos_nodes[1], cs);
        let p0_n1 = Self::multiplication_new(&pos_nodes[0], &inv_pos_nodes[1], cs);
        let n0_p1 = Self::multiplication_new(&inv_pos_nodes[0], &pos_nodes[1], cs);
        let n0_n1 = Self::multiplication_new(&inv_pos_nodes[0], &inv_pos_nodes[1], cs);

        // new_state 0
        // c0 = (1 - p0) * (1 - p1) * N + p0 * N1 + (1 - p0) * p1 * N1
        let s0_n0_n1 = Self::multiplication_new(&n0_n1, &state[0], cs);
        let tmp = Self::addition(&pos_nodes[0], &n0_p1);
        let s1_tmp = Self::multiplication_new(&tmp, &state[1], cs);

        // new_state 1
        // c1 = (1 - p0) * (1 - p1) * N1 + (1 - p1) * p0 * N + (1 - p0) * p1 * N2 + p0 * p1 * N2
        let s1_n0_n1 = Self::multiplication_new(&n0_n1, &state[1], cs);
        let s0_p0_n1 = Self::multiplication_new(&p0_n1, &state[0], cs);
        let tmp = Self::addition(&n0_p1, &p0_p1);
        let s2_tmp = Self::multiplication_new(&tmp, &state[2], cs);

        // new_state 2
        // c2 = (1 - p1) * N2 + (1 - p0) * p1 * N + p0 * p1 * N3
        let s2_n1 = Self::multiplication_new(&inv_pos_nodes[1], &state[2], cs);
        let s0_n0_p1 = Self::multiplication_new(&n0_p1, &state[0], cs);
        let s3_p0_p1 = Self::multiplication_new(&p0_p1, &state[3], cs);

        // new_state 3
        // c3 = (1 - p1) * N3 + (1 - p0) * p1 * N3 + p1 * p0 * N
        let tmp = Self::addition(&inv_pos_nodes[1], &n0_p1);
        let s3_tmp = Self::multiplication_new(&tmp, &state[3], cs);
        let s0_p0_p1 = Self::multiplication_new(&p0_p1, &state[0], cs);

        // output
        state[0] = Self::addition(&s0_n0_n1, &s1_tmp);

        state[1] = Self::addition(&s1_n0_n1, &s0_p0_n1);
        state[1] = Self::addition(&state[1], &s2_tmp);

        state[2] = Self::addition(&s2_n1, &s0_n0_p1);
        state[2] = Self::addition(&state[2], &s3_p0_p1);

        state[3] = Self::addition(&s0_p0_p1, &s3_tmp);

        Ok(())
    }

    // Different arrangements of node for values of p
    // p=0=000 => [N, N1, N2, N3, N4, N5, N6, N7]
    // p=1=100 => [N1, N, N2, N3, N4, N5, N6, N7]
    // p=2=010 => [N1, N2, N, N3, N4, N5, N6, N7]
    // p=3=110 => [N1, N2, N3, N, N4, N5, N6, N7]
    // p=4=001 => [N1, N2, N3, N4, N, N5, N6, N7]
    // p=5=101 => [N1, N2, N3, N4, N5, N, N6, N7]
    // p=6=011 => [N1, N2, N3, N4, N5, N6, N, N7]
    // p=7=111 => [N1, N2, N3, N4, N5, N6, N7, N]
    pub fn mt_perm8<E: Engine, CS: ConstraintSystem<E>>(
        state: &mut [ProofVar<E>],
        pos: &Option<usize>,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        debug_assert!(state.len() >= 8);

        let pos_ = match pos {
            Some(x) => match x {
                0 => [
                    Some(E::Fr::zero()),
                    Some(E::Fr::zero()),
                    Some(E::Fr::zero()),
                ],
                1 => [Some(E::Fr::one()), Some(E::Fr::zero()), Some(E::Fr::zero())],
                2 => [Some(E::Fr::zero()), Some(E::Fr::one()), Some(E::Fr::zero())],
                3 => [Some(E::Fr::one()), Some(E::Fr::one()), Some(E::Fr::zero())],
                4 => [Some(E::Fr::zero()), Some(E::Fr::zero()), Some(E::Fr::one())],
                5 => [Some(E::Fr::one()), Some(E::Fr::zero()), Some(E::Fr::one())],
                6 => [Some(E::Fr::zero()), Some(E::Fr::one()), Some(E::Fr::one())],
                7 => [Some(E::Fr::one()), Some(E::Fr::one()), Some(E::Fr::one())],
                _ => return Err(SynthesisError::Unsatisfiable),
            },
            None => [None; 3],
        };

        let pos_nodes: Vec<ProofVar<E>> = pos_.iter().map(|p| Self::new_variable(*p, cs)).collect();

        // enforce 0 or 1
        let inv_pos_nodes: Vec<ProofVar<E>> = pos_nodes
            .iter()
            .map(|p| Self::enforce_bool(p, cs))
            .collect();

        // precompute bit products
        let p0_p1 = Self::multiplication_new(&pos_nodes[0], &pos_nodes[1], cs);
        let n1_p2 = Self::multiplication_new(&inv_pos_nodes[1], &pos_nodes[2], cs);
        let p0_p2 = Self::multiplication_new(&pos_nodes[0], &pos_nodes[2], cs);
        let p0_n1 = Self::multiplication_new(&pos_nodes[0], &inv_pos_nodes[1], cs);
        let p1_p2 = Self::multiplication_new(&pos_nodes[1], &pos_nodes[2], cs);
        let n1_n2 = Self::multiplication_new(&inv_pos_nodes[1], &inv_pos_nodes[2], cs);
        let p1_n2 = Self::multiplication_new(&pos_nodes[1], &inv_pos_nodes[2], cs);

        let n0_n1_n2 = Self::multiplication_new(&inv_pos_nodes[0], &n1_n2, cs);
        let n0_n1_p2 = Self::multiplication_new(&inv_pos_nodes[0], &n1_p2, cs);
        let p0_n1_n2 = Self::multiplication_new(&pos_nodes[0], &n1_n2, cs);
        let n0_p1_n2 = Self::multiplication_new(&inv_pos_nodes[0], &p1_n2, cs);
        let p0_p1_n2 = Self::multiplication_new(&inv_pos_nodes[2], &p0_p1, cs);
        let p0_n1_p2 = Self::multiplication_new(&inv_pos_nodes[1], &p0_p2, cs);
        let n0_p1_p2 = Self::multiplication_new(&inv_pos_nodes[0], &p1_p2, cs);
        let p0_p1_p2 = Self::multiplication_new(&pos_nodes[0], &p1_p2, cs);

        // new_state 0
        // c0 = N * (1 - p0) * (1 - p1) * (1 - p2) + N1 * p2 + N1 * (1 - p2) * p1 + N1 * (1 - p2) * (1 - p1) * p0
        let s0_n0_n1_n2 = Self::multiplication_new(&n0_n1_n2, &state[0], cs);
        let tmp = Self::addition(&pos_nodes[2], &p1_n2);
        let tmp = Self::addition(&tmp, &p0_n1_n2);
        let c0_s1_tmp = Self::multiplication_new(&tmp, &state[1], cs);

        // new_state 1
        // c1 = N1 * (1 - p0) * (1 - p1) * (1 - p2) + N * p0 * (1 - p1) * (1 - p2) + N2 * p1 + N2 * (1 - p1) * p2
        let s1_n0_n1_n2 = Self::multiplication_new(&n0_n1_n2, &state[1], cs);
        let s0_p0_n1_n2 = Self::multiplication_new(&p0_n1_n2, &state[0], cs);
        let tmp = Self::addition(&pos_nodes[1], &n1_p2);
        let c1_s2_tmp = Self::multiplication_new(&tmp, &state[2], cs);

        // new_state 2
        // c2 = N2 * (1 - p1) * (1 - p2) + N * (1 - p0) * (1 - p2) * p1 + N3 * p2 + N3 * (1 - p2) * p0 * p1
        let s2_n1_n2 = Self::multiplication_new(&n1_n2, &state[2], cs);
        let s0_n0_p1_n2 = Self::multiplication_new(&n0_p1_n2, &state[0], cs);
        let tmp = Self::addition(&pos_nodes[2], &p0_p1_n2);
        let c2_s3_tmp = Self::multiplication_new(&tmp, &state[3], cs);

        // new_state 3
        // c3 =  N3 * (1 - p1) * (1 - p2) + N3 * (1 - p0) * (1 - p2) * p1 + N * p0 * p1 * (1 - p2) + N4 * p2
        let tmp = Self::addition(&n1_n2, &n0_p1_n2);
        let c3_s3_tmp = Self::multiplication_new(&tmp, &state[3], cs);
        let s0_p0_p1_n2 = Self::multiplication_new(&p0_p1_n2, &state[0], cs);
        let s4_p2 = Self::multiplication_new(&pos_nodes[2], &state[4], cs);

        // new_state 4
        // c4 = N4 * (1 - p2) + N * (1 - p0) * (1 - p1) * p2 + N5 * p0 * p2 + N5 * (1 - p0) * p1 * p2
        let s4_n2 = Self::multiplication_new(&inv_pos_nodes[2], &state[4], cs);
        let s0_n0_n1_p2 = Self::multiplication_new(&n0_n1_p2, &state[0], cs);
        let tmp = Self::addition(&p0_p2, &n0_p1_p2);
        let c4_s5_tmp = Self::multiplication_new(&tmp, &state[5], cs);

        // new_state 5
        // c5 = N5 * (1 - p0) * (1 - p1) * p2 + N5 * (1 - p2) + N * p0 * (1 - p1) * p2 + N6 * p1 * p2
        let tmp = Self::addition(&n0_n1_p2, &inv_pos_nodes[2]);
        let c5_s5_tmp = Self::multiplication_new(&tmp, &state[5], cs);
        let s0_p0_n1_p2 = Self::multiplication_new(&p0_n1_p2, &state[0], cs);
        let s6_p1_p2 = Self::multiplication_new(&p1_p2, &state[6], cs);

        // new_state 6
        // c6 = N6 * (1 - p1) * p2 + N6 * (1 - p2) + N * (1 - p0) * p1 * p2 + N7 * p0 * p1 * p2
        let tmp = Self::addition(&n1_p2, &inv_pos_nodes[2]);
        let c6_s6_tmp = Self::multiplication_new(&tmp, &state[6], cs);
        let s0_n0_p1_p2 = Self::multiplication_new(&n0_p1_p2, &state[0], cs);
        let s7_p0_p1_p2 = Self::multiplication_new(&p0_p1_p2, &state[7], cs);

        // new_state 7
        // c7 = N * p0 * p1 * p2 + N7 (1 -p0) + N7 (1 - p1) * p0 + N7 * (1 - p2) * p0 * p1
        let s0_p0_p1_p2 = Self::multiplication_new(&p0_p1_p2, &state[0], cs);
        let tmp = Self::addition(&inv_pos_nodes[0], &p0_n1);
        let tmp = Self::addition(&tmp, &p0_p1_n2);
        let c7_s7_tmp = Self::multiplication_new(&tmp, &state[7], cs);

        // output
        state[0] = Self::addition(&s0_n0_n1_n2, &c0_s1_tmp);

        state[1] = Self::addition(&s1_n0_n1_n2, &s0_p0_n1_n2);
        state[1] = Self::addition(&state[1], &c1_s2_tmp);

        state[2] = Self::addition(&s2_n1_n2, &s0_n0_p1_n2);
        state[2] = Self::addition(&state[2], &c2_s3_tmp);

        state[3] = Self::addition(&s0_p0_p1_n2, &s4_p2);
        state[3] = Self::addition(&state[3], &c3_s3_tmp);

        state[4] = Self::addition(&s4_n2, &s0_n0_n1_p2);
        state[4] = Self::addition(&state[4], &c4_s5_tmp);

        state[5] = Self::addition(&s0_p0_n1_p2, &s6_p1_p2);
        state[5] = Self::addition(&state[5], &c5_s5_tmp);

        state[6] = Self::addition(&s0_n0_p1_p2, &s7_p0_p1_p2);
        state[6] = Self::addition(&state[6], &c6_s6_tmp);

        state[7] = Self::addition(&s0_p0_p1_p2, &c7_s7_tmp);

        Ok(())
    }
}

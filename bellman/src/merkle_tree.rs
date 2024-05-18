use std::{
    fmt::{self, Display, Formatter},
    usize,
};

use bellman_ce::pairing::ff::PrimeField;

use crate::circuits::Permutation;

#[derive(Debug, Clone)]
pub struct NotFoundError;

impl Display for NotFoundError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Element not found in Tree")
    }
}

#[derive(Clone, Debug)]
pub struct Node<S: PrimeField> {
    pub(crate) digest: S,
    pub(crate) children: Option<Vec<Node<S>>>,
}

impl<S: PrimeField> Node<S> {
    pub fn new(dig: &S, children: Vec<Node<S>>) -> Self {
        Node {
            digest: dig.to_owned(),
            children: Some(children),
        }
    }

    pub fn new_leaf(dig: &S) -> Self {
        Node {
            digest: dig.to_owned(),
            children: None,
        }
    }

    pub fn get_dig(&self) -> S {
        self.digest.to_owned()
    }
}

#[derive(Clone, Debug)]
pub struct ProofNode<S: PrimeField> {
    pub(crate) digests: Vec<S>,
    pub(crate) position: usize,
}

impl<S: PrimeField> ProofNode<S> {
    pub fn new(digests: &[S], position: usize) -> Self {
        ProofNode {
            digests: digests.to_owned(),
            position,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MerkleTree<S: PrimeField, P: Permutation<S>> {
    arity: usize,
    t: usize,
    perm: P,
    root: Option<Node<S>>,
}

impl<S: PrimeField, P: Permutation<S>> MerkleTree<S, P> {
    pub fn new(perm: P) -> Self {
        let t = perm.get_t();
        let arity = Self::highest_power_of_2(t - 1);

        MerkleTree {
            arity,
            t,
            perm,
            root: None,
        }
    }

    pub fn get_root(&self) -> Result<S, NotFoundError> {
        match &self.root {
            Some(x) => Ok(x.get_dig()),
            None => Err(NotFoundError),
        }
    }

    fn highest_power_of_2(n: usize) -> usize {
        debug_assert!(n >= 1);
        let mut res = 1;
        let mut curr = 1;
        // try powers, starting from 2^1
        loop {
            curr <<= 1;
            if curr > n {
                break;
            }
            res = curr;
        }
        res
    }

    pub fn get_arity(&self) -> usize {
        self.arity
    }

    fn round_up_pow_n(input: usize, n: usize) -> usize {
        debug_assert!(n >= 1);
        let mut res = 1;
        // try powers, starting from n
        loop {
            res *= n;
            if res >= input {
                break;
            }
        }
        res
    }

    pub fn accumulate(&mut self, set: &[S]) {
        let set_size = set.len();
        let mut bound = Self::round_up_pow_n(set_size, self.arity);
        loop {
            if bound >= self.arity {
                break;
            }
            bound *= 2;
        }
        let mut nodes: Vec<Node<S>> = Vec::with_capacity(bound);
        for s in set {
            nodes.push(Node::new_leaf(s));
        }
        // pad
        for _ in nodes.len()..bound {
            nodes.push(nodes[set_size - 1].clone());
        }

        while nodes.len() > 1 {
            let new_len = nodes.len() / self.arity;
            let mut new_nodes: Vec<Node<S>> = Vec::with_capacity(new_len);
            for i in (0..nodes.len()).step_by(self.arity) {
                // digest
                let children: Vec<Node<S>> =
                    (i..i + self.arity).map(|j| nodes[j].clone()).collect();
                let mut el: Vec<S> = children.iter().map(|c| c.get_dig()).collect();
                el.resize(self.t, S::zero());
                let dig = self.perm.permutation(&el)[0];
                new_nodes.push(Node::new(&dig, children));
            }
            nodes = new_nodes;
        }
        self.root = Some(nodes[0].clone());
    }

    fn in_set(node: &Node<S>, element: &S, wit: &mut Vec<ProofNode<S>>) -> bool {
        let children = node.children.as_ref();
        match children {
            Some(nodes) => {
                let len = nodes.len();
                let mut r = false;
                let mut new_wit: Vec<S> = Vec::with_capacity(len - 1);
                let mut index: usize = 0;
                for (i, n) in nodes.iter().enumerate() {
                    if !r {
                        r = Self::in_set(n, element, wit);
                        if r {
                            index = i;
                        } else if i != len {
                            new_wit.push(n.digest.to_owned());
                        }
                    } else if i != len {
                        new_wit.push(n.digest.to_owned());
                    }
                }
                match r {
                    false => false,
                    true => {
                        wit.push(ProofNode::new(&new_wit, index));
                        true
                    }
                }
            }

            None => node.digest == *element,
        }
    }

    pub fn create_witness(&self, value: &S) -> Result<Vec<ProofNode<S>>, NotFoundError> {
        let mut res = Vec::new();
        match &self.root {
            None => Err(NotFoundError),
            Some(x) => {
                if !Self::in_set(x, value, &mut res) {
                    Err(NotFoundError)
                } else {
                    Ok(res)
                }
            }
        }
    }

    pub fn verify(&self, value: &S, wit: &[ProofNode<S>]) -> bool {
        match &self.root {
            None => false,
            Some(x) => {
                let mut dig = *value;
                for proof_node in wit {
                    let mut input: Vec<S> = vec![S::zero(); self.t];
                    for (i, n) in proof_node.digests.iter().enumerate() {
                        if i < proof_node.position {
                            input[i] = n.to_owned();
                        } else {
                            input[i + 1] = n.to_owned();
                        }
                    }
                    input[proof_node.position] = dig;
                    dig = self.perm.permutation(&input)[0];
                }
                dig == x.digest
            }
        }
    }
}

#[cfg(test)]
mod merkletree_tests {

    use rand::{
        distributions::{IndependentSample, Range},
        thread_rng,
    };

    use crate::{
        circuits::Permutation,
        gmimc::{gmimc::Gmimc, gmimc_instance_bls12::*, gmimc_instance_bn256::*},
        grendel::{grendel::Grendel, grendel_instance_bls12::*, grendel_instance_bn256::*},
        griffin::{griffin::Griffin, griffin_instances::*},
        neptune::{neptune::Neptune, neptune_instances::*},
        poseidon::{poseidon::Poseidon, poseidon_instance_bls12::*, poseidon_instance_bn256::*},
        rescue::{rescue::Rescue, rescue_instance_bls12::*, rescue_instance_bn256::*},
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 2;
    static LOG_SET_SIZE: usize = 10;

    fn merkletree<S: PrimeField, P: Permutation<S>>(perm: P) {
        let t = perm.get_t();
        let mut mt = MerkleTree::new(perm);
        let arity = mt.get_arity();
        let set_size = 1 << LOG_SET_SIZE;
        let mut rng = thread_rng();
        let dist: Range<usize> = Range::new(0, set_size);
        assert!(t > arity);
        assert_eq!(arity & (arity - 1), 0); // power of 2

        for _ in 0..TESTRUNS {
            let set: Vec<S> = (0..set_size)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();
            let index = dist.ind_sample(&mut rng);
            mt.accumulate(&set);
            let wit = mt.create_witness(&set[index]).unwrap();
            let res = mt.verify(&set[index], &wit);
            assert!(res);
        }
    }

    #[test]
    fn rescue_bls12_t3_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BLS_3_PARAMS));
    }

    #[test]
    fn rescue_bls12_t4_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BLS_4_PARAMS));
    }

    #[test]
    fn rescue_bls12_t5_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BLS_5_PARAMS));
    }

    #[test]
    fn rescue_bls12_t8_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BLS_8_PARAMS));
    }

    #[test]
    fn rescue_bls12_t9_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BLS_9_PARAMS));
    }

    #[test]
    fn rescue_bls12_t12_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BLS_12_PARAMS));
    }

    #[test]
    fn rescue_bn256_t3_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BN_3_PARAMS));
    }

    #[test]
    fn rescue_bn256_t4_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BN_4_PARAMS));
    }

    #[test]
    fn rescue_bn256_t5_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BN_5_PARAMS));
    }

    #[test]
    fn rescue_bn256_t8_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BN_8_PARAMS));
    }

    #[test]
    fn rescue_bn256_t9_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BN_9_PARAMS));
    }

    #[test]
    fn rescue_bn256_t12_merkletree_test() {
        merkletree(Rescue::new(&RESCUE_BN_12_PARAMS));
    }

    #[test]
    fn poseidon_bls12_t3_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BLS_3_PARAMS));
    }

    #[test]
    fn poseidon_bls12_t4_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BLS_4_PARAMS));
    }

    #[test]
    fn poseidon_bls12_t5_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BLS_5_PARAMS));
    }

    #[test]
    fn poseidon_bls12_t8_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BLS_8_PARAMS));
    }

    #[test]
    fn poseidon_bls12_t9_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BLS_9_PARAMS));
    }

    #[test]
    fn poseidon_bls12_t12_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BLS_12_PARAMS));
    }

    #[test]
    fn poseidon_bn256_t3_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BN_3_PARAMS));
    }

    #[test]
    fn poseidon_bn256_t4_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BN_4_PARAMS));
    }

    #[test]
    fn poseidon_bn256_t5_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BN_5_PARAMS));
    }

    #[test]
    fn poseidon_bn256_t8_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BN_8_PARAMS));
    }

    #[test]
    fn poseidon_bn256_t9_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BN_9_PARAMS));
    }

    #[test]
    fn poseidon_bn256_t12_merkletree_test() {
        merkletree(Poseidon::new(&POSEIDON_BN_12_PARAMS));
    }

    #[test]
    fn griffin_bls12_t3_merkletree_test() {
        merkletree(Griffin::new(&GRIFFIN_BLS_3_PARAMS));
    }

    #[test]
    fn griffin_bls12_t4_merkletree_test() {
        merkletree(Griffin::new(&GRIFFIN_BLS_4_PARAMS));
    }

    #[test]
    fn griffin_bls12_t8_merkletree_test() {
        merkletree(Griffin::new(&GRIFFIN_BLS_8_PARAMS));
    }

    #[test]
    fn griffin_bls12_t12_merkletree_test() {
        merkletree(Griffin::new(&GRIFFIN_BLS_12_PARAMS));
    }

    #[test]
    fn griffin_bn256_t3_merkletree_test() {
        merkletree(Griffin::new(&GRIFFIN_BN_3_PARAMS));
    }

    #[test]
    fn griffin_bn256_t4_merkletree_test() {
        merkletree(Griffin::new(&GRIFFIN_BN_4_PARAMS));
    }

    #[test]
    fn griffin_bn256_t8_merkletree_test() {
        merkletree(Griffin::new(&GRIFFIN_BN_8_PARAMS));
    }

    #[test]
    fn griffin_bn256_t12_merkletree_test() {
        merkletree(Griffin::new(&GRIFFIN_BN_12_PARAMS));
    }

    #[test]
    fn grendel_bls12_t3_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BLS_3_PARAMS));
    }

    #[test]
    fn grendel_bls12_t4_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BLS_4_PARAMS));
    }

    #[test]
    fn grendel_bls12_t5_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BLS_5_PARAMS));
    }

    #[test]
    fn grendel_bls12_t8_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BLS_8_PARAMS));
    }

    #[test]
    fn grendel_bls12_t9_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BLS_9_PARAMS));
    }

    #[test]
    fn grendel_bls12_t12_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BLS_12_PARAMS));
    }

    #[test]
    fn grendel_bn256_t3_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BN_3_PARAMS));
    }

    #[test]
    fn grendel_bn256_t4_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BN_4_PARAMS));
    }

    #[test]
    fn grendel_bn256_t5_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BN_5_PARAMS));
    }

    #[test]
    fn grendel_bn256_t8_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BN_8_PARAMS));
    }

    #[test]
    fn grendel_bn256_t9_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BN_9_PARAMS));
    }

    #[test]
    fn grendel_bn256_t12_merkletree_test() {
        merkletree(Grendel::new(&GRENDEL_BN_12_PARAMS));
    }

    #[test]
    fn gmimc_bls12_t3_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BLS_3_PARAMS));
    }

    #[test]
    fn gmimc_bls12_t4_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BLS_4_PARAMS));
    }

    #[test]
    fn gmimc_bls12_t5_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BLS_5_PARAMS));
    }

    #[test]
    fn gmimc_bls12_t8_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BLS_8_PARAMS));
    }

    #[test]
    fn gmimc_bls12_t9_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BLS_9_PARAMS));
    }

    #[test]
    fn gmimc_bls12_t12_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BLS_12_PARAMS));
    }

    #[test]
    fn gmimc_bn256_t3_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BN_3_PARAMS));
    }

    #[test]
    fn gmimc_bn256_t4_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BN_4_PARAMS));
    }

    #[test]
    fn gmimc_bn256_t5_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BN_5_PARAMS));
    }

    #[test]
    fn gmimc_bn256_t8_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BN_8_PARAMS));
    }

    #[test]
    fn gmimc_bn256_t9_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BN_9_PARAMS));
    }

    #[test]
    fn gmimc_bn256_t12_merkletree_test() {
        merkletree(Gmimc::new(&GMIMC_BN_12_PARAMS));
    }

    #[test]
    fn neptune_bls12_t4_merkletree_test() {
        merkletree(Neptune::new(&NEPTUNE_BLS_4_PARAMS));
    }

    #[test]
    fn neptune_bls12_t8_merkletree_test() {
        merkletree(Neptune::new(&NEPTUNE_BLS_8_PARAMS));
    }

    #[test]
    fn neptune_bls12_t12_merkletree_test() {
        merkletree(Neptune::new(&NEPTUNE_BLS_12_PARAMS));
    }

    #[test]
    fn neptune_bn256_t4_merkletree_test() {
        merkletree(Neptune::new(&NEPTUNE_BN_4_PARAMS));
    }

    #[test]
    fn neptune_bn256_t8_merkletree_test() {
        merkletree(Neptune::new(&NEPTUNE_BN_8_PARAMS));
    }

    #[test]
    fn neptune_bn256_t12_merkletree_test() {
        merkletree(Neptune::new(&NEPTUNE_BN_12_PARAMS));
    }
}

use alloc::vec::Vec;
use core::fmt::Debug;

use p3_field::{ExtensionField, Field};
use p3_matrix::Matrix;

/// A set of parameters defining a specific instance of the FRI protocol.
#[derive(Debug)]
pub struct FriParameters<M> {
    pub log_blowup: usize,
    // TODO: This parameter and FRI early stopping are not yet implemented in `CirclePcs`.
    pub log_final_poly_len: usize,
    pub num_queries: usize,
    pub proof_of_work_bits: usize,
    pub mmcs: M,
}

impl<M> FriParameters<M> {
    pub const fn blowup(&self) -> usize {
        1 << self.log_blowup
    }

    pub const fn final_poly_len(&self) -> usize {
        1 << self.log_final_poly_len
    }

    /// Returns the soundness bits of this FRI instance based on the
    /// [ethSTARK](https://eprint.iacr.org/2021/582) conjecture.
    ///
    /// Certain users may instead want to look at proven soundness, a more complex calculation which
    /// isn't currently supported by this crate.
    pub const fn conjectured_soundness_bits(&self) -> usize {
        self.log_blowup * self.num_queries + self.proof_of_work_bits
    }
}

/// Whereas `FriParameters` encompasses parameters the end user can set, `FriFoldingStrategy` is
/// set by the PCS calling FRI, and abstracts over implementation details of the PCS.
pub trait FriFoldingStrategy<F: Field, EF: ExtensionField<F>> {
    type InputProof;
    type InputError: Debug;

    /// We can ask FRI to sample extra query bits (LSB) for our own purposes.
    /// They will be passed to our callbacks, but ignored (shifted off) by FRI.
    fn extra_query_index_bits(&self) -> usize;

    /// Fold a row, returning a single column.
    /// Right now the input row will always be 2 columns wide,
    /// but we may support higher folding arity in the future.
    fn fold_row(
        &self,
        index: usize,
        log_height: usize,
        beta: EF,
        evals: impl Iterator<Item = EF>,
    ) -> EF;

    /// Same as applying fold_row to every row, possibly faster.
    fn fold_matrix<M: Matrix<EF>>(&self, beta: EF, m: M) -> Vec<EF>;
}

/// Creates a minimal set of `FriParameters` for testing purposes.
/// These parameters are designed to reduce computational cost during tests.
pub const fn create_test_fri_params<Mmcs>(
    mmcs: Mmcs,
    log_final_poly_len: usize,
) -> FriParameters<Mmcs> {
    FriParameters {
        log_blowup: 2,
        log_final_poly_len,
        num_queries: 2,
        proof_of_work_bits: 1,
        mmcs,
    }
}

/// Creates a minimal set of `FriParameters` for testing purposes, with zk enabled.
/// These parameters are designed to reduce computational cost during tests.
pub const fn create_test_fri_params_zk<Mmcs>(mmcs: Mmcs) -> FriParameters<Mmcs> {
    FriParameters {
        log_blowup: 2,
        log_final_poly_len: 0,
        num_queries: 2,
        proof_of_work_bits: 1,
        mmcs,
    }
}

/// Creates a set of `FriParameters` suitable for benchmarking.
/// These parameters represent typical settings used in production-like scenarios.
pub const fn create_benchmark_fri_params<Mmcs>(mmcs: Mmcs) -> FriParameters<Mmcs> {
    FriParameters {
        log_blowup: 1,
        log_final_poly_len: 0,
        num_queries: 100,
        proof_of_work_bits: 16,
        mmcs,
    }
}

/// Creates a set of `FriParameters` suitable for benchmarking with zk enabled.
/// These parameters represent typical settings used in production-like scenarios.
pub fn create_benchmark_fri_params_zk<Mmcs>(mmcs: Mmcs) -> FriParameters<Mmcs> {
    FriParameters {
        log_blowup: 2,
        log_final_poly_len: 0,
        num_queries: 100,
        proof_of_work_bits: 16,
        mmcs,
    }
}

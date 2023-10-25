use std::borrow::Cow;

use raft::v1::{ConfChange, ConfChangeSingle, ConfChangeType, ConfChangeV2};

pub mod raft {
    pub mod v1 {
        include!("protos/mod.rs");

        pub use raft::*;

        impl Snapshot {
            /// For a given snapshot, determine if it's empty or not.
            pub fn is_empty(&self) -> bool {
                self.get_metadata().index == 0
            }
        }
    }
}

pub mod util {
    use crate::raft::v1::ConfState;

    impl<Iter1, Iter2> From<(Iter1, Iter2)> for ConfState
    where
        Iter1: IntoIterator<Item = u64>,
        Iter2: IntoIterator<Item = u64>,
    {
        fn from((voters, learners): (Iter1, Iter2)) -> Self {
            let mut conf_state = ConfState::default();
            conf_state.mut_voters().extend(voters);
            conf_state.mut_learners().extend(learners);
            conf_state
        }
    }
}

pub fn new_conf_change_single(node_id: u64, ty: ConfChangeType) -> ConfChangeSingle {
    let mut single = ConfChangeSingle::default();
    single.node_id = node_id;
    single.set_change_type(ty);
    single
}

/// Abstracts over ConfChangeV2 and (legacy) ConfChange to allow
/// treating them in a unified manner.
pub trait ConfChangeI {
    /// Converts conf change to `ConfChangeV2`.
    fn into_v2(self) -> ConfChangeV2;

    /// Gets conf change as `ConfChangeV2`.
    fn as_v2(&self) -> Cow<ConfChangeV2>;

    /// Converts conf change to `ConfChange`.
    ///
    /// `ConfChangeV2` can't be changed back to `ConfChange`.
    fn as_v1(&self) -> Option<&ConfChange>;
}

impl ConfChangeI for ConfChange {
    #[inline]
    fn into_v2(mut self) -> ConfChangeV2 {
        let mut cc = ConfChangeV2::default();
        let single = new_conf_change_single(self.node_id, self.get_change_type());
        cc.mut_changes().push(single);
        cc.set_context(self.take_context());
        cc
    }

    #[inline]
    fn as_v2(&self) -> Cow<ConfChangeV2> {
        Cow::Owned(self.clone().into_v2())
    }

    #[inline]
    fn as_v1(&self) -> Option<&ConfChange> {
        Some(self)
    }
}

impl ConfChangeI for ConfChangeV2 {
    #[inline]
    fn into_v2(self) -> ConfChangeV2 {
        self
    }

    #[inline]
    fn as_v2(&self) -> Cow<ConfChangeV2> {
        Cow::Borrowed(self)
    }

    #[inline]
    fn as_v1(&self) -> Option<&ConfChange> {
        None
    }
}

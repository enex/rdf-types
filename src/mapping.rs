/// Handles mapping from an external type to another one possibly used internal
pub trait IdMap<V, ID>{
    fn id_for(&self, v: V) -> Option<ID>;
    fn for_id(&self, id: ID) -> Option<V>;
}

/// Handles mapping from an external type to another one possibly used internal
/// entries are created if they do not exsist
pub trait MutIdMap<V, ID>{
    fn mut_id_for(&mut self, v: V) -> ID;
    fn mut_for_id(&mut self, id: ID) -> V;
}

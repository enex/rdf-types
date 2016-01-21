mod triple;
mod mapping;
mod reflect;
pub mod ns;
pub use triple::*;
pub use mapping::*;
pub use reflect::*;


/// Trait that must be implemented by all graph representations
pub trait Graph<S, P, O>{
    //TODO: add fu
    fn all();

    /// Returns an iterator over all the Triples that match the triple pattern.
    fn find(&self, Triple<S, P, O>);

    /// Answer true iff the graph contains a triple matching (s, p, o).
    fn contains(&self, Triple<S, P, O>) -> bool;

    //TODO: add depends on

    /// the number of triples
    fn len() -> u64;

    fn add_to<G: ModifyGraph<S, P, O>>(&self, _: G){
        //TODO: add all triples into the graph
    }
}

/// a trait wrapping funnctions to modify a graph or generate a graph
pub trait ModifyGraph<S, P, O>{
    fn add(&mut self, Triple<S, P, O>);
    fn remove(&mut self, Triple<S, P, O>);

    fn add_from<G: Graph<S, P, O>>(&mut self, _: G){
        //TODO: add everything from
    }
}

/// A graph that can be read and modified
pub trait MutGraph<S, P, O>:
    Graph<S, P, O> +
    ModifyGraph<S, P, O>{

    /// remove all triples from the graph
    /// the default gets all triples and then calls remove for each one
    fn clear(&mut self){
        //TODO: implement it
    }
}

#[test]
fn it_works() {
}

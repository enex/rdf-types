use Graph;
use std::marker::PhantomData;

/// Struct to simplify interaction with a graph by providing nodes representing
/// nodes in the graph that can be modified and queried
pub struct Reflection<G: Graph<S, P, O>, S, P, O>{
    id: S,
    graph: G,
    predicate: PhantomData<P>,
    object: PhantomData<O>
}

impl<G: Graph<S, P, O>, S, P, O> Reflection<G, S, P, O>{
    pub fn new(graph: G, id: S) -> Reflection<G, S, P, O>{
        Reflection{
            id: id,
            graph: graph,
            predicate: PhantomData,
            object: PhantomData
        }
    }

    /*
    pub fn out(&self, p: P) -> MultiReflection<G, S, P, O>{
        Reflection::new(self.graph, )
    }*/
}

// type that signifies that a property should be interpreted as reverse
pub struct Reverse<T>(T);

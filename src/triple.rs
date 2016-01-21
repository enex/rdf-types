use std::convert::From;

// a generic triple that is used everywhere and might implement other traits
// if the types match
#[derive(Clone, Debug)]
pub struct Triple<S,P=S,O=S>{
    pub subject: S,
    pub predicate: P,
    pub object: O
}

impl<S, P, O> From<(S, P, O)> for Triple<S, P, O>{
    fn from(t: (S, P, O)) -> Triple<S, P, O>{
        Triple{
            subject: t.0,
            predicate: t.1,
            object: t.2
        }
    }
}

impl<I: Clone> From<[I; 3]> for Triple<I>{
    fn from(t: [I; 3]) -> Triple<I>{
        Triple{
            subject: t[0].clone(),
            predicate: t[1].clone(),
            object: t[2].clone()
        }
    }
}

impl<S,P,O> Triple<S,P,O>{
    pub fn new(s: S, p: P, o: O) -> Triple<S,P,O>{
        Triple{
            subject: s,
            predicate: p,
            object: o
        }
    }
}

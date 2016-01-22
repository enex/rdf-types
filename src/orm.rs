macro_rules! rdf{
    (
        $(
            class $name:ident = $id:expr {
                $(
                    $prop_name:ident : $prop_type:ty = $prop_id:expr
                ),*
            }
        )*
    ) => (
        $(
            pub struct $name{
                pub id: String
            }
            impl $name{
                pub fn id() -> &'static str{ $id }
                $(
                    pub fn $prop_name () -> Option<$prop_type>{
                        None
                    }
                )*
            }
        )*
    )
}

macro_rules! rules{
    (
        {
            $($rule:tt)*
        }=>{
            $($implies:tt)*
        }
    ) => (

    )
}

rdf!{
    class Term = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Term" {
        name: BNode = "..."
    }
    class BNode = "..."{

    }
    class RdfTerm = "..."{

    }
    class Literal = "..."{

    }
    class RdfGraph = "..."{

    }
    class Symbol = "..."{

    }
    class Formula = "..."{
        statements: Symbol = "..."
    }
}

rules!{
    {
        Simon Wohnort Laufach .
    } => {
        |s, p, o| name
    }
}

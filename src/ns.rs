/*
rdf: "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
rdfs: "http://www.w3.org/2000/01/rdf-schema#"
*/

macro_rules! namespace{
    (
        $($name:ident : $base:expr {
            $( $sub:ident: $sub_name:expr),+
        })*
    ) => (
        $(pub mod $name{
            pub static BASE: &'static str = $base;
            $(
                pub static $sub : &'static str = concat!($base, $sub_name);
            )+
        })*
        fn full_uri(prefix: &str, suffix: &str) -> Option<&'static str>{
            match (prefix, suffix){
                $(
                    $(
                        (stringify!($name), $sub_name) => Some($name::$sub),
                    )+
                )*
                _ => None
            }
        }
        fn prefixed(full: &str) -> Option<(&'static str, &'static str)>{
            match full{
                $(
                    $(
                        concat!($base, $sub_name) => Some((stringify!($name), $sub_name)),
                    )+
                )*
                _ => None
            }
        }
    )
}

namespace!{
    rdfs: "http://www.w3.org/2000/01/rdf-schema#"{
        CLASS:           "Class",
        DATATYPE:        "Datatype",
        RESOURCE:        "Resource",
        SUB_CLASS_OF:    "subClassOf",
        SUB_PROPERTY_OF: "subPropertyOf",
        COMMENT:         "comment",
        LABEL:           "label",
        DOMAIN:          "domain",
        RANGE:           "range",
        SEE_ALSO:        "seeAlso",
        IS_DEFINED_BY:   "isDefinedBy",
        LITERAL:         "Literal",
        CONTAINER:       "Container",
        MEMBER:          "member",
        CONTAINER_MEMBERSHIP_PROPERTY: "ContainerMembershipProperty"
    }
    rdf: "http://www.w3.org/1999/02/22-rdf-syntax-ns#"{
        TYPE:          "type",
        HTML:          "HTML",
        LANG_STRING:   "langString",
        PLAIN_LITERAL: "PlainLiteral",
        PROPERTY:      "Property",
        STATEMENT:     "Statement",
        SUBJECT:       "subject",
        PREDICATE:     "predicate",
        OBJECT:        "object",
        BAG:           "Bag",
        SEQ:           "Seq",
        ALT:           "Alt",
        VALUE:         "value",
        LIST:          "List",
        NIL:           "nil",
        FIRST:         "first",
        REST:          "rest",
        XMLLITERAL:    "XMLLiteral"
    }
}

#[test]
fn test_prefix(){
    assert_eq!(full_uri("rdf","Property"), Some(rdf::PROPERTY));
    assert_eq!(prefixed(rdf::PROPERTY), Some(("rdf","Property")));
}

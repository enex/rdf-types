/*
rdf: "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
rdfs: "http://www.w3.org/2000/01/rdf-schema#"
*/

macro_rules! namespace{
    ( $name:ident : $base:expr {
        $( $sub:ident: $sub_name:expr),+
    }) => (
        pub mod $name{
            pub static BASE: &'static str = $base;
            $(

                pub static $sub : &'static str = concat!($base, $sub_name);
            )+
        }
    )
}

namespace!{
    rdfs: "http://www.w3.org/2000/01/rdf-schema#"{
        CLASS: "Class",
        DATATYPE: "Datatype"
    }
}
namespace!{
    rdf: "http://www.w3.org/1999/02/22-rdf-syntax-ns#"{
        TYPE:          "type",
        HTML:          "HTML",
        LANG_STRING:   "langString",
        PLAINLITERAL:  "PlainLiteral",
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

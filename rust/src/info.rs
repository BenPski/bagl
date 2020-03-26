// Type and data information
// useful for parsing data defintions
// type is defined by name and the variable content
// data is either a variable (wildcard) or a name and it arguments
// the arguments are other data definitions? might mix type level things (e.g., Stream a = Stream a (Stream a))
// List a = Cons a (List a) | Nil
// Type definition = Constructor [Type arguments] | Constructor
// Type definition is Type Constructor (Type variables)
// Type arguments: Type Variables or Type definition

// all datatypes are a sum of products, so first determine how to represent a product, then abstract to sums

// base product type is
// Product a b = Pair a b
// Product is a type constructor
// a and b are type variables
// Pair is a data constructor

// in order to do recursive types
// List a = Cons a (List a) | Nil
// need to have type constructor application for the List a on the right hand side

// List a can be looked at as a function
// List : a -> List a, where a is a type

// products are then function like
// Cons : a -> List a -> List a
// Nil : List a

// type definition
use crate::ast::Definition;
use crate::ast::Expr;

use crate::info::TypeInfo::*;

use std::cell::RefCell;
use std::rc::Rc;

pub enum TypeInfo {
    TConstructor(String),               // a type constructor
    TApp(Box<TypeInfo>, Box<TypeInfo>), // type application
    TVar(String),                       // or a type variable
}

// products are the constructor name and the type arguments
pub struct ProdInfo {
    pub name: String,
    pub args: Vec<TypeInfo>,
}

// sum is a collection of products
pub struct SumInfo {
    pub alts: Vec<ProdInfo>,
}

// definition of a data type
pub struct DataInfo {
    pub type_info: TypeInfo,
    pub data_info: SumInfo,
}

impl TypeInfo {
    fn new(input: Vec<String>) -> TypeInfo {
        // need to make variables, constructors, and applies
        // if first input is capitalized then it is a constructor
        // otherwise it should be a variable
        if input.len() == 0 {
            println!("Some issue with: {:?}", input);
            panic!("can't create type info from empty vector");
        } else if input.len() == 1 {
            let item = &input[0];
            if item.chars().next().unwrap().is_uppercase() {
                TConstructor(item.to_string())
            } else {
                TVar(item.to_string())
            }
        } else {
            // is an application
            let right = TypeInfo::new((&input[input.len() - 1..input.len()]).to_vec());
            let left = TypeInfo::new((&input[0..input.len() - 1]).to_vec());
            TApp(Box::new(left), Box::new(right))
        }
    }

    fn get_name(&self) -> String {
        match self {
            TConstructor(s) => s.to_string(),
            TApp(left, _) => left.get_name(),
            _ => {
                panic!("Didn't find a type constructor.");
            }
        }
    }
}

impl ProdInfo {
    fn new(name: String, args: Vec<TypeInfo>) -> ProdInfo {
        ProdInfo { name, args }
    }
}

impl SumInfo {
    fn new(alts: Vec<ProdInfo>) -> SumInfo {
        SumInfo { alts }
    }
}

impl DataInfo {
    fn new(type_info: TypeInfo, d_info: Vec<ProdInfo>) -> DataInfo {
        DataInfo {
            type_info,
            data_info: SumInfo::new(d_info),
        }
    }

    pub fn to_data(&self) -> Vec<Rc<Expr>> {
        // go through the data_info and create Data()
        let mut exprs = Vec::new();
        let type_name = self.type_info.get_name();
        for item in &self.data_info.alts {
            exprs.push(Rc::new(Expr::Data(
                item.args.len(),
                type_name.to_string(),
                item.name.to_string(),
                Vec::new(),
            )));
        }
        exprs
    }

    pub fn to_definitions(&self) -> Vec<Definition> {
        let mut defs = Vec::new();
        let type_name = self.type_info.get_name();
        for item in &self.data_info.alts {
            let def = Definition::new(
                Rc::new(Expr::Var(item.name.to_string(), RefCell::new(0))),
                Rc::new(Expr::Data(
                    item.args.len(),
                    type_name.to_string(),
                    item.name.to_string(),
                    Vec::new(),
                )),
            );

            defs.push(def);
        }
        defs
    }
}

// now do not have to construct in the grammar, just collect the strings
pub fn create_data_info(lhs: Vec<String>, rhs: Vec<Vec<Vec<String>>>) -> DataInfo {
    // lhs is the type definition
    // rhs are the constructor definitions
    // don't really like the nested vectors for rhs, but it should work

    let type_info = TypeInfo::new(lhs);
    let mut data_info = Vec::new();
    for item in rhs {
        // first should be constructor name
        // rest should be type info
        let mut type_data = Vec::new();
        for x in &item[1..] {
            type_data.push(TypeInfo::new(x.to_vec()));
        }
        data_info.push(ProdInfo::new(item[0][0].to_string(), type_data));
    }
    DataInfo::new(type_info, data_info)
}

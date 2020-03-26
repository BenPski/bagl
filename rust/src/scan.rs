/*

scan the input to determine the depth of the different variable locations in the environment

insert the depth into each variable

need to also have evaluation create environments in the desired ways


letrec:
    vars = defs
in expr

a new layer to the environment is added and all variables and definitions should have that depth

let:
    a = b
    c = f
in expr

becomes

let a = b in let c = f in expr

new layer added to environment and the defined variable and expression should point to the original layer


for lambdas have 2 options
    either do substitution as usual
    or treat it like a let and define the argument in a let

treating like it is a let is likely more effective, substitution has to descend the whole branch to do the substitution which can take quite a while
App(Lam(head, body), arg) -> Let(head, arg, body)


case statements, need to resolve depth in the condition and the branches
    the condition should be found in the current env
    each branch will have new variables defined with a letrec so add one to the dept


should this build up the environment and then use pointers instead?
    possibly have a very large environment to hold in memory when made
    just use depth


since I don't have cases quite worked out, just going to ignore them for now



may need to be determining depth from the top rather than depth from the bottom
    have to traverse all the way to the end and then determine the depth?
    or when calling a function defined as a variable does it just get a fresh environment?


the simplest case of the issue is:
    main = (\ y . y) (\ x . + 1 x) 1

when the second lambda tries to lookup x it looks in the wrong place
in the scanning we see that an environment layer gets added to the id function, but the add function never finds out about it
    have to keep track of the environment depth from left to right

now the problem is with
id y = y;

main = id (\ x . + 1 x) 2

since the depth of the variable isn't adjusted when it gets substituted the environment changes, but the resolver won't know that
    either the resolver needs to be more complex or the order of looking things up needs to be adjusted

this is why it seems that the depth needs to be set as a distance from the end of the environment rather than the bottom
    avoids the problem because environments are always added to the top and the position is defined relative to the top


now want to store how far away a variable is from an environment
that means at something that defines an environment is at 0 and then deeper into it


or it is at the definition level that things start being defined as 0


Env = Empty
main = (\x . x) 1

env = Context(x = 1, Empty)
main = x

x should associate with the top of the environment

Env = Empty
main = (\x y . x) 1 2

env = Context(x = 1, Empty)
main = (\ y . x) 2

env = Context(y = 2, Context(x = 1, Empty))
main = x

when processing let's should to reset the depth counter, so that when calling the variable it points to the right place

now the index should correspond to how many environments to drop when looking for the variable

seems to now be working, really overcomplicated that at first
    let and letrec now act differently as desired


*/

use crate::ast::Expr;
use std::rc::Rc;

pub fn resolve(expr: Rc<Expr>, depth: usize) -> usize {
    match &*expr {
        Expr::Var(_, d) => {
            // set depth
            d.replace(depth);
            depth
        }
        Expr::App(left, right) => {
            // have to do special treatment as lambdas will modify the environment when defined
            let d = resolve(Rc::clone(left), depth);
            resolve(Rc::clone(right), d)
        }
        Expr::Let(_, defs, body) => {
            // for each variable create environment and resolve the depth in the variable and the definition
            for def in defs {
                resolve(Rc::clone(def), 1);
            }
            resolve(Rc::clone(body), 0)
        }
        Expr::LetRec(_, defs, body) => {
            // define all variables in a new environment layer and set all of the depths to the new one
            for def in defs {
                resolve(Rc::clone(def), 0);
            }
            resolve(Rc::clone(body), 0)
        }
        Expr::If(cond, b1, b2) => {
            resolve(Rc::clone(cond), depth);
            resolve(Rc::clone(b1), depth);
            resolve(Rc::clone(b2), depth);
            depth
        }
        Expr::Lam(_, body) => resolve(Rc::clone(body), 0),
        _ => depth,
    }
}

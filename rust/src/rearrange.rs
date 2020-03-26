/*

analyze lets to determine where recursive definitions are and when definitions are not

allows to start only with lets and the recursiveness is determined


using tarjan's algorithm will give all the desired results
    cycles grouped
    topologically ordered

The way I've written the algorithm only works with ints rather than general objects
    for now just map back and forth

One thing that doesn't get automatically caught is self recursion, just has to be detected externally

first take the variables and associate them with an index
    find occurences of these variables and construct the graph associated with this
        recurse when finding internal let's
    use tarjan
    reconstruct with lets and letrecs


works now, maybe inefficient with scanning both recursion and dependencies separately


*/

use crate::ast::Expr;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

// recreate the ast (for now immutable style) with the substitutions of lets/letrecs for letrecs
pub fn change_lets(expr: Rc<Expr>) -> Rc<Expr> {
    match &*expr {
        Expr::LetRec(vars, defs, body) => {
            // first collect variables and give them a number
            // look through definitions and find dependencies
            // mark the ones that are self-recursive
            // use tarjan to rearrange
            // convert to lets and letrecs
            // lets come from single groups
            // letrecs come from self-recursive single groups and all cycles
            // should declare them in the order that tarjan's algorithm returns

            // first need to recurse on defs
            let mut defs_new = Vec::new();
            for def in defs {
                defs_new.push(change_lets(Rc::clone(def)));
            }
            let defs = defs_new;

            let mut names = Vec::new();
            let mut assoc = HashMap::new();
            let mut self_rec = Vec::new();

            for var in vars {
                if let Expr::Var(s, _) = Rc::deref(&var) {
                    names.push(s.to_string());
                    assoc.insert(s, names.len() - 1);
                }
            }

            let mut graph = HashMap::new();

            for i in 0..defs.len() {
                let depends = dependencies(&names, Rc::clone(&defs[i]), Vec::new());
                if depends.contains(&names[i]) {
                    self_rec.push(i);
                }
                let mut depends_index = Vec::new();
                for d in depends {
                    if let Some(v) = assoc.get(&d) {
                        depends_index.push(*v);
                    }
                }
                if let Some(v) = assoc.get(&names[i]) {
                    graph.insert(*v, depends_index);
                }
            }
            let mut organized = Vec::new();
            tarjan(&graph, &mut organized);
            organized.reverse(); // order to build let ast out of
            let mut expr = change_lets(Rc::clone(body));
            for cycle in organized {
                if cycle.len() == 1 {
                    let index = cycle[0];
                    if self_rec.contains(&index) {
                        expr = Rc::new(Expr::LetRec(
                            vec![Rc::clone(&vars[index])],
                            vec![Rc::clone(&defs[index])],
                            expr,
                        ));
                    } else {
                        expr = Rc::new(Expr::Let(
                            vec![Rc::clone(&vars[index])],
                            vec![Rc::clone(&defs[index])],
                            expr,
                        ));
                    }
                } else {
                    let mut vars_new = Vec::new();
                    let mut defs_new = Vec::new();
                    for index in cycle {
                        vars_new.push(Rc::clone(&vars[index]));
                        defs_new.push(Rc::clone(&defs[index]));
                    }
                    expr = Rc::new(Expr::LetRec(vars_new, defs_new, expr));
                }
            }
            expr
        }
        Expr::Let(_, _, _) => {
            panic!("shouldn't have run into let when trying to rearrange letrecs")
        }
        Expr::Lam(head, body) => Rc::new(Expr::Lam(Rc::clone(head), change_lets(Rc::clone(body)))),
        Expr::App(left, right) => Rc::new(Expr::App(
            change_lets(Rc::clone(left)),
            change_lets(Rc::clone(right)),
        )),
        Expr::If(cond, b1, b2) => Rc::new(Expr::If(
            change_lets(Rc::clone(cond)),
            change_lets(Rc::clone(b1)),
            change_lets(Rc::clone(b2)),
        )),
        Expr::Case(cond, pats, defs) => {
            let mut new_defs = Vec::new();
            for def in defs {
                new_defs.push(change_lets(Rc::clone(def)));
            }
            Rc::new(Expr::Case(
                change_lets(Rc::clone(cond)),
                pats.to_vec(),
                new_defs,
            ))
        }
        _ => expr,
    }
}

fn dependencies(names: &Vec<String>, def: Rc<Expr>, acc: Vec<String>) -> Vec<String> {
    // look through def and see if any variables match a name
    // probably has issues with shadowing, will ignore for now (probably want to make shadowing an error anyways)
    match &*def {
        Expr::Var(s, _) => {
            if names.contains(s) {
                let mut acc = acc;
                acc.push(s.to_string());
                acc
            } else {
                acc
            }
        }
        Expr::Lam(_, body) => dependencies(names, Rc::clone(body), acc),
        Expr::App(left, right) => {
            let left_acc = dependencies(names, Rc::clone(left), acc);
            dependencies(names, Rc::clone(right), left_acc)
        }
        Expr::If(cond, b1, b2) => {
            let cond_acc = dependencies(names, Rc::clone(cond), acc);
            let b1_acc = dependencies(names, Rc::clone(b1), cond_acc);
            dependencies(names, Rc::clone(b2), b1_acc)
        }
        Expr::Case(cond, _, branches) => {
            let mut acc = acc;
            for branch in branches {
                acc = dependencies(names, Rc::clone(branch), acc);
            }
            dependencies(names, Rc::clone(cond), acc)
        }
        Expr::Let(_, defs, body) => {
            let mut acc = acc;
            for def in defs {
                acc = dependencies(names, Rc::clone(def), acc);
            }
            dependencies(names, Rc::clone(body), acc)
        }
        Expr::LetRec(_, defs, body) => {
            let mut acc = acc;
            for def in defs {
                acc = dependencies(names, Rc::clone(def), acc);
            }
            dependencies(names, Rc::clone(body), acc)
        }
        _ => acc,
    }
}

fn min(a: usize, b: usize) -> usize {
    if a <= b {
        a
    } else {
        b
    }
}
//letting a graph be a hashmap for now
fn tarjan(graph: &HashMap<usize, Vec<usize>>, result: &mut Vec<Vec<usize>>) {
    let mut index_counter = 0;
    let mut stack = Vec::new();
    let mut lowlinks = HashMap::new();
    let mut index = HashMap::new();

    for node in graph.keys() {
        if !lowlinks.contains_key(node) {
            strongconnect(
                graph,
                &mut index_counter,
                &mut stack,
                &mut lowlinks,
                &mut index,
                result,
                *node,
            );
        }
    }
}

fn strongconnect(
    graph: &HashMap<usize, Vec<usize>>,
    index_counter: &mut usize,
    stack: &mut Vec<usize>,
    lowlinks: &mut HashMap<usize, usize>,
    index: &mut HashMap<usize, usize>,
    result: &mut Vec<Vec<usize>>,
    node: usize,
) {
    index.insert(node, *index_counter);
    lowlinks.insert(node, *index_counter);
    *index_counter += 1;
    stack.push(node);

    let succ = graph.get(&node);
    let mut successors = Vec::new();
    if let Some(a) = succ {
        successors = a.to_vec();
    }

    for successor in successors {
        if !lowlinks.contains_key(&successor) {
            strongconnect(
                graph,
                index_counter,
                stack,
                lowlinks,
                index,
                result,
                successor,
            );
            let mut new_val = 0;
            if let (Some(a), Some(b)) = (lowlinks.get(&node), lowlinks.get(&successor)) {
                new_val = min(*a, *b);
            }
            lowlinks.insert(node, new_val);
        } else if stack.contains(&successor) {
            let mut new_val = 0;
            if let Some(a) = lowlinks.get(&node) {
                new_val = min(*a, index[&successor]);
            }
            lowlinks.insert(node, new_val);
        }
    }
    if let Some(a) = lowlinks.get(&node) {
        if a == &index[&node] {
            let mut conn_comp = Vec::new();
            loop {
                if let Some(successor) = stack.pop() {
                    conn_comp.push(successor);
                    if successor == node {
                        break;
                    }
                }
            }
            result.push(conn_comp);
        }
    }
}

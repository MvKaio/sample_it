use std::collections::HashMap;
use std::error::Error;

use rusqlite::Connection;
use good_lp::variable::Variable;
use good_lp::*;
use good_lp::{variables, variable, Expression, default_solver};

use crate::database;
use crate::database::model::{Sample, SampleResult, Constraint, Operator};

use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn solve(sample: Sample, connection: &Connection) -> Result<SampleResult, Box<dyn Error>> {
    let mut collection = database::functions::get_collection(sample.collection_id, connection)?;
    let constraints: Vec<Constraint> = sample.constraints.iter().map(|cons| {
        let mut new_constraint = cons.clone();
        new_constraint.label = (collection.labels.iter().find(|l| l.name == cons.label.name).unwrap()).clone();
        new_constraint
    }).collect();

    collection.items.shuffle(&mut thread_rng());

    // variables -> one for each item [indexed by id]
    let mut variables: HashMap<u32, Variable> = HashMap::new();
    let mut problem = variables!();
    let mut sum_all = Expression::with_capacity(collection.items.len());

    for item in collection.items.iter() {
        let variable = problem.add(variable().binary());
        variables.insert(item.id, variable);
        sum_all = sum_all + variable;
    }

    let mut system = problem.maximise(sum_all.clone())
                            .using(default_solver);

    system = system.with(sum_all.eq(sample.size as i32));

    // constraints -> one for each sample
    for constraint in constraints.iter() {
        let mut exp = Expression::with_capacity(collection.items.len());
        for item in collection.items.iter() {
            if let Some(_) =  item.labels.iter().find(|l| l.id == constraint.label.id) {
                exp = exp + variables.get(&item.id).unwrap();
            } 
        }
        let cons = match constraint.operator {
            Operator::LessOrEqual => exp.clone().leq(constraint.number),
            Operator::Equal => exp.clone().eq(constraint.number),
            Operator::GreaterOrEqual => exp.clone().geq(constraint.number as i32),
        };
        system = system.with(cons);
    }

    let solution = system.solve();
    if let Err(_) = solution {
        return Ok(SampleResult::SolutionNotFound);
    }
    let solution = solution.unwrap();

    let mut items = vec![];
    for item in collection.items.iter() {
        let var = variables.get(&item.id).unwrap();
        if solution.value(var.clone()) > 0.0 {
            items.push(item.clone());
        }
    }

    Ok(SampleResult::Solved { items })
}

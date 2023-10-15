use std::fmt::Debug;
pub mod semantics;
use semantics::Acceptability::{IN, OUT, UNDEC};
use varisat::{CnfFormula, ExtendFormula, Lit, Var};

use self::semantics::Labelling;

use super::sat::SAT;

#[derive(Debug)]
pub struct Attack {
    origin: usize,
    target: usize,
}

impl Attack {
    pub fn new(origin: usize, target: usize) -> Attack {
        Attack { origin, target }
    }
}

#[derive(Debug)]
pub struct AF {
    num_of_args: usize,
    attacks: Vec<Attack>,
}

impl semantics::Semantics for AF {
    fn complete(self) -> Vec<Labelling> {
        let n = self.num_of_args;
        let mut formula = CnfFormula::new();
        let inn = formula.new_var_iter(n).collect::<Vec<Var>>();
        let out = formula.new_var_iter(n).collect::<Vec<Var>>();
        let und = formula.new_var_iter(n).collect::<Vec<Var>>();

        /* Section 3.1 http://www.mthimm.de/pub/2020/Klein_2020.pdf */
        // (1)
        for i in 0..n {
            formula.add_clause(&[inn[i].positive(), out[i].positive(), und[i].positive()]);
            formula.add_clause(&[inn[i].negative(), out[i].negative()]);
            formula.add_clause(&[inn[i].negative(), und[i].negative()]);
            formula.add_clause(&[out[i].negative(), und[i].negative()]);
        }
        let attacker_map = self.attacker_map();
        for i in 0..n {
            let attackers = &attacker_map[i];
            // (2)
            if attackers.is_empty() {
                formula.add_clause(&[inn[i].positive(), out[i].negative(), und[i].negative()]);
                continue;
            }
            // (3)
            for &j in attackers {
                formula.add_clause(&[inn[i].negative(), out[j].positive()]);
            }
            // (4)
            let mut clause4 = attackers
                .iter()
                .map(|&j| out[j].negative())
                .collect::<Vec<Lit>>();
            clause4.push(inn[i].positive());
            formula.add_clause(&clause4);
            // (5)
            let mut clause5 = attackers
                .iter()
                .map(|&j| inn[j].positive())
                .collect::<Vec<Lit>>();
            clause5.push(out[i].negative());
            formula.add_clause(&clause5);
            // (6)
            for &j in attackers {
                formula.add_clause(&[inn[j].negative(), out[i].positive()]);
            }
        }
        self.compute(&formula)
    }
}

impl AF {
    /** Creates a new Argumentation Framework from an attack relation.
     * The framework has arguments 0, 1, ..., max; where max is the highest number in `attacks`
     */
    pub fn new(attacks: Vec<Attack>) -> AF {
        let max = attacks.iter().flat_map(|a| [a.origin, a.target]).max();
        AF {
            num_of_args: match max {
                Some(x) => x + 1,
                None => 0,
            },
            attacks,
        }
    }

    /** Computes a labelling from a boolean assignment of literals */
    pub fn label(&self, lits: &Vec<Lit>) -> Labelling {
        let n = self.num_of_args;
        Labelling(
            (0..n)
                .map(|i| {
                    if lits[i].is_positive() {
                        return IN;
                    }
                    if lits[n + i].is_positive() {
                        return OUT;
                    }
                    return UNDEC;
                })
                .collect(),
        )
    }

    fn compute(&self, formula: &CnfFormula) -> Vec<Labelling> {
        SAT::new()
            .enumerate(formula)
            .iter()
            .map(|model| self.label(model))
            .collect::<Vec<Labelling>>()
    }

    fn attacker_map(&self) -> Vec<Vec<usize>> {
        let mut result = vec![];
        for arg in 0..self.num_of_args {
            let mut attackers = vec![];
            for Attack { origin, target } in &self.attacks {
                if *target == arg {
                    attackers.push(*origin)
                }
            }
            result.push(attackers);
        }
        result
    }
}

use std::{collections::HashMap, fmt::Debug};
pub mod encoding;
pub mod semantics;
use semantics::Acceptability::{IN, OUT, UNDEC};
use varisat::{CnfFormula, ExtendFormula, Lit, Var};

use self::{encoding::Enconding, semantics::Labelling};

use super::sat::{CnfFormulaExtension, Formula, Vars, SAT};

#[derive(Debug)]
pub struct Attack(pub usize, pub usize);

pub struct AF {
    pub num_of_args: usize,
    pub attacks: Vec<Attack>,
    names: Option<HashMap<String, usize>>,
}

impl Debug for AF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut binding = f.debug_struct("AF");
        let n: usize = self.num_of_args;
        let names_by_index: Option<Vec<&str>> = self.names_by_index();
        binding.field("num_of_args", &n);
        match names_by_index {
            Some(names_by_index) => {
                return binding
                    .field(
                        "attacks",
                        &self
                            .attacks
                            .iter()
                            .map(|att| {
                                let Attack(origin, target) = att;

                                (
                                    names_by_index.get(*origin).unwrap_or(&"null"),
                                    names_by_index.get(*target).unwrap_or(&"null"),
                                )
                            })
                            .collect::<Vec<_>>(),
                    )
                    .finish();
            }
            None => binding.field("attacks", &self.attacks).finish(),
        }
    }
}

impl semantics::Semantics for AF {
    fn complete(&self) -> Vec<Labelling> {
        let mut formula = self.create_formula();
        self.add_complete_clauses(&mut formula);
        self.compute(&formula.cnf)
    }
    fn stable(&self) -> Vec<Labelling> {
        let mut formula = self.create_formula();
        self.add_stable_clauses(&mut formula);
        self.compute(&formula.cnf)
    }

    /* Algorithm 1 from https://arxiv.org/pdf/1310.4986.pdf */
    fn preferred(&self) -> Vec<Labelling> {
        let mut labellings = vec![];
        let mut formula = self.create_formula();
        let n = formula.vars.i.len();
        self.add_complete_clauses(&mut formula);
        self.add_not_empty_clause(&mut formula);

        let _compute_preferred_candidate = |cnfdf: &mut CnfFormula| {
            let mut pref_candidate = vec![];
            loop {
                let mut all_are_in = true;
                if let Some(model) = SAT::solve(&cnfdf) {
                    pref_candidate = model; // move ownership
                    let mut remaining: Vec<Lit> = vec![];
                    for i in 0..n {
                        let lit = formula.vars.i[i].positive();
                        if pref_candidate[i].is_positive() {
                            // IN
                            cnfdf.add_clause(&vec![lit]);
                        } else {
                            // OUT or UNDEC
                            remaining.push(lit);
                            all_are_in = false;
                        }
                    }
                    cnfdf.add_clause(&remaining);
                }
                // all_are_in is also true when last_complete == None (i.e., no model was found)
                if all_are_in {
                    break;
                }
            }
            pref_candidate
        };

        loop {
            let mut cnfdf = formula.cnf.clone();
            let pref_candidate = _compute_preferred_candidate(&mut cnfdf);
            if pref_candidate.is_empty() {
                break;
            }
            labellings.push(self.label(&pref_candidate));
            let optimize_clause = (0..n)
                .filter_map(|i| {
                    if pref_candidate[i].is_positive() {
                        return None; // when IN
                    }
                    return Some(formula.vars.i[i].positive()); // when not IN
                })
                .collect::<Vec<Lit>>();
            formula.cnf.add_clause(&optimize_clause);
        }
        if labellings.is_empty() {
            labellings.push(Labelling(vec![]));
        }
        labellings
    }
}

impl AF {
    /** Creates a new Argumentation Framework from an attack relation.
     * The framework has arguments 0, 1, ..., max; where max is the highest number in `attacks`
     */
    pub fn new(attacks: Vec<Attack>) -> AF {
        let max = attacks
            .iter()
            .flat_map(|Attack(origin, target)| vec![origin, target])
            .max();
        AF {
            num_of_args: match max {
                Some(x) => x + 1,
                None => 0,
            },
            attacks,
            names: None,
        }
    }

    pub fn new_named(attacks: Vec<Attack>, names: HashMap<String, usize>) -> AF {
        AF {
            num_of_args: names.len(),
            attacks,
            names: Some(names),
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
        SAT::enumerate(formula)
            .iter()
            .map(|model| self.label(model))
            .collect::<Vec<Labelling>>()
    }

    fn attacker_map(&self) -> Vec<Vec<usize>> {
        let n = self.num_of_args;
        let mut result = vec![];
        for arg in 0..n {
            let mut attackers = vec![];
            for Attack(origin, target) in &self.attacks {
                if *target == arg {
                    attackers.push(*origin);
                }
            }
            result.push(attackers);
        }
        result
    }

    fn create_formula(&self) -> Formula {
        let n = self.num_of_args;
        let mut cnf = CnfFormula::new();
        let i = cnf.new_var_iter(n).collect::<Vec<Var>>();
        let o = cnf.new_var_iter(n).collect::<Vec<Var>>();
        let u = cnf.new_var_iter(n).collect::<Vec<Var>>();
        Formula {
            vars: Vars { i, o, u },
            cnf,
        }
    }

    fn add_complete_clauses(&self, formula: &mut Formula) {
        let n: usize = self.num_of_args;
        let Formula { vars, cnf } = formula;
        let Vars {
            i: inn,
            o: out,
            u: und,
        } = vars;

        /*
            Definition 5 https://arxiv.org/pdf/1310.4986.pdf
            C_in  <-> are the set of clauses (3) and (4)
            C_out <-> are the set of clauses (5) and (6)
        */
        let attacker_map = self.attacker_map();
        for i in 0..n {
            // (1)
            cnf.add_clause(&[inn[i].positive(), out[i].positive(), und[i].positive()]);
            cnf.add_clause(&[inn[i].negative(), out[i].negative()]);
            cnf.add_clause(&[inn[i].negative(), und[i].negative()]);
            cnf.add_clause(&[out[i].negative(), und[i].negative()]);

            let attackers = &attacker_map[i];
            // (2)
            if attackers.is_empty() {
                cnf.add_clause(&[inn[i].positive()]);
                cnf.add_clause(&[out[i].negative()]);
                cnf.add_clause(&[und[i].negative()]);
                continue;
            }
            // (3)
            let mut clause3 = attackers
                .iter()
                .map(|&j| out[j].negative())
                .collect::<Vec<Lit>>();
            clause3.push(inn[i].positive());
            cnf.add_clause(&clause3);
            // (4)
            for &j in attackers {
                cnf.add_clause(&[inn[i].negative(), out[j].positive()]);
            }
            // (5)
            for &j in attackers {
                cnf.add_clause(&[inn[j].negative(), out[i].positive()]);
            }
            // (6)
            let mut clause6 = attackers
                .iter()
                .map(|&j| inn[j].positive())
                .collect::<Vec<Lit>>();
            clause6.push(out[i].negative());
            cnf.add_clause(&clause6);
        }
    }

    fn add_stable_clauses(&self, formula: &mut Formula) {
        self.add_complete_clauses(formula);
        for i in 0..self.num_of_args {
            let undec_false = formula.vars.u[i].negative();
            formula.cnf.add_clause(&vec![undec_false]);
        }
    }

    fn add_not_empty_clause(&self, formula: &mut Formula) {
        formula.cnf.add_clause(
            &formula
                .vars
                .i
                .iter()
                .map(|v| v.positive())
                .collect::<Vec<Lit>>(),
        )
    }

    pub fn names_by_index(&self) -> Option<Vec<&str>> {
        match &self.names {
            Some(names) => {
                let mut by_index = vec!["null"; self.num_of_args];
                for (name, &i) in names {
                    by_index[i] = &name;
                }
                Some(by_index)
            }
            None => None,
        }
    }
}

impl From<Enconding> for AF {
    fn from(enc: Enconding) -> Self {
        match enc {
            Enconding::SIMPLE(labels, attacks) => {
                let mut att = vec![];
                let mut index_by_label: HashMap<String, usize> = HashMap::new();
                for (i, label) in labels.iter().enumerate() {
                    index_by_label.insert(label.to_owned(), i);
                }
                for (origin, target) in attacks {
                    if let Some(&origin_i) = index_by_label.get(&origin) {
                        if let Some(&target_i) = index_by_label.get(&target) {
                            att.push(Attack(origin_i, target_i));
                        }
                    }
                }
                AF::new_named(att, index_by_label)
            }
            Enconding::ERROR(_) => AF::new(vec![]),
        }
    }
}

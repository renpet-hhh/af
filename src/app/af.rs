use std::{collections::HashMap, fmt::Debug};
pub mod semantics;
use semantics::Acceptability::{IN, OUT, UNDEC};
use varisat::{CnfFormula, ExtendFormula, Lit, Var};
use web_sys::File;

use self::semantics::Labelling;

use super::sat::{CnfFormulaExtension, Formula, Vars, SAT};

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

pub struct AF {
    num_of_args: usize,
    attacks: Vec<Attack>,
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
                                let Attack { origin, target } = att;

                                (names_by_index.get(*origin), names_by_index.get(*target))
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
        let max = attacks.iter().flat_map(|a| [a.origin, a.target]).max();
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
            Section 3.1 http://www.mthimm.de/pub/2020/Klein_2020.pdf
            Definition 5 https://arxiv.org/pdf/1310.4986.pdf
            C_in  <-> are the set of clauses (3) and (4)
            C_out <-> are the set of clauses (5) and (6)
        */
        // (1)
        for i in 0..n {
            cnf.add_clause(&[inn[i].positive(), out[i].positive(), und[i].positive()]);
            cnf.add_clause(&[inn[i].negative(), out[i].negative()]);
            cnf.add_clause(&[inn[i].negative(), und[i].negative()]);
            cnf.add_clause(&[out[i].negative(), und[i].negative()]);
        }
        let attacker_map = self.attacker_map();
        for i in 0..n {
            let attackers = &attacker_map[i];
            // (2)
            if attackers.is_empty() {
                cnf.add_clause(&[inn[i].positive(), out[i].negative(), und[i].negative()]);
                continue;
            }
            // (3)
            for &j in attackers {
                cnf.add_clause(&[inn[i].negative(), out[j].positive()]);
            }
            // (4)
            let mut clause4 = attackers
                .iter()
                .map(|&j| out[j].negative())
                .collect::<Vec<Lit>>();
            clause4.push(inn[i].positive());
            cnf.add_clause(&clause4);
            // (5)
            let mut clause5 = attackers
                .iter()
                .map(|&j| inn[j].positive())
                .collect::<Vec<Lit>>();
            clause5.push(out[i].negative());
            cnf.add_clause(&clause5);
            // (6)
            for &j in attackers {
                cnf.add_clause(&[inn[j].negative(), out[i].positive()]);
            }
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

    pub async fn from_file<'a>(file: File) -> AF {
        let content = wasm_bindgen_futures::JsFuture::from(file.text()).await;
        let empty = AF::new(vec![]);
        match content {
            Err(_) => empty,
            Ok(content) => {
                if let Some(text) = content.as_string() {
                    let mut names = HashMap::new();
                    let mut attacks = vec![];
                    for line in text.lines() {
                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }

                        let start = line.find('(');
                        let end = line.find(')');
                        if let Some(start) = start {
                            if let Some(end) = end {
                                let before = &line[..start];
                                let center = &line[start + 1..end];
                                match before {
                                    "arg" => {
                                        names.insert(center.to_owned(), names.len());
                                        continue;
                                    }
                                    "att" => {
                                        let parts = center.split(',').collect::<Vec<_>>();
                                        if let [origin, target] = parts[..] {
                                            let origin = names.get(origin);

                                            let target = names.get(target);

                                            if let Some(&origin) = origin {
                                                if let Some(&target) = target {
                                                    attacks.push(Attack { origin, target });
                                                }
                                            }
                                        }
                                        continue;
                                    }
                                    _ => continue,
                                }
                            }
                        }
                    }
                    return AF::new_named(attacks, names);
                }
                empty
            }
        }
    }

    fn names_by_index(&self) -> Option<Vec<&str>> {
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

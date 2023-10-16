use varisat::{solver::Solver, CnfFormula, ExtendFormula, Lit, Var};

pub struct SAT {}

impl SAT {
    pub fn solve(cnf: &CnfFormula) -> Option<Vec<Lit>> {
        let mut solver = Solver::new();
        solver.add_formula(cnf);
        let _ = solver.solve();
        solver.model()
    }

    pub fn enumerate(cnf: &CnfFormula) -> Vec<Vec<Lit>> {
        let mut solver = Solver::new();
        let mut result: Vec<Vec<Lit>> = vec![];
        solver.add_formula(cnf);
        while solver.solve().unwrap_or(false) {
            let model = solver.model();
            if let Some(m) = model {
                result.push(m.clone());
                let exclude = m.iter().map(|&lit| !lit).collect::<Vec<Lit>>();
                solver.add_clause(&exclude);
            }
        }
        result
    }
}

pub struct Vars {
    pub i: Vec<Var>,
    pub o: Vec<Var>,
    pub u: Vec<Var>,
}
pub struct Formula {
    pub vars: Vars,
    pub cnf: CnfFormula,
}

pub trait CnfFormulaExtension {
  fn clone(&self) -> CnfFormula;
}

impl CnfFormulaExtension for CnfFormula {
  fn clone(&self) -> CnfFormula {
      let mut cnf = CnfFormula::new();
      for clause in self.iter() {
        cnf.add_clause(clause);
      }
      cnf
  }
}
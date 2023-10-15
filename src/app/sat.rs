use varisat::{solver::Solver, CnfFormula, ExtendFormula, Lit};

pub struct SAT<'a> {
  solver: Solver<'a>
}

impl SAT<'_> {
  pub fn new<'a>() -> SAT<'a> {
      let sat = SAT { solver: Solver::new() };
      sat
  }

  pub fn enumerate(&mut self, cnf: &CnfFormula) -> Vec<Vec<Lit>> {
      let mut result: Vec<Vec<Lit>> = vec![];
      self.solver.add_formula(cnf);
      while self.solver.solve().unwrap_or(false) {
          let model = self.solver.model();
          if let Some(m) = model {
              result.push(m.clone());
              let exclude = m.iter().map(|&lit| !lit).collect::<Vec<Lit>>();
              self.solver.add_clause(&exclude);
          }
      }
      result
  }
}


pub enum Enconding {
    /* SIMPLE(labels, attacks)

      Example:

      arg(x).
      arg(y).
      arg(x, y).

      SIMPLE(vec!["x", "y"], vec![("x", "y")])
    */
    SIMPLE(Vec<String>, Vec<(String, String)>),
    ERROR(String),
}

impl Enconding {
    pub fn parse_simple<'a>(text: String) -> Enconding {
        let mut labels = vec![];
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
                            let label = center.to_owned();
                            labels.push(label);
                            continue;
                        }
                        "att" => {
                            let parts = center.split(',').collect::<Vec<_>>();
                            if let [origin, target] = parts[..] {
                                attacks.push((origin.to_owned(), target.to_owned()));
                            }
                            continue;
                        }
                        _ => continue,
                    }
                }
            }
        }
        return Enconding::SIMPLE(labels, attacks);
    }
}

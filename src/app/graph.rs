use super::{
    af::{
        semantics::{Acceptability, Labelling},
        AF,
    },
    glue::update_vis_network,
};

pub trait VisDrawable {
    fn update_vis(&self, id: &str, labelling: Option<&Labelling>);
}

fn color_by_acceptability(acc: &Acceptability) -> String {
    match acc {
        Acceptability::IN => String::from("green"),
        Acceptability::OUT => String::from("red"),
        Acceptability::UNDEC => String::from("blue"),
    }
}

impl VisDrawable for AF {
    fn update_vis(&self, id: &str, labelling: Option<&Labelling>) {
        let mut labels: Vec<String> = vec![];
        let mut colors: Vec<String> = vec![];
        if let Some(_labels) = self.names_by_index() {
            for (i, &label) in _labels.iter().enumerate() {
                labels.push(label.to_owned());
                if let Some(labelling) = labelling {
                    let color = color_by_acceptability(&labelling.0[i]);
                    colors.push(color);
                }
            }
        } else {
            for i in 0..self.num_of_args {
                labels.push(i.to_string());
            }
        }
        update_vis_network(id, labels, &self.attacks, colors);
    }
}

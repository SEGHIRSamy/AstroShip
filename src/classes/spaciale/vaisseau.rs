use crate::classes::spaciale::planete::Planete;

pub struct Vaisseau {
    carburant: i32,
    uranium: i32,
    position: Option<Planete>, // La planète actuelle du vaisseau (None = espace)
}

impl Vaisseau {
    pub fn new(carburant: i32, uranium: i32, position: Option<Planete>) -> Self {
        Self { carburant, uranium, position }
    }

    pub fn afficher_etat(&self) {

        let position_str = match &self.position {
            Some(planete) => format!("Sur {}", planete.nom),
            None => "Dans l'espace".to_string(),
        };
        println!("Carburant: {}, Uranium: {}, Position: {}", self.carburant, self.uranium, position_str);
    }

    pub fn voyager(&mut self, planete: &Planete) -> bool {
        if self.carburant >= planete.cout_voyage {
            self.carburant -= planete.cout_voyage;
            self.position = Some(planete.clone());
            println!("Voyage réussi vers {} ! Carburant restant: {}", planete.nom, self.carburant);
            true
        } else {
            println!("Pas assez de carburant pour aller sur {} !", planete.nom);
            false
        }
    }
}

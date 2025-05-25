use crate::classes::spaciale::voyage_planete::VoyagePlanete;

pub struct Vaisseau {
    carburant: i32,
    uranium: i32,
    position: Option<VoyagePlanete>,
}

#[allow(dead_code)]
impl Vaisseau {
    /// Crée un nouveau vaisseau avec du carburant, de l'uranium et une position initiale
    pub fn new(carburant: i32, uranium: i32, position: Option<VoyagePlanete>) -> Self {
        Self { carburant, uranium, position }
    }

    /// Affiche l'état actuel du vaisseau dans la console
    pub fn afficher_etat(&self) {
        let position_str = match &self.position {
            Some(planete) => format!("Sur {}", planete.nom),
            None => "Dans l'espace".to_string(),
        };
        println!(
            "Carburant: {}, Uranium: {}, Position: {}",
            self.carburant, self.uranium, position_str
        );
    }

    /// Permet de voyager vers une planète si le carburant est suffisant
    pub fn voyager(&mut self, planete: &VoyagePlanete) -> bool {
        if self.carburant >= planete.cout_voyage {
            self.carburant -= planete.cout_voyage;
            self.position = Some(planete.clone());
            println!(
                "Voyage réussi vers {} ! Carburant restant: {}",
                planete.nom, self.carburant
            );
            true
        } else {
            println!("Pas assez de carburant pour aller sur {} !", planete.nom);
            false
        }
    }

    /// Retourne le carburant actuel
    pub fn get_carburant(&self) -> i32 {
        self.carburant
    }

    /// Retourne l'uranium actuel
    pub fn get_uranium(&self) -> i32 {
        self.uranium
    }

    /// Retourne la position actuelle du vaisseau
    pub fn get_position(&self) -> Option<&VoyagePlanete> {
        self.position.as_ref()
    }
}

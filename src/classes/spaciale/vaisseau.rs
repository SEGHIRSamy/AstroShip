use crate::planete::Planete;

pub struct Vaisseau {
    carburant: i32,
    uranium: i32,
}

impl Vaisseau {
    // Constructeur
    pub fn new(carburant: i32, uranium: i32) -> Self {
        Self { carburant, uranium }
    }

    // Méthode pour afficher l'état du vaisseau
    pub fn afficher_etat(&self) {
        println!("Carburant: {}, Uranium: {}", self.carburant, self.uranium);
    }

    // Méthode pour consommer du carburant
    pub fn consommer_carburant(&mut self, quantite: i32) {
        if self.carburant >= quantite {
            self.carburant -= quantite;
            println!("Carburant consommé: {}. Reste: {}", quantite, self.carburant);
        } else {
            println!("Pas assez de carburant !");
        }
    }

    // Méthode pour voyager vers une planète
    pub fn voyager(&mut self, planete: &Planete) -> bool {
        if self.carburant >= planete.cout_voyage {
            self.carburant -= planete.cout_voyage;
            println!("Voyage réussi vers {} ! Carburant restant: {}", planete.nom, self.carburant);
            true
        } else {
            println!("Pas assez de carburant pour aller sur {} !", planete.nom);
            false
        }
    }
}

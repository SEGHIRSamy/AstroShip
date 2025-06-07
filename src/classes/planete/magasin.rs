use serde::{Deserialize, Serialize};
use crate::classes::entite::personnage_principal::PersonnagePrincipal;
use crate::classes::entite::inventaire::Inventaire;
use crate::classes::marchandage::{affaire::Affaire};
use crate::classes::affichage::affichage_deplacement::AffichageDeplacement;

/// Structure représentant un magasin qui propose des affaires à l'achat.
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Magasin {
    pub affaires: Vec<Affaire>, // Liste des affaires disponibles
    pub phrase_arrive: Vec<String>,
}
#[allow(dead_code)]
impl Magasin {
    /// Constructeur pour créer un nouveau magasin avec une liste d'affaires.
    ///
    /// # Arguments
    /// - `affaires`: Un vecteur contenant les objets et leurs prix/quantités.
    pub fn new(affaires: Vec<Affaire>, phrase_arrive: Vec<String>) -> Magasin {
        Magasin { affaires, phrase_arrive }
    }

    /// Retourne une référence à la liste des affaires du magasin.
    pub fn get_affaires(&self) -> &Vec<Affaire> {
        &self.affaires
    }

    /// Permet d'ajouter une affaire au magasin.
    ///
    /// # Arguments
    /// - `affaire`: Une affaire à ajouter à la liste.
    pub fn ajouter_affaire(&mut self, affaire: Affaire) {
        self.affaires.push(affaire);
    }

    /// Méthode pour acheter un objet spécifique depuis le magasin.
    ///
    /// # Arguments
    /// - `index`: L'index de l'affaire dans la liste des affaires.
    /// - `inventaire`: Une référence mutable à l'inventaire de l'acheteur.
    ///
    /// # Retourne
    /// - `Ok(())` si l'achat est réussi.
    /// - `Err(&'static str)` en cas d'échec (ex : fonds insuffisants, stock épuisé).
    pub fn acheter(&mut self, index: usize, inventaire: &mut Inventaire) -> Result<(), &'static str> {
        if index >= self.affaires.len() {
            return Err("Affaire non valide.");
        }

        let affaire = &mut self.affaires[index];

        // Vérifier si l'inventaire a assez de monnaie
        if inventaire.get_monnaie() < *affaire.get_prix() {
            return Err("Fonds insuffisants.");
        }

        // Vérifier la disponibilité de l'objet
        if !*affaire.get_infini() && *affaire.get_quantite() == 0 {
            return Err("Stock épuisé.");
        }

        // Déduire le prix de l'argent de l'inventaire
        inventaire.remove_monnaie(*affaire.get_prix());

        // Ajouter l'objet à l'inventaire
        inventaire.add_objet(affaire.get_instance().clone());

        // Réduire la quantité, sauf si elle est infinie
        if !*affaire.get_infini() {
            affaire.set_quantite(*affaire.get_quantite() - 1);
        }

        Ok(())
    }

    /// Fonction pour acheter dans le magasin
    /// Fonction pour acheter dans le magasin
    pub fn interaction_magasin(&mut self, personnage: &mut PersonnagePrincipal) {
        AffichageDeplacement::lancer_animation("auberge", self.phrase_arrive.clone());

        loop {
            let affaires = self.get_affaires();
            println!("\n=== Bienvenue au magasin ===");
            println!("Monnaie actuelle : {} pièces", personnage.inventaire.get_monnaie());
            println!("Voici les objets disponibles :");

            for (i, affaire) in affaires.iter().enumerate() {
                println!(
                    "[{}] {} - {} pièces - Quantité : {}{}",
                    i,
                    affaire.get_instance().get_nom(),
                    affaire.get_prix(),
                    affaire.get_quantite(),
                    if *affaire.get_infini() { " (infini)" } else { "" }
                );
            }

            println!("Entrez l'index de l'objet à acheter ou 'q' pour quitter le magasin :");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.eq_ignore_ascii_case("q") {
                println!("Merci de votre visite !");
                break;
            }

            let index: usize = match input.parse() {
                Ok(i) if i < affaires.len() => i,
                _ => {
                    println!("Entrée invalide. Veuillez réessayer.");
                    continue;
                }
            };

            match self.acheter(index, &mut personnage.inventaire) {
                Ok(_) => println!("Achat réussi."),
                Err(e) => println!("Achat échoué : {}", e),
            }
        }
    }

}
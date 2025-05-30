use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::charger_partie::ChargerPartie;
use crate::classes::gestion_evenement::choix::Choix;
use crate::classes::gestion_evenement::quitter_jeu::QuitterJeu;
use crate::classes::sauvegarde::sauvegarde::Sauvegarde;
use crate::classes::structure_json::personnage_principal_json::PersonnagePrincipalJson;

pub struct BoucleJeu {
  personnage: PersonnagePrincipalJson,
}

impl BoucleJeu {
  pub fn new(nouvelle_partie: bool) -> BoucleJeu {
    let sauvegarde: Sauvegarde = Sauvegarde::new();
    if nouvelle_partie {
      AfficheTexte::affiche("Création d'une nouvelle partie...".to_string(), 30);
      let personnage: PersonnagePrincipalJson = sauvegarde.charge("nouveau_personnage.json".to_string()).unwrap();
      sauvegarde.sauvegarde("personnage_principal.json".to_string(), &personnage).unwrap();
      BoucleJeu {
        personnage,
      }
    } else {
      AfficheTexte::affiche("Chargement de la partie sauvegardée...".to_string(), 30);
      let personnage: PersonnagePrincipalJson = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
      BoucleJeu {
        personnage,
      }
    }
  }

  pub fn boucleJeu(&mut self) {
    let sauvegarde: Sauvegarde = Sauvegarde::new();
    let nbrUraniumDemande: u32 = 10;
    let mut jeuEnCours: bool = true;
    
    while jeuEnCours && self.personnage.get_pv() > 0 && self.personnage.get_uranium() < nbrUraniumDemande {
      AfficheTexte::affiche(format!("PV: {}, nom: {}", self.personnage.get_pv(), self.personnage.get_nom()), 30);

      //Lancer l'événement/planete 
      
      // AfficheTexte::affiche("Sauvegarde automatique...".to_string(), 30);
      // sauvegarde.sauvegarde("personnage_principal.json".to_string(), &self.personnage).unwrap();
    }

    if self.personnage.get_pv() == 0 {
      AfficheTexte::affiche("Vous êtes mort !".to_string(), 30);
      let choix_mort = Choix::new(vec![
        ("Charger la dernière sauvegarde".to_string(), Box::new(ChargerPartie::new())),
        ("Quitter".to_string(), Box::new(QuitterJeu::new()))
      ]);
      choix_mort.lancer_choix();
    } 
    else if self.personnage.get_uranium() >= nbrUraniumDemande {
      AfficheTexte::affiche("Vous avez gagné !!!!!!!".to_string(), 50);
    }
  }
}
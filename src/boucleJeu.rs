pub struct BoucleJeu {
  jeuEnCours: bool = true,
  nombreUraniumDemande: u32 = 10,
  // servicePersonnage:
  // serviceEvenement: 
  // serviceItem:
  // servicePlanete:
  // etc ....
}

impl BoucleJeu {
  pub fn new() -> BoucleJeu {
    BoucleJeu {
      jeuEnCours: true
    }
  }

  pub fn chargerDonnees (self) {
    
  }




  pub fn boucleJeu(self, nouvellePartie: bool) -> bool {
    let mut joueur = Joueur::new();
    let evenement: String;
    if nouvellePartie {
      evenement = self.creerPartie(&joueur);
    } else {
      evenement = self.sauvegarde.chargePartie(&joueur);
    }
    
    self.chargerDonnees();
    let mut evenementActuel = self.serviceEvenement.chargerEvenement(evenement);
    while jeuEnCours || joueur.getNombreUranium() < nombreUraniumDemande {
      let resultat = evenementActuel.action(joueur);
      evenementActuel = self.serviceEvenement.chargerEvenement(resultat);
    }
    if !jeuEnCours {
      self.sauvegarde.sauvegardePartie(joueur);
      false
    }
    true
  }
}

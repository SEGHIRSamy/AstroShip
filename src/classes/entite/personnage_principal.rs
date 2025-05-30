use std::fs::File;
use std::io::{Read, Write};
use crate::classes::entite::entite::{Entite, Personnage};
use crate::classes::entite::inventaire::Inventaire;
use crate::classes::marchandage::objet::Objet;
use crate::classes::structure_json::personnage_principal_json::{InventaireJson, ObjetJson, PersonnagePrincipalJson};

#[allow(dead_code)]
pub struct PersonnagePrincipal {
    pub entite: Entite,      // Composition, réutilise `Entite`
    pub inventaire: Inventaire, // Ajout d'un inventaire
    pub chance: u32,             // Nouvelle statistique
}

#[allow(dead_code)]
impl PersonnagePrincipal {
    pub fn new(
        nom: &str,
        points_de_vie: u32,
        points_de_vie_max: u32,
        force: u32,
        intelligence: u32,
        vitesse: u32,
        chance: u32,
    ) -> Self {
        Self {
            entite: Entite::new(nom, points_de_vie, points_de_vie_max, force, intelligence, vitesse),
            inventaire: Inventaire::new(),
            chance,
        }
    }

    pub fn charger_personnage_principal_depuis_json() -> std::io::Result<PersonnagePrincipal> {
        let mut fichier = File::open("JSON/personnage_principal.json")?;
        let mut contenu = String::new();
        fichier.read_to_string(&mut contenu)?;

        let donnees: PersonnagePrincipalJson = serde_json::from_str(&contenu)?;

        let mut inventaire = Inventaire::new();
        inventaire.set_monnaie(donnees.inventaire.monnaie as u32);
        for obj in donnees.inventaire.objets {
            inventaire.add_objet(Objet::new(&obj.nom, &obj.description, obj.quantite));
        }

        Ok(PersonnagePrincipal {
            entite: Entite::new(
                &donnees.nom,
                donnees.points_de_vie,
                donnees.points_de_vie_max,
                donnees.force,
                donnees.intelligence,
                donnees.vitesse,
            ),
            inventaire,
            chance: donnees.chance,
        })
    }

    pub fn sauvegarder_personnage_principal_vers_json(perso: &PersonnagePrincipal) -> std::io::Result<()> {
        let objets: Vec<ObjetJson> = perso.inventaire.get_instance()
            .iter()
            .map(|o| ObjetJson {
                nom: o.get_nom().to_string(),
                description: o.get_description().to_string(),
                quantite: o.get_quantite() ,
            })
            .collect();

        let donnees = PersonnagePrincipalJson {
            nom: perso.entite.get_nom().to_string(),
            points_de_vie: perso.entite.get_points_de_vie(),
            points_de_vie_max: perso.entite.get_points_de_vie_max(),
            force: perso.entite.get_force(),
            intelligence: perso.entite.get_intelligence(),
            vitesse: perso.entite.get_vitesse(),
            chance: perso.chance,
            uranium: 0,
            inventaire: InventaireJson {
                monnaie: perso.inventaire.get_monnaie() ,
                objets,
            },
        };

        let json = serde_json::to_string_pretty(&donnees)?;
        let mut fichier = File::create("JSON/personnage_principal.json")?;
        fichier.write_all(json.as_bytes())?;
        Ok(())
    }

}

#[allow(dead_code)]
impl Personnage for PersonnagePrincipal {
    fn augmentation_niveau(&mut self, choix_statistique: &str) {
        if choix_statistique == "chance" {
            self.chance += 1;
            println!("Votre chance a augmenté !");
        } else {
            // Appelle `augmentation_niveau` de l'entité sous-jacente
            self.entite.augmentation_niveau(choix_statistique);
        }
    }

    fn afficher_statistiques(&self) {
        self.entite.afficher_statistiques();
        println!("Chance : {}", self.chance);

        println!("\n=== Inventaire ===");
        self.inventaire.afficher_inventaire(); // Affiche le contenu de l'inventaire
    }

}


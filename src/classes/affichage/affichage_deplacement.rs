use std::{io, thread, time};

use rand::{prelude::IndexedRandom, rng};

use crate::classes::affichage::affiche_texte::AfficheTexte;

// On définit les diffénrents assets pour les animations
const STICKMAN: [&str; 6] = [
    "",
    "",
    "",
    "   O  ",
    "  /|\\ ",
    "  / \\ ",
];

const VAISSEAU: [&str; 6] = [
    "        |        ",
    "       -+-       ",
    "      /-|-\\      ",
    "      | O |      ",
    "      |   |      ",
    "     /-----\\     ",
];

const PLANETE: [&str; 4] = [
    "       _____       ",
    "    .-'     '-.    ",
    "  .'  PLANETE  '.  ",
    "  '-._______,-'    ",
];

const GROTTE: [&str; 6] = [
    "       ________      ",
    "     /          \\   ",
    "    /            \\  ",
    "   |              | ",
    "   |     ____     | ",
    "   |____|    |____| ",
];

const MAGASIN: [&str; 6] = [
    "    ____________   ",
    "   |   ____     |  ",
    "   |  |    |    |  ",
    "   |  |____|    |  ",
    "   |            |  ",
    "   |____________|  ",
];

const AUBERGE: [&str; 6] = [
    "       ________    ",
    "      |  ____  |   ",
    "     /| |    | |\\  ",
    "    || | [] | || | ",
    "    || |____| || | ",
    "    ||________||_| ",
];

pub struct AffichageDeplacement {
}

impl AffichageDeplacement {
    /// Retourne une représentation ASCII d’un lieu donné sous forme de tableau de 6 lignes.
    ///
    /// # Arguments
    /// - `destination`: Le nom du lieu (par exemple "magasin", "auberge", "zone hostile").
    ///
    /// # Returns
    /// - `Some(&[&str; 6])` : Si le lieu est reconnu.
    /// - `None` : Si le lieu n'est pas défini.
    pub fn get_lieu_ascii(destination: &str) -> Option<&'static [&'static str; 6]> {
        match destination {
            "zone hostile" => Some(&GROTTE),
            "magasin" => Some(&MAGASIN),
            "auberge" => Some(&AUBERGE),
            _ => None,
        }
    }

    /// Affiche une phrase aléatoire parmi une liste, avec un effet de délai.
    ///
    /// # Arguments
    /// - `phrases`: Un vecteur de chaînes de caractères contenant les phrases possibles.
    ///
    /// # Comportement
    /// - Sélectionne une phrase aléatoire.
    /// - L'affiche avec une animation (délai entre caractères + pause).
    ///
    /// # Panics
    /// - Ne panique pas directement, mais ne fait rien si la liste est vide.
    ///
    /// # Remarque
    /// - Cette fonction utilise un RNG (aléatoire), assure-toi que le crate `rand` est importé correctement.
    fn phrase_random(phrases: Vec<String>) {
        if let Some(phrase) = phrases.choose(&mut rng()) {
            AfficheTexte::affiche(phrase.to_string(), 10);
            thread::sleep(time::Duration::from_millis(1500));
        }
    }

    /// Affiche une frame ASCII combinant un lieu et un personnage (stickman) à une position donnée.
    ///
    /// # Arguments
    /// - `lieu`: Un tableau de 6 lignes ASCII représentant un environnement.
    /// - `position`: La position horizontale du personnage.
    ///
    /// # Comportement
    /// - Affiche chaque ligne du décor.
    /// - Ajoute le personnage à la bonne position sur chaque ligne.
    pub fn afficher_frame(lieu: &[&str; 6], position: usize) {
        for i in 0..lieu.len() {
            if i < STICKMAN.len() {
                println!("{}{}{}", lieu[i], " ".repeat(position), STICKMAN[i]);
            } else {
                println!("{}", lieu[i]);
            }
        }
    }

    /// Lance une animation spatiale simulant le départ ou l'arrivée d'une fusée sur une planète.
    ///
    /// # Arguments
    /// - `destination`: Soit `"depart"`, soit `"arrivee"`.
    /// - `phrase_arrive`: Un vecteur de phrases aléatoires à afficher à la fin.
    ///
    /// # Comportement
    /// - Affiche une animation console en ASCII simulant le mouvement.
    /// - Affiche ensuite une phrase aléatoire si disponible.
    ///
    /// # Erreurs
    /// - Affiche un message si la destination est inconnue.
    pub fn lancer_animation_spatiale(destination: &str, phrase_arrive: Vec<String>) {
        let frames = 20;
        let delay = time::Duration::from_millis(100);

        match destination {
            "arrivee" => {
                for step in 0..frames {
                    print!("\x1B[2J\x1B[1;1H");

                    // Afficher la planète au bas de l'écran
                    let vide_ligne = 25 - PLANETE.len() - step;
                    for _ in 0..vide_ligne {
                        println!();
                    }

                    for line in PLANETE.iter() {
                        println!("{}", line);
                    }

                    // Afficher la fusée qui monte
                    if step < 20 {
                        for _ in 0..(20 - step) {
                            println!();
                        }

                        for line in VAISSEAU.iter() {
                            println!("{}", line);
                        }
                    }

                    thread::sleep(delay);
                }

                AffichageDeplacement::phrase_random(phrase_arrive);
            }

            "depart" => {
                for step in (0..frames).rev() {
                    print!("\x1B[2J\x1B[1;1H");

                    // Fusée descend
                    for _ in 0..(step + 2) {
                        println!();
                    }

                    for line in VAISSEAU.iter() {
                        println!("{}", line);
                    }

                    // Planète fixe en bas
                    let vide_ligne = 25usize.saturating_sub(PLANETE.len() + step + VAISSEAU.len());
                    for _ in 0..vide_ligne {
                        println!();
                    }

                    for line in PLANETE.iter() {
                        println!("{}", line);
                    }

                    thread::sleep(delay);
                }

                println!("\n🚀 La fusée a quitté l'orbite !");
            }

            _ => {
                println!("Commande spatiale inconnue : utilisez \"depart\" ou \"arrivee\"");
            }
        }
    }

    /// Lance une animation console représentant un déplacement dans une zone d’une planète (auberge, magasin...).
    ///
    /// # Arguments
    /// - `destination`: Nom du lieu (doit être reconnu par `get_lieu_ascii`).
    /// - `phrase_arrive`: Une liste de phrases à afficher après l'animation.
    ///
    /// # Comportement
    /// - Affiche une animation de transition du stickman jusqu’au lieu cible.
    /// - Affiche une phrase d’arrivée aléatoire.
    ///
    /// # Erreurs
    /// - Affiche "Destination inconnue" si le lieu n’est pas reconnu.
    pub fn lancer_animation(destination: &str, phrase_arrive: Vec<String>) {
        let lieu_ascii = match AffichageDeplacement::get_lieu_ascii(destination) {
            Some(l) => l,
            None => {
                println!("Destination inconnue.");
                return;
            }
        };

        for pos in (0..=25).rev() {
            print!("\x1B[2J\x1B[1;1H");
            AffichageDeplacement::afficher_frame(lieu_ascii, pos);
            io::Write::flush(&mut io::stdout()).unwrap();
            thread::sleep(time::Duration::from_millis(120));
        }

        AffichageDeplacement::phrase_random(phrase_arrive);

        println!("\nLe personnage est arrivé à la {} !", destination);
    }
}
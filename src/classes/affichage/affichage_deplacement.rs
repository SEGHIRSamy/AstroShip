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
    // Retourne l'asset correspondant au lieux voulu
    pub fn get_lieu_ascii(destination: &str) -> Option<&'static [&'static str; 6]> {
        match destination {
            "zone hostile" => Some(&GROTTE),
            "magasin" => Some(&MAGASIN),
            "auberge" => Some(&AUBERGE),
            _ => None,
        }
    }

    // Renvoie une phrase aléatoire
    fn phrase_random(phrases: Vec<String>) {
        if let Some(phrase) = phrases.choose(&mut rng()) {
            AfficheTexte::affiche(phrase.to_string(), 10);
            thread::sleep(time::Duration::from_millis(1500));
        }
    }

    // Affiche une frame de l'animation
    pub fn afficher_frame(lieu: &[&str; 6], position: usize) {
        for i in 0..lieu.len() {
            if i < STICKMAN.len() {
                println!("{}{}{}", lieu[i], " ".repeat(position), STICKMAN[i]);
            } else {
                println!("{}", lieu[i]);
            }
        }
    }

    // Permet de lancer une animation de transition entre les planetes
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

    // Permet de lancer une animation de transition entre zone d'une planete
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
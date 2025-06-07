use std::{io, thread, time};

use rand::{prelude::IndexedRandom, rng};

use crate::classes::{affichage::affiche_texte::AfficheTexte, sauvegarde::sauvegarde::Sauvegarde};

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
    pub fn get_lieu_ascii(destination: &str) -> Option<&'static [&'static str; 6]> {
        match destination {
            "zone hostile" => Some(&GROTTE),
            "magasin" => Some(&MAGASIN),
            "auberge" => Some(&AUBERGE),
            _ => None,
        }
    }

    fn phrase_random(destination: &str) {
        let chargeur: Sauvegarde = Sauvegarde::new();
        let phrases_zone_hostile: Vec<String> = chargeur.charge("phrase_lieux/zone_hostile.json".to_string()).unwrap();

        let phrases_magasin: Vec<String>      = chargeur.charge("phrase_lieux/magasin.json".to_string()).unwrap();

        let phrases_auberge: Vec<String>      = chargeur.charge("phrase_lieux/auberge.json".to_string()).unwrap();

        let phrases = match destination {
            "zone hostile" => &phrases_zone_hostile,
            "magasin" => &phrases_magasin,
            "auberge" => &phrases_auberge,
            _ => {
                println!("Pas de phrases disponibles pour ce lieu.");
                return;
            }
        };

        if let Some(phrase) = phrases.choose(&mut rng()) {
            AfficheTexte::affiche(phrase.to_string(), 10);
            thread::sleep(time::Duration::from_millis(1500));
        }

    }

    pub fn afficher_frame(lieu: &[&str; 6], position: usize) {
        for i in 0..lieu.len() {
            if i < STICKMAN.len() {
                println!("{}{}{}", lieu[i], " ".repeat(position), STICKMAN[i]);
            } else {
                println!("{}", lieu[i]);
            }
        }
    }

    pub fn lancer_animation_spatiale(destination: &str) {
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

                println!("\n🌍 La fusée a atterri sur la planète !");
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

    pub fn lancer_animation(destination: &str) {
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

        AffichageDeplacement::phrase_random(destination);

        println!("\nLe personnage est arrivé à la {} !", destination);
    }
}
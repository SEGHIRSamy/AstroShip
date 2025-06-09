mod classes;

use crate::classes::affichage::intro::Intro;
use crate::classes::gestion_evenement::evenement::Evenement;

fn main() {
    Intro::new().action();
}

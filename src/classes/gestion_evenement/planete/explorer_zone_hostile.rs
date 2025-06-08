use crate::classes::gestion_evenement::evenement::Evenement;
use crate::classes::planete::zone_hostile::ZoneHostile;

pub struct ExplorerZoneHostile<'a> {
    zone_hostile: &'a mut ZoneHostile
}

impl<'a> ExplorerZoneHostile<'a> {
    pub fn new(zone_hostile: &'a mut ZoneHostile) -> Self { Self {zone_hostile}}
}

impl<'a> Evenement for ExplorerZoneHostile<'a> {
    fn action(&mut self) {
        self.zone_hostile.explorer();
    }
}
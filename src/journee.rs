use serde::Deserialize;
use std::cmp::Ordering;
use std::ops::Add;

use crate::date::Date;

#[derive(Clone, Debug, Deserialize)]
// une journée est une rencontre
pub struct Journee {
    // la date de la rencontre
    pub date: Date,
    // l'ensemble des matchs individuels
    matchs: Vec<Match>,
}

#[derive(Clone, Debug, Deserialize)]
// représente une match individuel
pub struct Match {
    // le nom de l'adversaire
    // nom: String,
    // le coefficient multiplicateur des points
    // coeff: f32,
    // les points gagnés (coefficient compris)
    ex: f32,
}

impl Journee {
    // indique si une rencontre fait partie de l'année donnée
    pub fn est_saison(&self, annee: u16) -> bool {
        (self.date.annee == annee && self.date.mois >= 9)
            || (self.date.annee - 1 == annee && self.date.mois <= 6)
    }

    // calcule les points obtenus durant une rencontre
    pub fn ex(&self) -> f32 {
        self.matchs.iter().fold(0.0, |x, y| x + y.ex)
    }
}

impl Ord for Journee {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Journee {}

impl PartialOrd for Journee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

impl PartialEq for Journee {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }
}

// la somme de 2 journées
impl Add for Journee {
    type Output = Journee;
    fn add(self, other: Journee) -> Self {
        Journee {
            date: self.date,
            matchs: self.matchs.into_iter().chain(other.matchs).collect(),
        }
    }
}

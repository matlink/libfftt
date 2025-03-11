use serde::Deserialize;
use std::ops::Add;

use crate::journee::Journee;

#[derive(Clone, Default, Debug, Deserialize)]
// une partie représente l'ensemble des rencontres d'une saison
pub struct Partie {
    // le nombre de matchs validés
    #[allow(dead_code)]
    processed: u8,
    num_matchs: u8,
    // les points gagnés sur ces matchs
    ex: f32,
    // les rencontres
    pub journees: Vec<Journee>,
}

// la somme de 2 parties
impl Add for Partie {
    type Output = Partie;
    fn add(self, other: Partie) -> Self {
        Partie {
            num_matchs: self.num_matchs + other.num_matchs,
            ex: self.ex + other.ex,
            journees: self.journees.into_iter().chain(other.journees).collect(),
            ..self
        }
    }
}

impl Partie {
    // la somme d'un ensemble de parties
    pub fn sum_parties(parties: &[Partie]) -> Partie {
        parties.iter().fold(
            Partie {
                processed: 0,
                num_matchs: 0,
                ex: 0.0,
                journees: Vec::new(),
            },
            |x, y| x + y.clone(),
        )
    }

    // regrouper les journées d'une partie
    pub fn regrouper_journees(mut self) -> Option<Partie> {
        let mut journees: Vec<Journee> = Vec::new();
        self.journees.sort();
        for j in &self.journees {
            match journees.len() {
                0 => journees.push(j.clone()),
                _ => {
                    if j.date == journees.last()?.date {
                        let autre_journee = journees.pop()?;
                        journees.push(autre_journee + j.clone());
                    } else {
                        journees.push(j.clone());
                    }
                }
            }
        }
        self.journees = journees;
        Some(self)
    }
}

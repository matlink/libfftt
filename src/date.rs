use chrono::{Datelike, Local};
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use std::{cmp::Ordering, fmt};

use super::{DEBSAISON_JOUR, DEBSAISON_MOIS, MISAISON_JOUR, MISAISON_MOIS};

#[derive(Clone, Debug, Serialize)]
pub struct Date {
    pub annee: u16,
    pub mois: u8,
    pub jour: u8,
}

impl FromStr for Date {
    type Err = ();

    /// Crée une date à partir de la chaîne au format JJ/MM/AA
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split('/').collect();
        Ok(Date {
            jour: splitted[0].parse().expect("Le jour doit être un nombre"),
            mois: splitted[1].parse().expect("Le mois doit être un nombre"),
            annee: splitted[2].parse().expect("L'année doit être un nombre"),
        })
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Ok(Date::from_str(s).unwrap())
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#02}/{:#02}/{}", self.jour, self.mois, self.annee)
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.annee.cmp(&other.annee) {
            Ordering::Equal => match self.mois.cmp(&other.mois) {
                Ordering::Equal => self.jour.cmp(&other.jour),
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.jour == other.jour && self.mois == other.mois && self.annee == other.annee
        // self.annee == other.annee && self.mois == other.mois && self.jour == other.jour
    }
}

impl std::hash::Hash for Date {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_i32(i32::from(
            self.annee + u16::from(self.mois) + u16::from(self.jour),
        ));
        let _ = state.finish();
    }
}

impl Date {
    #[cfg(test)]
    pub fn new(annee: u16, mois: u8, jour: u8) -> Date {
        Date { annee, mois, jour }
    }
    /// Retourne la date d'aujourd'hui
    pub fn now() -> Date {
        let now = Local::now();
        Date {
            annee: u16::try_from(now.year()).expect("Year must be a u16 number"),
            mois: u8::try_from(now.month()).expect("Month must be a u8 number"),
            jour: u8::try_from(now.day()).expect("Day must be a u8 number"),
        }
    }

    /// Retourne si la date appartient à la phase 2
    pub fn phase2(&self) -> bool {
        // entre février et août
        (MISAISON_MOIS < self.mois && self.mois < DEBSAISON_MOIS)
        // fin janvier
            || (self.mois == MISAISON_MOIS && MISAISON_JOUR <= self.jour)
            // début septembre
            || (self.mois == DEBSAISON_MOIS && self.jour < DEBSAISON_JOUR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_phase2() {
        let d = Date::new(2023, 1, 1);
        assert!(!d.phase2());
        let d = Date::new(2023, 1, 11);
        assert!(d.phase2());
        let d = Date::new(2023, 1, 15);
        assert!(d.phase2());
        let d = Date::new(2023, 9, 1);
        assert!(!d.phase2());
        let d = Date::new(2023, 9, 11);
        assert!(!d.phase2());
        let d = Date::new(2023, 9, 30);
        assert!(!d.phase2());
        let d = Date::new(2024, 2, 1);
        assert!(d.phase2());
        let d = Date::new(2024, 2, 10);
        assert!(d.phase2());
        let d = Date::new(2024, 2, 15);
        assert!(d.phase2());
        let d = Date::new(2024, 3, 1);
        assert!(d.phase2());
        let d = Date::new(2024, 8, 31);
        assert!(d.phase2());
        let d = Date::new(2024, 9, 10);
        assert!(!d.phase2());
        let d = Date::new(2024, 9, 11);
        assert!(!d.phase2());
    }

    #[test]
    fn test_order() {
        let d_1 = Date::new(2023, 1, 1);
        let d_2 = Date::new(2023, 1, 2);
        assert!(d_1 < d_2);
        let d_1 = Date::new(2022, 1, 2);
        let d_2 = Date::new(2023, 1, 1);
        assert!(d_1 < d_2);
        let d_1 = Date::new(2023, 1, 1);
        let d_2 = Date::new(2023, 1, 1);
        assert!(d_1 == d_2);
        let d_1 = Date::new(2023, 1, 1);
        let d_2 = Date::new(2022, 12, 31);
        assert!(d_1 > d_2);
    }

    #[test]
    fn bench_order() {
        let d_1 = Date::new(2023, 1, 1);
        let d_2 = Date::new(2023, 1, 31);
        let before = Instant::now();
        for _ in 1..1_000_000_000 {
            assert!(d_1 < d_2);
        }
        println!("Elapsed time: {:.2?}", before.elapsed());
    }
}

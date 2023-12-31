use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Quirk {
    NeverEats,
    NeverSleeps,
    NeverGoesFast,
    NeverGoesDirectly,
    NeverLooksAtCamera,
    NeverHappy,
    NeverAngry,
    NeverBored,
    SometimesMischivous,
    NeverSitsOnNest,
    NeverScared,
    NeverExcited,
    Loner
}

pub fn get_n_random_quirks(n: usize) -> Vec<Quirk> {
    let mut rng = rand::thread_rng();
    let mut quirks: Vec<Quirk> = vec![
        Quirk::NeverEats,
        Quirk::NeverSleeps,
        Quirk::NeverGoesFast,
        Quirk::NeverGoesDirectly,
        Quirk::NeverLooksAtCamera,
        Quirk::NeverHappy,
        Quirk::NeverAngry,
        Quirk::NeverBored,
        Quirk::SometimesMischivous,
        Quirk::NeverSitsOnNest,
        Quirk::Loner
    ];
    quirks.shuffle(&mut rng);
    quirks.truncate(n);
    quirks
}

pub fn get_quirk_description(quirk: Quirk) -> String {
    match quirk {
        Quirk::NeverEats => vec![
            "Never eats",
            "Doesn't eat",
            "Is never hungry",
            "Is currently fasting",
            "#fasting"
        ],
        Quirk::NeverSleeps => vec![
            "Never sleeps",
            "Doesn't sleep",
            "Is never tired",
        ],
        Quirk::NeverGoesFast => vec![
            "Never speeds up",
            "Always walks slowly",
        ],
        Quirk::NeverGoesDirectly => vec![
            "Always takes the long way",
        ],
        Quirk::NeverLooksAtCamera => vec![
            "Is camera shy and never looks at it",
            "Won't look at you",
        ],
        Quirk::NeverHappy => vec![
            "Never smiles :(",
            "Is never happy",
            "Depressed"
        ],
        Quirk::NeverAngry => vec![
            "Never angry",
            "Doesn't get angry"
        ],
        Quirk::NeverBored => vec![
            "Is never bored",
            "Never gets bored",
        ],
        Quirk::SometimesMischivous => vec![
            "Sometimes has a creepy smile",
            "Sometimes has an evil smile",
        ],
        Quirk::NeverSitsOnNest => vec![
            "Hates laying eggs",
            "Does not plan to have children",
            "#childfree",
        ],
        Quirk::NeverScared => vec![
            "Never gets scared",
            "Is never scared",
            "Is fearless",
            "Is not afraid of anything",
        ],
        Quirk::NeverExcited => vec![
            "Is never excited about anything",
        ],
        Quirk::Loner => vec![
            "Does not like to be in the centre of attention",
            "Sometimes strays further from the farm"
        ],
    }.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn annotate_quirks(quirks: Vec<Quirk>) -> Vec<(Quirk, String)> {
    quirks.iter().map(|quirk| (*quirk, get_quirk_description(*quirk))).collect()
}

#[allow(dead_code)]
pub fn generate_annotated_quirks(n: usize) -> Vec<(Quirk, String)> {
    annotate_quirks(get_n_random_quirks(n))
}
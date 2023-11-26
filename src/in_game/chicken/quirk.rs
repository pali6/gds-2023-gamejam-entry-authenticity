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
        Quirk::SometimesMischivous
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
            "Is never hungry"
        ],
        Quirk::NeverSleeps => vec![
            "Never sleeps",
            "Doesn't sleep",
            "Is never tired"
        ],
        Quirk::NeverGoesFast => vec![
            "Doesn't rush anywhere",
            "Never speeds up",
            "Is never in a hurry"
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
            "Is never happy"
        ],
        Quirk::NeverAngry => vec![
            "Never angry"
        ],
        Quirk::NeverBored => vec![
            "Is never bored",
            "Very emotional"
        ],
        Quirk::SometimesMischivous => vec![
            "Sometimes has a creepy smile",
            "Not so innocent"
        ],
    }.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn annotate_quirks(quirks: Vec<Quirk>) -> Vec<(Quirk, String)> {
    quirks.iter().map(|quirk| (*quirk, get_quirk_description(*quirk))).collect()
}

pub fn generate_annotated_quirks(n: usize) -> Vec<(Quirk, String)> {
    annotate_quirks(get_n_random_quirks(n))
}
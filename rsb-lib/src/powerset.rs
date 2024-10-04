pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    set.iter().fold(vec![vec![]], |mut powerset, value| {
        let new = powerset.clone().into_iter().map(|mut set| {
            set.push(*value);
            set
        });
        powerset.extend(new);
        powerset
    })
}

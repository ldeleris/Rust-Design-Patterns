//! This is documentation for the `functional` crate.
//!

pub mod monoids;
pub mod functors;
pub mod monads;

pub fn monoids() {
    use monoids::*;

    println!("-------");
    println!("Monoids");
    println!("-------");

    let strings =
        vec![String::from("This is\n"), String::from("a list of\n"), String::from("strings!")];
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let res1 = strings.clone().into_iter().fold(StringConcatenation.zero(),
                                                |acc, x| StringConcatenation.op(acc, x));
    println!("{}", res1);

    let res1 =
        numbers.clone().into_iter().fold(IntAddition.zero(), |acc, x| IntAddition.op(acc, x));
    println!("{:?} (+): {}", numbers.clone(), res1);

    let res1 = numbers.clone().into_iter().fold(IntMultiplication.zero(),
                                                |acc, x| IntMultiplication.op(acc, x));
    println!("{:?} (*): {}", numbers.clone(), res1);

    let res1 = IntAddition::fold(numbers.clone());
    println!("{}", res1);

    println!("{:?} (+): {}",
             numbers.clone(),
             MOpt::fold(numbers.clone(), &IntAddition));
    println!("{:?} (*): {}",
             numbers.clone(),
             MOpt::fold(numbers.clone(), &IntMultiplication));
    println!("{:?} (++): {}",
             strings.clone(),
             MOpt::fold(strings.clone(), &StringConcatenation));

    println!("{:?} : {}",
             numbers.clone(),
             MOpt::fold_map(numbers.clone(), &StringConcatenation, |x| format!("{}", x)));

    let strings = vec![String::from("1"),
                       String::from("2"),
                       String::from("3"),
                       String::from("4"),
                       String::from("5"),
                       String::from("6")];

    println!("{:?} : {}",
             strings.clone(),
             MOpt::fold_map(strings.clone(), &IntAddition, |x| x.parse().unwrap()));

    println!("4! is: {}",
             MOpt::balance_fold(vec![1, 2, 3, 4], &IntMultiplication, MOpt::identity));
}


pub fn functors() {
    use functors::*;

    println!("--------");
    println!("Functors");
    println!("--------");
}


pub fn monads() {
    use monads::*;

    println!("------");
    println!("Monads");
    println!("------");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::{str::FromStr, io};
use std::fmt;
use fmt::Debug;
use rand::prelude::*;


const SIZE:usize = 5;


#[derive(Debug, PartialEq, Copy, Clone)]
enum Pawn {
    Red,
    Green,
    Yellow,
    Blue,
    Pink,
    Empty
}

impl Pawn {
    // Generate a random new game with unique colors
    fn new_random_game()->Pattern {
        let mut p:Pattern = [Self::Empty; SIZE];      
        let mut rng = thread_rng();
        let iter = 
            [Self::Red, Self::Green, Self::Yellow, Self::Blue, Self::Pink]
            .choose_multiple(&mut rng, SIZE).enumerate();
    
        for (i, c) in iter {
            p[i] = *c;
        }
        p
    }
}

impl std::str::FromStr for Pawn {
    type Err = String;    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Pawn::Blue),
            "red" => Ok(Pawn::Red),
            "green" => Ok(Pawn::Green),
            "yellow" => Ok(Pawn::Yellow),
            "pink" => Ok(Pawn::Pink),
            _ => Err(format!("Color:{} do not exists!", s))
        }
    }
}

impl fmt::Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Pawn::Blue => write!(f, "blue"),
            &Pawn::Red => write!(f, "red"),
            &Pawn::Green => write!(f, "green"),
            &Pawn::Yellow => write!(f, "yellow"),
            &Pawn::Pink => write!(f, "pink"),
            &Pawn::Empty => write!(f, "_")
        }
    }
}

type Pattern=[Pawn; SIZE];


// Create a new game
// Wait for proposition like color-color-... (5 times)
// Replace unmatched colors with Pawn::Empty
// Return number of blacks and whites until proposition and solution match
fn main() {
    let input = &mut String::new();

    let game_solution = Pawn::new_random_game();

    'game: loop {
        input.clear();
        match io::stdin().read_line(input) {
            Ok(_n) => {
                
                let mut proposition:Pattern = [Pawn::Empty; SIZE];
                
                let v = split_into_type::<Pawn>(input, '-', Pawn::Empty);
            
                for (i, c)in v.iter().enumerate() {
                    if i==SIZE {break;}
                    proposition[i] = *c;
                }
                
                match check_matches(&game_solution, &proposition) {
                    (5, 0) => {
                        println!("You win!");
                        break 'game;
                    },
                    (b, w) => println!("{} black, {} white", b, w),
                }
                

            }
            Err(error) => println!("error: {}", error),
        }
    }
}

// Transform a String (T sep T sep ...) into an iterable of type T
fn split_into_type<T>(source: &str, pattern: char, default: T) -> Vec<T> where T:FromStr+Copy {
    source
        .trim()
        .split(pattern)
        .map(
            |s| s.parse().unwrap_or(default)
        )
        .collect()
}

// Search &T in a mutable reference of Vec<&T>
// Remove it from Vector if exists and return TRUE
// Return FALSE if not
fn remove_in_vector<T>(needle: &T, vec:&mut Vec<&T>) -> bool where T:PartialEq {
    match vec.iter().position(|e| *e == needle) {
        Some(i) => {vec.remove(i); true},
        None => false
    }
}

// Compare two Pattern, the proposition against the solution
// Return a tuple of:
// - Number of exact matches (color AND position) => black
// - Number of other matches (color BUT NOT position) => white
fn check_matches(solution:&Pattern, proposition:&Pattern)->(u8, u8) {
    
    let iter = solution.iter().zip(proposition.iter());
    let mut score_black_white:(u8, u8) = (0,0);
    let buffer_sol:&mut Vec<&Pawn> = &mut Vec::new();
    let buffer_prop:&mut Vec<&Pawn> = &mut Vec::new();
    
    for t in iter {
        match t {
            (sol, prop) if sol==prop => score_black_white.0 += 1,
            (sol, prop) => {

                match remove_in_vector(sol, buffer_prop) {
                    true => score_black_white.1 += 1,
                    false => buffer_sol.push(sol)
                }

                match remove_in_vector(prop, buffer_sol) {
                    true => score_black_white.1 += 1,
                    false => buffer_prop.push(prop)
                }

            }
        }
    };
    
    score_black_white
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_into_type() {
        assert_eq!(split_into_type::<Pawn>("blue-red", '-', Pawn::Empty), vec![Pawn::Blue, Pawn::Red]);
        assert_eq!(split_into_type::<Pawn>("blue-road", '-', Pawn::Empty), vec![Pawn::Blue, Pawn::Empty]);
        assert_eq!(split_into_type::<Pawn>("blueroad", '-', Pawn::Empty), vec![Pawn::Empty]);
        assert_eq!(split_into_type::<Pawn>("", '-', Pawn::Empty), vec![Pawn::Empty]);
    }

    #[test]
    fn test_new_random_game() {
        let game1 = Pawn::new_random_game();
        let game2 = Pawn::new_random_game();
        assert_ne!(game1, game2);
    }

    #[test]
    fn test_check_matches() {
        assert_eq!(
            check_matches(
                &[Pawn::Blue, Pawn::Green, Pawn::Yellow, Pawn::Red, Pawn::Pink],
                &[Pawn::Blue, Pawn::Green, Pawn::Yellow, Pawn::Red, Pawn::Pink]            
            ),
            (5, 0)
        );

        assert_eq!(
            check_matches(
                &[Pawn::Blue, Pawn::Green, Pawn::Pink, Pawn::Red, Pawn::Yellow],
                &[Pawn::Blue, Pawn::Yellow, Pawn::Green, Pawn::Red, Pawn::Pink]            
            ),
            (2, 3)
        );

        assert_eq!(
            check_matches(
                &[Pawn::Blue, Pawn::Green, Pawn::Pink, Pawn::Red, Pawn::Green],
                &[Pawn::Blue, Pawn::Yellow, Pawn::Green, Pawn::Red, Pawn::Yellow]            
            ),
            (2, 1)
        );

        assert_eq!(
            check_matches(
                &[Pawn::Blue, Pawn::Green, Pawn::Red, Pawn::Pink, Pawn::Yellow],
                &[Pawn::Green, Pawn::Yellow, Pawn::Blue, Pawn::Red, Pawn::Pink]            
            ),
            (0, 5)
        );

        assert_eq!(
            check_matches(
                &[Pawn::Blue, Pawn::Green, Pawn::Pink, Pawn::Yellow, Pawn::Yellow],
                &[Pawn::Green, Pawn::Red, Pawn::Red, Pawn::Red, Pawn::Pink]            
            ),
            (0, 2)
        );
    }

    #[test]
    fn test_remove_in_vector(){

        let vec = &mut vec![&1, &2, &3];
        
        assert_eq!(remove_in_vector(&2, vec), true);
        assert_eq!(*vec, vec![&1, &3]);

        let vec = &mut vec![&1, &2, &3];
        
        assert_eq!(remove_in_vector(&4, vec), false);
        assert_eq!(*vec, vec![&1, &2, &3]);
    }
}
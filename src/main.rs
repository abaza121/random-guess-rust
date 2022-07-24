use rand::Rng;
use std::io;
use std::cmp;

fn main() {

    let randomValue = rand::thread_rng().gen_range(0..=100);
    let mut isOver = false;
    while !isOver 
    {
        isOver = CompareAndShow(randomValue, GetGuess());
    } 
}

fn GetGuess() -> i32
{
    println!("Enter your guess");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read from stdin");
    let entered = input.trim()
                            .parse::<i32>()
                            .expect("Not a number!");
    println!("you entered {entered}");
    return entered;
}

fn CompareAndShow(value:i32, guessed:i32) -> bool
{
    let comparer = match value.cmp(&guessed)
    {
        cmp::Ordering::Equal => 
        {
            println!("wow you won!");
            return true;
        }
        cmp::Ordering::Less => println!("smaller"),
        cmp::Ordering::Greater => println!("bigger")
    };

    false
}

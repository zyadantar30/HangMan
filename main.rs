extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::io;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter
{
    character: char,
    revealed: bool
}

enum GameProgress
{
    InProgress,
    Won,
    Lost
}

fn main() 
{
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    println!("Welcome To HangMan..");

    loop
    {
        println!("\nYou have {} turns left" , turns_left);
        display_progress(&letters);

        println!("\nPlease enter a letter to guess:");
        let user_char = read_user_input_character();

        /*exit if user enters an '*' */
        if user_char == '*'
        {
            break;
        }
        /*update the revealed state of each letter if the user has gussed a correct letter */
        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut()
        {
            if letter.character == user_char
            {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }
        /*here the user loses a turn if gussed wrong */
        if at_least_one_revealed == false
        {
            turns_left -= 1;
        }
        /*check game progress */
        match check_progress(turns_left , &letters)
        {
            GameProgress::InProgress => continue,
            GameProgress::Won =>
            {
                println!("\nCongrats , You won! the word was {}" , selected_word);
                break;
            }
            GameProgress::Lost =>
            {
                println!("\nSorry you Lost!");
                break;
            }
        }
    }
    println!("GoodBye!")
}

fn select_word() -> String
{
    /*Open File */
    let mut file = File::open("words.txt").expect("Could not open the file");

    /*Loading file contents */
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("An error occuerd while reading the file !");

    /*Get individual Words into vector */
    let available_words: Vec<&str> = file_content.trim().split(',').collect();

    /*Generate random index */
    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter>
{
    /*create empty vector */
    let mut letters: Vec<Letter> = Vec::new();

    /*wrap each charachter in a letter struct */
    for c in word.chars()
    {
        letters.push(Letter{
            character: c,
            revealed: false
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>)
{
    let mut display_string = String::from("Progress:");

    /*display appropriate charachter (letter or _ )for each letter */
    for letter in letters
    {
        display_string.push(' ');

        if letter.revealed
        {
            display_string.push(letter.character);
        }
        else
        {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn read_user_input_character() -> char
{
    let mut user_input = String::new();

    /*Get user input */
    match io::stdin().read_line(&mut user_input)
    {
        Ok(_) =>
        {
            match user_input.chars().next()
            {
                Some(c) =>{return c;}
                None => {return '*';}
            }
        }
        Err(_) => {return '*';}
    }
}

fn check_progress(turns_left: u8 , letters: &Vec<Letter>) -> GameProgress
{
    /*Determin if all letters have been reaveld */
    let mut all_reveald = true;
    for letter in letters
    {
        if ! letter.revealed
        {
            all_reveald = false;
        }
    }

    if all_reveald
    {
        return GameProgress::Won;
    }

    if turns_left > 0
    {
        return GameProgress::InProgress;
    }
    return GameProgress::Lost;
}
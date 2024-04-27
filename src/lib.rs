use random_word::Lang;
const BACKSPACE_CHAR: char = '\u{8}';
const SPACE_CHAR: char = ' ';

#[derive(PartialEq, Debug)]
pub enum CharColor {
    Gray,
    White,
    Red,
    TransparentRed,
}

#[derive(PartialEq, Debug)]
pub struct Character {
    pub value: char,
    pub color: CharColor,
    pub underlined: bool,
}


enum InputAction {
    Insert(char),
    Space,
    Backspace,
}

pub struct Game {
    pub characters: Vec<Character>,
    pub position: usize,
    pub started: bool
}


impl Game {
    pub fn new(text: String) -> Game {
        let mut game = Game { characters: vec![], position: 0, started:false };
        for character in text.chars() {
            game.characters.push(Character { value: character, color: CharColor::Gray, underlined: false });
        }
        game
    }

    pub fn generate(size:usize) -> Game{
        let mut game = Game { characters: vec![], position: 0, started:false };
        let mut words: Vec<&str> = vec![];
        for word in 0..size{
           words.push(random_word::gen(Lang::En));
        }
        let final_string = words.join(" ");
        for character in final_string.chars() {
            game.characters.push(Character { value: character, color: CharColor::Gray, underlined: false });
        }
         game
    }

    fn expected_character(&self) -> &Character {
        &self.characters[self.position]
    }

    fn remove_underline(&mut self, position: usize) {
        self.characters[position].underlined = false;
    }

    fn handle_action(&mut self, action: InputAction) {
        match action {
            InputAction::Backspace => {

                self.position -= 1;
                if self.expected_character().value == SPACE_CHAR{
                    for character in self.characters[..self.position].iter_mut().rev() {
                        if character.value == SPACE_CHAR {
                            break
                        }
                        character.underlined = false;
                    }

                }
                if self.expected_character().color == CharColor::TransparentRed {
                    self.characters.remove(self.position);
                } else if self.expected_character().color == CharColor::Gray {
                    while self.expected_character().color == CharColor::Gray {
                        self.position -= 1;
                    }
                    self.position += 1;
                }

                self.change_character_color(CharColor::Gray);
            }
            InputAction::Space => {
                if self.is_started_word() {
                    self.position += self.next_word_distance();
                }
                if self.is_error_in_previous_word() {

                    for character in self.characters[..self.position-1].iter_mut().rev() {
                        if character.value != SPACE_CHAR {
                            character.underlined = true;
                        }
                        else{
                            break
                        }
                    }
                }
            }
            InputAction::Insert(character) => {
                if !self.started {
                    self.started = true
                }
                if self.expected_character().value != character {
                    if self.expected_character().value == SPACE_CHAR {
                        self.characters.insert(self.position, Character { value: character, color: CharColor::TransparentRed, underlined: false })
                    } else {
                        self.change_character_color(CharColor::Red);
                    }
                } else {
                    self.change_character_color(CharColor::White);
                }
                self.position += 1;
            }
        }
    }

    pub fn input(&mut self, c: char) {
        match c {
            BACKSPACE_CHAR => {
                if self.position == 0 || (!self.is_started_word() && !self.is_error_in_previous_word()) {
                    return;
                }
                self.handle_action(InputAction::Backspace);
            }
            SPACE_CHAR => {
                self.handle_action(InputAction::Space);
            }
            _ => {
                self.handle_action(InputAction::Insert(c));
            }
        };
    }

    fn is_started_word(&mut self) -> bool {
        if self.position == 0 {
            return false;
        }
        self.characters[self.position - 1].value != SPACE_CHAR
    }

    fn next_word_distance(&self) -> usize {
        for (index, character) in self.characters[self.position..].iter().enumerate() {
            if character.value == SPACE_CHAR {
                return index + 1;
            }
        }
        0
    }

    fn change_character_color(&mut self, color: CharColor) {
        self.characters[self.position].color = color;
    }

    fn is_error_in_previous_word(&self) -> bool {
        let mut first_space_found = false;
        for character in self.characters[..self.position].iter().rev() {
            if !first_space_found && character.value == SPACE_CHAR {
                first_space_found = true;
            } else if first_space_found && (character.value != SPACE_CHAR && character.color != CharColor::White) {
                return true;
            } else if first_space_found && character.value == SPACE_CHAR {
                return false;
            }
        }
        false
    }

}

#[cfg(test)]
mod tests {
    use crate::CharColor::{Gray, Red, White};
    use super::*;

    #[test]
    fn create_new_game() {
        let game = Game::new(String::from("hello world"));
        assert_eq!(game.characters,
                   vec![Character { value: 'h', color: CharColor::Gray, underlined: false },
                        Character { value: 'e', color: CharColor::Gray, underlined: false },
                        Character { value: 'l', color: CharColor::Gray, underlined: false },
                        Character { value: 'l', color: CharColor::Gray, underlined: false },
                        Character { value: 'o', color: CharColor::Gray, underlined: false },
                        Character { value: ' ', color: CharColor::Gray, underlined: false },
                        Character { value: 'w', color: CharColor::Gray, underlined: false },
                        Character { value: 'o', color: CharColor::Gray, underlined: false },
                        Character { value: 'r', color: CharColor::Gray, underlined: false },
                        Character { value: 'l', color: CharColor::Gray, underlined: false },
                        Character { value: 'd', color: CharColor::Gray, underlined: false },
                   ]);
    }

    #[test]
    fn type_correct_char_change_letter_color_to_white() {
        let mut game = Game::new(String::from("hello world"));
        game.input('h');
        game.input('e');
        assert_eq!(game.characters[0].color, CharColor::White);
        assert_eq!(game.characters[1].color, CharColor::White);
    }

    #[test]
    fn type_correct_char_increment_position() {
        let mut game = Game::new(String::from("hello world"));
        assert_eq!(game.position, 0);
        game.input('h');
        assert_eq!(game.position, 1);
    }

    #[test]
    fn type_incorrect_char_change_letter_color_to_red() {
        let mut game = Game::new(String::from("hello world"));
        game.input('x');
        assert_eq!(game.characters[0].color, CharColor::Red)
    }

    #[test]
    fn backspace_decrement_position_and_change_char_color_to_gray() {
        let mut game = Game::new(String::from("hello world"));
        game.input('h');
        game.input('e');
        game.input('\u{8}');
        assert_eq!(game.position, 1);
        assert_eq!(game.characters[1].color, CharColor::Gray);
    }

    #[test]
    fn backspace_on_first_position_do_nothing() {
        let mut game = Game::new(String::from("hello world"));
        game.input('\u{8}');
        assert_eq!(game.position, 0);
        assert_eq!(game.characters[0].color, CharColor::Gray);
    }


    #[test]
    fn check_if_current_word_is_started() {
        let mut game = Game::new(String::from("hello world"));
        game.input('h');
        assert_eq!(game.is_started_word(), true);
    }

    #[test]
    fn check_if_current_word_is_not_started() {
        let mut game = Game::new(String::from("hello world"));
        assert_eq!(game.is_started_word(), false);
    }

    #[test]
    fn find_next_word_distance() {
        let mut game = Game::new(String::from("hello world"));
        game.input('h');
        assert_eq!(game.next_word_distance(), 5);
    }

    #[test]
    fn space_on_started_word_jump_to_next_word() {
        let mut game = Game::new(String::from("hello world"));
        game.input('h');
        game.input(' ');
        assert_eq!(game.position, 6);
    }

    #[test]
    fn backspace_on_jump_word_return_to_previous_position_before_jump() {
        let mut game = Game::new(String::from("hello world"));
        game.input('h');
        game.input('e');
        game.input(' ');
        game.input('\u{8}');
        assert_eq!(game.position, 2);
    }

    #[test]
    fn space_on_not_started_word_do_nothing() {
        let mut game = Game::new(String::from("ab world bernie"));
        game.input('a');
        game.input('b');
        game.input(' ');
        game.input(' ');
        assert_eq!(game.position, 3);
    }

    #[test]
    fn space_when_expected_space_insert_normally() {
        let mut game = Game::new(String::from("hello world bernie"));
        game.input('h');
        game.input('e');
        game.input('l');
        game.input('l');
        game.input('o');
        game.input(' ');
        assert_eq!(game.position, 6);
    }

    #[test]
    fn wrong_char_when_space_insert_red_char() {
        let mut game = Game::new(String::from("ab ho"));
        game.input('a');
        game.input('b');
        game.input('c');
        game.input('d');
        assert_eq!(game.characters, vec![
            Character { value: 'a', color: CharColor::White, underlined: false },
            Character { value: 'b', color: CharColor::White, underlined: false },
            Character { value: 'c', color: CharColor::TransparentRed, underlined: false },
            Character { value: 'd', color: CharColor::TransparentRed, underlined: false },
            Character { value: ' ', color: CharColor::Gray, underlined: false },
            Character { value: 'h', color: CharColor::Gray, underlined: false },
            Character { value: 'o', color: CharColor::Gray, underlined: false },
        ]);
    }

    #[test]
    fn remove_wrong_char_instead_space_remove_char_in_game_characters() {
        let mut game = Game::new(String::from("ab ho"));
        game.input('a');
        game.input('b');
        game.input('c');
        game.input('d');
        game.input('\u{8}');
        assert_eq!(game.characters, vec![
            Character { value: 'a', color: CharColor::White, underlined: false },
            Character { value: 'b', color: CharColor::White, underlined: false },
            Character { value: 'c', color: CharColor::TransparentRed, underlined: false },
            Character { value: ' ', color: CharColor::Gray, underlined: false },
            Character { value: 'h', color: CharColor::Gray, underlined: false },
            Character { value: 'o', color: CharColor::Gray, underlined: false },
        ]);
        assert_eq!(game.position, 3);
    }

    #[test]
    fn cannot_go_back_if_no_error_in_previous_word_and_not_started_word() {
        let mut game = Game::new(String::from("ab ho"));
        game.input('a');
        game.input('b');
        game.input(' ');
        game.input('\u{8}');
        assert_eq!(game.position, 3);
    }

    #[test]
    fn check_if_error_in_previous_word() {
        let mut game = Game::new(String::from("ab ho"));
        game.input('a');
        game.input('x');
        game.input(' ');
        assert_eq!(game.is_error_in_previous_word(), true);
    }

    #[test]
    fn check_if_no_error_in_previous_word() {
        let mut game = Game::new(String::from("ab ho"));
        game.input('a');
        game.input('b');
        game.input(' ');
        assert_eq!(game.is_error_in_previous_word(), false);
    }

    #[test]
    fn check_previous_word_with_error_is_underlined() {
        let mut game = Game::new(String::from("ab ho"));
        game.input('x');
        game.input('b');
        game.input(' ');
        assert_eq!(game.characters,
                   vec![
                       Character { value: 'a', color: Red, underlined: true },
                       Character { value: 'b', color: White, underlined: true },
                       Character { value: ' ', color: Gray, underlined: false },
                       Character { value: 'h', color: Gray, underlined: false },
                       Character { value: 'o', color: Gray, underlined: false },
                   ]
        );
    }


    #[test]
    fn check_backspace_to_previous_word_with_error_remove_underlining() {
        let mut game = Game::new(String::from("ab ho"));
        game.input('x');
        game.input('b');
        game.input(' ');
        game.input('\u{8}');
        assert_eq!(game.characters,
                   vec![
                       Character { value: 'a', color: Red, underlined: false },
                       Character { value: 'b', color: White, underlined: false },
                       Character { value: ' ', color: Gray, underlined: false },
                       Character { value: 'h', color: Gray, underlined: false },
                       Character { value: 'o', color: Gray, underlined: false },
                   ]
        );
        assert_eq!(game.position, 2);
    }

    #[test]
    fn check_error_in_previous_word_when_jump() {
        let mut game = Game::new(String::from("abc ho"));
        game.input('a');
        game.input(' ');
        assert_eq!(game.characters,
                   vec![
                       Character { value: 'a', color: White, underlined: true },
                       Character { value: 'b', color: Gray, underlined: true },
                       Character { value: 'c', color: Gray, underlined: true },
                       Character { value: ' ', color: Gray, underlined: false },
                       Character { value: 'h', color: Gray, underlined: false },
                       Character { value: 'o', color: Gray, underlined: false },
                   ]
        );
    }

    #[test]
    fn check_underline_correctly_after_2_words() {
        let mut game = Game::new(String::from("ab ab ab"));
        game.input('a');
        game.input('b');
        game.input(' ');
        game.input('a');
        game.input('b');
        game.input(' ');
        assert_eq!(game.characters,
                   vec![
                       Character { value: 'a', color: White, underlined: false },
                       Character { value: 'b', color: White, underlined: false },
                       Character { value: ' ', color: Gray, underlined: false },
                       Character { value: 'a', color: White, underlined: false },
                       Character { value: 'b', color: White, underlined: false },
                       Character { value: ' ', color: Gray, underlined: false },
                       Character { value: 'a', color: Gray, underlined: false },
                       Character { value: 'b', color: Gray, underlined: false },
                   ]
        );
    }

    #[test]
    fn check_one_wrong_word_underline_online_one_word() {
        let mut game = Game::new(String::from("ab ab ab"));
        game.input('a');
        game.input('b');
        game.input(' ');
        game.input('x');
        game.input('b');
        game.input(' ');
        assert_eq!(game.characters,
                   vec![
                       Character { value: 'a', color: White, underlined: false },
                       Character { value: 'b', color: White, underlined: false },
                       Character { value: ' ', color: Gray, underlined: false },
                       Character { value: 'a', color: Red, underlined: true },
                       Character { value: 'b', color: White, underlined: true },
                       Character { value: ' ', color: Gray, underlined: false },
                       Character { value: 'a', color: Gray, underlined: false },
                       Character { value: 'b', color: Gray, underlined: false },
                   ]
        );
    }

    #[test]
    fn check_start_game_on_first_input(){
       let mut game = Game::generate(30);
        game.input('e');
        assert_eq!(game.started, true)
    }



}

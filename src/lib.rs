#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    all_rolls: [u16;23],
    roll_counter: usize,
    frame_counter: usize,
}

const LAST_FRAME: usize = 10;
const STRIKE_PINS: u16 = 10;
const STRIKE_POINTS: u16 = 10;

impl BowlingGame {
    pub fn new() -> Self {
        Self { all_rolls: [0;23], roll_counter: 0, frame_counter: 0 }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        match self.is_valid_number_of_pins(pins) {
            Err(error) => return Err(error),
            _ => ()
        };

        self.all_rolls[self.roll_counter] = pins;

        if BowlingGame::is_strike(pins) {
            self.roll_counter += 2;
        } else {
            self.roll_counter += 1;
        }

        if self.roll_counter % 2 == 0 && self.frame_counter < LAST_FRAME {
            self.frame_counter += 1;
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            return None;
        }
        let mut score = 0;

        let mut previous_was_strike = false;

        for frame in 1..=LAST_FRAME {
            let first_roll_index = (frame - 1) * 2;
            let first_roll = self.all_rolls[first_roll_index];

            if BowlingGame::is_strike(first_roll) {
                score += self.handle_strike_score(first_roll_index, frame);
                previous_was_strike = true;
                continue;
            }
            if previous_was_strike {
                previous_was_strike = false;
                continue;
            }

            let second_roll_index = first_roll_index + 1;
            let second_roll = self.all_rolls[second_roll_index];
            if BowlingGame::is_spare(first_roll, second_roll) {
                score += 10 + self.all_rolls[second_roll_index + 1];
                continue;
            }

            score += first_roll + second_roll;
        }

        Some(score)
    }

    fn is_valid_number_of_pins(&self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let is_second_roll_in_frame = self.roll_counter % 2 != 0;

        if is_second_roll_in_frame {
            let previous_roll = self.all_rolls[self.roll_counter - 1];
            let frame_pins = previous_roll + pins;
            if frame_pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        Ok(())
    }

    fn handle_strike_score(&self, strike_roll_index: usize, frame: usize) -> u16 {
        let next_roll = self.all_rolls[strike_roll_index + 2];

        let is_next_roll_strike = BowlingGame::is_strike(next_roll);

        let next_roll_index = match is_next_roll_strike {
            true => strike_roll_index + 4,
            false =>  strike_roll_index + 3
        };

        let next_next_move = self.all_rolls[next_roll_index];

        if is_next_roll_strike || BowlingGame::is_last_frame(frame) {
            STRIKE_POINTS + next_roll + next_next_move
        } else {
            STRIKE_POINTS + ( 2 * next_roll ) + ( 2 * next_next_move )
        }
    }

    fn is_last_frame(frame: usize) -> bool {
        frame == LAST_FRAME
    }

    fn is_game_complete(&self) -> bool {
        let last_frame_is_spare = BowlingGame::is_spare(self.all_rolls[18], self.all_rolls[19]);
        let last_frame_is_strike = BowlingGame::is_strike(self.all_rolls[18]);

        if last_frame_is_spare { // if last frame is spare, player gets one extra roll
            return self.roll_counter >= 21;
        } else if last_frame_is_strike {
            if  BowlingGame::is_strike(self.all_rolls[20]) {
                return self.roll_counter == 24;
            } else {
                return self.roll_counter == 22;
            }
        } else {
            return self.roll_counter == 20;
        }
    }

    fn is_strike(pins: u16) -> bool {
        pins == STRIKE_PINS
    }

    fn is_spare(first_roll: u16, second_roll: u16) -> bool {
        (first_roll + second_roll) == 10 && !BowlingGame::is_strike(first_roll)
    }
}

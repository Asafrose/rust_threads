use std::{sync::{Arc, Condvar, Mutex, mpsc::Receiver}, time::Duration, usize};

use crate::{
    common::{Guess, IncorrectStringLenError},
    infra::{encryption_helper, generic_error::GenericError, randomizer},
};

pub struct Encrypter {
    string_len: usize,
    clear_string: String,
    channel_reciever: Receiver<Guess>,
    arc: Arc<(Mutex<Option<Vec<u8>>>, Condvar)>,
    timeout: Duration
}

impl Encrypter {
    pub fn new(
        string_len: usize,
        channel_reciever: Receiver<Guess>,
        arc: Arc<(Mutex<Option<Vec<u8>>>, Condvar)>,
        timeout: Duration
    ) -> Result<Self, IncorrectStringLenError> {
        if string_len % 8 != 0 {
            Err(IncorrectStringLenError::new(string_len))
        } else {
            Ok(Self {
                string_len,
                clear_string: String::default(),
                channel_reciever,
                arc,
                timeout
            })
        }
    }

    pub fn run(&mut self) -> Result<(), GenericError> {
        println!("|Encrypter| started");
        self.recycle_string()?;

        loop {
            match self.channel_reciever.recv_timeout(self.timeout) {
                Ok(guess) => {
                    if guess.payload == self.clear_string {
                        println!(
                            "|Encrypter| Recieved correct guess from decrypter {} which won the race",
                            guess.id
                        );
                        self.recycle_string()?;
                    } else {
                        println!(
                            "|Encrypter| Recieved wrong guess from decrypter {} [payload={}]",
                            guess.id, guess.payload
                        );
                    }
                }
                Err(timeout) => {
                    println!("|Encrypter| timed out on recieve [error={}]", timeout);
                    self.recycle_string()?;
                }
            }
        }
    }

    fn recycle_string(&mut self) -> Result<(), GenericError> {
        let (mutex, condvar) = &*self.arc;
        let key = randomizer::get_random_string(self.string_len / 8);
        let mut guard = mutex.lock()?;
        self.clear_string = randomizer::get_random_string(self.string_len);
        *guard = Some(encryption_helper::encrypt(
            self.clear_string.as_bytes(),
            &key,
        ));
        condvar.notify_all();
        println!(
            "|Encrypter| Generated new password [clear_string={}]",
            self.clear_string
        );
        Ok(())
    }
}

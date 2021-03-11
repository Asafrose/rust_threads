use std::{str::{self}, sync::{Arc, Condvar, Mutex, mpsc::Sender}};

use crate::{
    common::{Guess, IncorrectStringLenError},
    infra::{
        encryption_helper,
        generic_error::{GenericError, ToGenericError},
        iterator_extension::IteratorExtension,
        randomizer,
    },
};

pub struct Decrypter {
    id: u32,
    string_length: usize,
    channel_sender: Sender<Guess>,
    arc: Arc<(Mutex<Option<Vec<u8>>>, Condvar)>,
}

impl Decrypter {
    pub fn new(
        id: u32,
        string_length: usize,
        channel_sender: Sender<Guess>,
        arc: Arc<(Mutex<Option<Vec<u8>>>, Condvar)>,
    ) -> Result<Self, IncorrectStringLenError> {
        if string_length % 8 != 0 {
            Err(IncorrectStringLenError::new(string_length))
        } else {
            Ok(Self {
                id,
                string_length,
                channel_sender,
                arc,
            })
        }
    }

    pub fn run(&self) -> Result<(), GenericError> {
        println!("|Decrypter-{}| started", self.id);

        let (mutex, condvar) = &*self.arc;

        {
            let mut guard = mutex.lock()?;
            while *guard == None {
                guard = condvar.wait(guard)?;
            }
        }
        let mut counter = 0;

        let mut encrypted_data = self.get_encrypted_data()?;
        loop {
            counter += 1;

            let new_encrypted_data = self.get_encrypted_data()?;
            if !new_encrypted_data.sequence_equals(&encrypted_data) {
                encrypted_data = new_encrypted_data;
                counter = 0;
            }

            let key = self.generate_key();
            match encryption_helper::decrypt(&encrypted_data, &key){
                Ok(decryped) => {
                    match str::from_utf8(&decryped) {
                        Ok(str) => {
                            println!(
                                "|Decrypter-{}| found clear string [str={} key={} retries={}]",
                                self.id, str, key, counter
                            );
                            self.channel_sender
                                .send(Guess {
                                    id: self.id,
                                    payload: str.to_string(),
                                })?;
                        }
                        Err(_) => {}
                    };
                }
                Err(_) => {}
            }
        }
    }

    fn get_encrypted_data(&self) -> Result<Vec<u8>, GenericError> {
        let mutex = &((*self.arc).0);

        match &*(mutex.lock()?) {
            Some(vec) => Ok(vec.clone()),
            None => Err("encrypter not initialized".to_generic_error()),
        }
    }

    fn generate_key(&self) -> String {
        randomizer::get_random_string(self.string_length / 8)
    }
}

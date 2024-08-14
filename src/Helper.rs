use rand::seq::SliceRandom;
use rand::Rng;
use std::fs;
use std::time::{Instant};

pub struct Helper {
    clock_time: Option<Instant>,
}

impl Helper {
    pub fn new() -> Self {
        Self {
            clock_time: None,
        }
    }

    // Suponha que esta seja a sua lista de palavras
    pub fn calculate_combination(&self, words: &mut Vec<String>, current: Option<Vec<String>>) -> Vec<String> {
        let n = words.len();

        if let Some(mut current_combination) = current {
            // Encontrar o índice da permutação atual
            let mut i = n as isize - 2;
            while i >= 0 && words.iter().position(|x| x == &current_combination[i as usize])
                >= words.iter().position(|x| x == &current_combination[i as usize + 1])
            {
                i -= 1;
            }

            if i == -1 {
                // Se a permutação atual é a última, retornar a primeira permutação
                words.sort();
                return words.clone();
            }

            // Encontrar o menor elemento à direita de permutacaoAtual[i] que seja maior que permutacaoAtual[i]
            let mut j = n as isize - 1;
            while words.iter().position(|x| x == &current_combination[j as usize])
                <= words.iter().position(|x| x == &current_combination[i as usize])
            {
                j -= 1;
            }

            // Trocar permutacaoAtual[i] com o menor elemento à direita que seja maior
            current_combination.swap(i as usize, j as usize);

            // Inverter a parte à direita de permutacaoAtual[i]
            let mut right_part = current_combination.split_off(i as usize + 1);
            right_part.reverse();

            // Concatenar a parte à esquerda, o elemento trocado e a parte invertida à direita
            current_combination.extend(right_part);
            current_combination
        } else {
            // Se current for None, retorna a primeira permutação possível
            words.sort();
            words.iter().take(12).cloned().collect()
        }
    }

    pub fn seed_shuffle<T: Clone>(&self, arr: &mut Vec<T>, seed: f64) -> Vec<T> {
        let mut rng = self.random_with_seed(seed);
        let mut shuffled_array = arr.clone();
        shuffled_array.shuffle(&mut rng);
        shuffled_array
    }

    pub fn random_with_seed(&self, seed: f64) -> impl Rng {
        let mut s = seed.to_bits() as u64;
        rand::rngs::StdRng::seed_from_u64(s)
    }

    pub fn read_file(&self, file: &str, def: &str) -> String {
        fs::read_to_string(file).unwrap_or_else(|_| def.to_string())
    }

    pub fn read_int_file(&self, file: &str, def: i32) -> i32 {
        self.read_file(file, &def.to_string()).parse().unwrap_or(def)
    }

    pub fn start_clock(&mut self) {
        if self.clock_time.is_some() {
            panic!("o relógio já está rodando");
        }
        self.clock_time = Some(Instant::now());
    }

    pub fn end_clock(&mut self, log: Option<&str>) -> u128 {
        if let Some(start_time) = self.clock_time {
            let elapsed_time = start_time.elapsed().as_millis();
            if let Some(log_message) = log {
                println!("{} ms ::: {}", elapsed_time, log_message);
            }
            self.clock_time = None;
            elapsed_time
        } else {
            panic!("O relógio não está rodando");
        }
    }

    pub async fn delay(&self, time: u64) {
        tokio::time::sleep(std::time::Duration::from_millis(time)).await;
    }
}
use rand::prelude::*;
use std::time::Instant;

fn bubble_sort_classico(vetor: &mut [u32]) {
    let tam = vetor.len();
    for i in 0..tam {
        for j in 0..(tam - i - 1) {
            if vetor[j] > vetor[j + 1] {
                let temp = vetor[j];
                vetor[j] = vetor[j + 1];
                vetor[j + 1] = temp;
            }
        }
    }
}

fn bubble_sort_adaptativo(vetor: &mut [u32]) {
    let tam = vetor.len();
    for i in 0..tam {
        let mut houve_troca = false;
        for j in 0..(tam - i - 1) {
            if vetor[j] > vetor[j + 1] {
                let temp = vetor[j];
                vetor[j] = vetor[j + 1];
                vetor[j + 1] = temp;
                houve_troca = true;
            }
        }
        if !houve_troca {
            break;
        }
    }
}

fn popula_vec(refvec: &mut [u32]){
    let mut rng = rand::rng();
    for i in 0..refvec.len(){
        refvec[i] = rng.random::<u32>();    
    }
    
}

fn main(){
    let mut vec1: Vec<u32> = vec![0; 50_000];
    let mut vec2: Vec<u32> = vec![0; 50_000];

    popula_vec(&mut vec1);
    popula_vec(&mut vec2);


    let inicio1 = Instant::now();
    bubble_sort_classico(&mut vec1);
    println!("Classico demorou = {:?}", inicio1.elapsed());

    let inicio2 = Instant::now();
    bubble_sort_adaptativo(&mut vec2);
    println!("Adaptativo demorou = {:?}", inicio2.elapsed());

}
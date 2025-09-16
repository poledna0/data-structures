// Henrique Poledna

#[derive(Debug, Clone)]
pub struct No {
    pub valor: i32,
    pub proximo: Option<usize>,
}

impl No {
    pub fn new(valor: i32) -> Self {
        No {
            valor,
            proximo: None,
        }
    }
}

#[derive(Debug)]
pub struct Lista {
    pub nos: Vec<No>,
    pub inicio: Option<usize>,
    pub fim: Option<usize>,
}

impl Lista {
    pub fn new() -> Self {
        Lista {
            nos: Vec::new(),
            inicio: None,
            fim: None,
        }
    }

    pub fn esta_vazia(&self) -> bool {
        self.inicio.is_none()
    }
    #[allow(dead_code)]
    pub fn imprimir(&self) {
        
        print!("Lista: [");
        let mut indice_atual_opt = self.inicio;
        
        while let Some(indice_atual) = indice_atual_opt {
            let no = &self.nos[indice_atual];
            print!("{}", no.valor);

            indice_atual_opt = no.proximo;

            if indice_atual_opt.is_some() {
                print!(" -> ");
            }
        }
        println!("]");
    }

    pub fn inserir_no_inicio(&mut self, valor: i32) {
        let mut novo_no = No::new(valor);
        
        novo_no.proximo = self.inicio;

        self.nos.push(novo_no);
        let novo_indice = self.nos.len() - 1;

        self.inicio = Some(novo_indice);

        if self.fim.is_none() {
            self.fim = Some(novo_indice);
        }
    }

    pub fn inserir_no_final(&mut self, valor: i32) {
        if self.esta_vazia() {
            self.inserir_no_inicio(valor);
            return;
        }

        let novo_no = No::new(valor);

        // adiciona o novo no ao vetor e pega seu indice
        self.nos.push(novo_no);
        let novo_indice = self.nos.len() - 1;
        
        // pega o indice do no que atualmete e o último
        let indice_fim_antigo = self.fim.unwrap();

        // faz o antigo ultimo no apontar para o novo no
        self.nos[indice_fim_antigo].proximo = Some(novo_indice);

        // atualiza o fim da lista para ser o novo nó
        self.fim = Some(novo_indice);
    }
    #[allow(dead_code)]
    pub fn inserir_em_posicao(&mut self, valor: i32, n: usize) {
        if n == 0 {
            self.inserir_no_inicio(valor);
            return;
        }

        let mut indice_anterior_opt = self.inicio;
        for _ in 0..(n) {
            match indice_anterior_opt {
                Some(indice) => indice_anterior_opt = self.nos[indice].proximo,
                None => { 
                    println!("Erro fora dos limites da lista inserindo no final.");
                    self.inserir_no_final(valor);
                    return;
                }
            }
        }

        if let Some(indice_anterior) = indice_anterior_opt {
             // se o no anterior era o fim, isso é o mesmo que inserir no final
            if self.nos[indice_anterior].proximo.is_none() {
                 self.inserir_no_final(valor);
                 return;
            }

            let mut novo_no = No::new(valor);
            // o proximo do novo nó sera o proximo do no anterior
            novo_no.proximo = self.nos[indice_anterior].proximo;

            self.nos.push(novo_no);
            let novo_indice = self.nos.len() - 1;

            // O próximo do nó anterior agora aponta para o novo nó
            self.nos[indice_anterior].proximo = Some(novo_indice);
        } else {
             println!("Erro: Posição {} fora dos limites da lista. Inserindo no final.", n);
             self.inserir_no_final(valor);
        }
    }


    pub fn remover_do_inicio(&mut self) -> Option<i32> {
        self.inicio.map(|indice_inicio| {
            let no_removido = self.nos[indice_inicio].clone();
            
            self.inicio = no_removido.proximo;

            if self.inicio.is_none() {
                self.fim = None;
            }
        
            no_removido.valor
        })
    }
    #[allow(dead_code)]
    pub fn remover_do_final(&mut self) -> Option<i32> {
        if self.esta_vazia() {
            return None;
        }
        
        if self.inicio == self.fim {
            return self.remover_do_inicio();
        }

        let mut indice_atual = self.inicio.unwrap();
        let indice_fim = self.fim.unwrap();

        while self.nos[indice_atual].proximo != Some(indice_fim) {
            indice_atual = self.nos[indice_atual].proximo.unwrap();
        }
        
        let no_removido = self.nos[indice_fim].clone();
        
        self.nos[indice_atual].proximo = None;
        self.fim = Some(indice_atual);
        
        Some(no_removido.valor)
    }

    pub fn remover_de_posicao(&mut self, n: usize) -> Option<i32> {
        if self.esta_vazia() {
            return None;
        }

        if n == 0 {
            return self.remover_do_inicio();
        }

        let mut indice_anterior_opt = self.inicio;
        for _ in 0..(n-1) {
            match indice_anterior_opt {
                Some(indice) => indice_anterior_opt = self.nos[indice].proximo,
                None => return None,
            }
        }

        let indice_a_remover = match indice_anterior_opt {
            Some(indice) => self.nos[indice].proximo,
            None => return None,
        };
        
        if let Some(indice_removido) = indice_a_remover {
            let no_removido = self.nos[indice_removido].clone();
            let indice_anterior = indice_anterior_opt.unwrap();

            self.nos[indice_anterior].proximo = no_removido.proximo;
            
            if self.fim == Some(indice_removido) {
                self.fim = Some(indice_anterior);
            }

            Some(no_removido.valor)
        } else {
            None
        }
    }
}

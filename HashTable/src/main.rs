mod table;
mod hash;

const NUM_BUCKETS: usize = 10;

#[derive(Debug, Clone)]
struct Par {
	chave: String,
	valor: String,
}

struct HashTable {
	buckets: Vec<table::Lista>,
	pares: Vec<Par>,
}

impl HashTable {
	fn new() -> Self {
		let mut buckets = Vec::with_capacity(NUM_BUCKETS);
		for _ in 0..NUM_BUCKETS {
			buckets.push(table::Lista::new());
		}
		HashTable { buckets, pares: Vec::new() }
	}

	fn hash(chave: &String) -> usize {
		crate::hash::devolve_hash(chave) as usize % NUM_BUCKETS
	}

	fn inserir(&mut self, chave: String, valor: String) {
		let idx = Self::hash(&chave);
		let par = Par { chave: chave.clone(), valor };
		self.pares.push(par);
		let par_idx = self.pares.len() - 1;
		self.buckets[idx].inserir_no_final(par_idx as i32);
	}

	fn buscar(&self, chave: &str) -> Option<&String> {
		let idx = Self::hash(&chave.to_string());
		let mut atual = self.buckets[idx].inicio;
		while let Some(i) = atual {
			let par_idx = self.buckets[idx].nos[i].valor as usize;
			if self.pares[par_idx].chave == chave {
				return Some(&self.pares[par_idx].valor);
			}
			atual = self.buckets[idx].nos[i].proximo;
		}
		None
	}

	fn remover(&mut self, chave: &str) -> bool {
		let idx = Self::hash(&chave.to_string());
		let mut atual = self.buckets[idx].inicio;
		let mut pos = 0;
		while let Some(i) = atual {
			let par_idx = self.buckets[idx].nos[i].valor as usize;
			if self.pares[par_idx].chave == chave {
				self.buckets[idx].remover_de_posicao(pos);
				return true;
			}
			atual = self.buckets[idx].nos[i].proximo;
			pos += 1;
		}
		false
	}

	fn exibir(&self) {
		for (i, bucket) in self.buckets.iter().enumerate() {
			print!("Bucket {}: ", i);
			let mut atual = bucket.inicio;
			while let Some(idx) = atual {
				let par_idx = bucket.nos[idx].valor as usize;
				let par = &self.pares[par_idx];
				print!("[{}: {}] -> ", par.chave, par.valor);
				atual = bucket.nos[idx].proximo;
			}
			println!("None");
		}
	}
}

fn main() {
    // chatgpt: inserção de 10 elementos com colisões
	let mut tabela = HashTable::new();

	// Inserção de 10 elementos (com colisões)
	let dados = vec![
		("abcdefghij", "Valor para abcdefghij"),
		("klmnopqrst", "Valor para klmnopqrst"),
		("uvwxyzabcd", "Valor para uvwxyzabcd"),
		("mnopqrstuv", "Valor para mnopqrstuv"),
		("abcdefghik", "Valor para abcdefghik"),
		("klmnopqrsu", "Valor para klmnopqrsu"),
		("uvwxyzabce", "Valor para uvwxyzabce"),
		("mnopqrstuw", "Valor para mnopqrstuw"),
		("abcdefghil", "Valor para abcdefghil"),
		("klmnopqrsw", "Valor para klmnopqrsw"),
	];

	for (chave, valor) in dados.iter() {
		tabela.inserir(chave.to_string(), valor.to_string());
	}

	println!("\nTabela Hash após inserções:");
	tabela.exibir();

	let busca = "klmnopqrst";
	println!("\nBuscando chave '{}':", busca);
	match tabela.buscar(busca) {
		Some(valor) => println!("Encontrado: {}", valor),
		None => println!("Não encontrado!"),
	}

	// Remoção
	let remover = "klmnopqrst";
	println!("\nRemovendo chave '{}':", remover);
	if tabela.remover(remover) {
		println!("Removido com sucesso!");
	} else {
		println!("Chave não encontrada!");
	}

	println!("\nTabela Hash após remoção:");
	tabela.exibir();
}
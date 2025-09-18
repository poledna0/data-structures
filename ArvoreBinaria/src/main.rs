use std::io::{self, Write};
use std::cmp::Ordering;

struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32) -> Self
        Node {
            key,
            left: None,
            right: None,
        }
}

pub struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }
    pub fn insert(&mut self, key: i32) {

        Self::insert_recursive(&mut self.root, key);
    }

    fn insert_recursive(node: &mut Option<Box<Node>>, key: i32) {
        match node {
            None => {
                *node = Some(Box::new(Node::new(key)));
            }
            Some(current_node) => {
                match key.cmp(&current_node.key) {
                    Ordering::Less => Self::insert_recursive(&mut current_node.left, key),
                    Ordering::Greater => Self::insert_recursive(&mut current_node.right, key),
                    Ordering::Equal => {}
                }
            }
        }
    }


    pub fn search(&self, key: i32) -> (Vec<i32>, bool) {
        let mut path = Vec::new();
        let mut current = &self.root;
        while let Some(node) = current {
            path.push(node.key);
            match key.cmp(&node.key) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return (path, true), 
            }
        }
        
        (path, false)
    }


    pub fn print_in_order(&self) {
        let mut result = Vec::new();
        Self::in_order_recursive(&self.root, &mut result);
        println!("Em-ordem (In-order):  {:?}", result);
    }

    pub fn print_pre_order(&self) {
        let mut result = Vec::new();
        Self::pre_order_recursive(&self.root, &mut result);
        println!("Pré-ordem (Pre-order): {:?}", result);
    }

    pub fn print_post_order(&self) {
        let mut result = Vec::new();
        Self::post_order_recursive(&self.root, &mut result);
        println!("Pós-ordem (Post-order): {:?}", result);
    }
    
    fn in_order_recursive(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            Self::in_order_recursive(&n.left, result);
            result.push(n.key);
            Self::in_order_recursive(&n.right, result);
        }
    }

    fn pre_order_recursive(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            result.push(n.key);
            Self::pre_order_recursive(&n.left, result);
            Self::pre_order_recursive(&n.right, result);
        }
    }

    fn post_order_recursive(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            Self::post_order_recursive(&n.left, result);
            Self::post_order_recursive(&n.right, result);
            result.push(n.key);
        }
    }


    pub fn print_tree(&self) {
        if self.root.is_none() {
            println!("(Árvore vazia)");
            return;
        }
        Self::print_recursive(&self.root, String::from(""), true);
    }
                // chat gpt sabe mt
    fn print_recursive(node: &Option<Box<Node>>, prefix: String, is_tail: bool) {
        if let Some(n) = node {
            let new_prefix = prefix.clone() + if is_tail { "    " } else { "│   " };
            Self::print_recursive(&n.right, new_prefix.clone(), false);

            println!("{}{}{}", prefix, if is_tail { "└── " } else { "├── " }, n.key);
            
            Self::print_recursive(&n.left, new_prefix, true);
        }
    }
    
    
    pub fn remove(&mut self, key: i32) {
        Self::remove_recursive(&mut self.root, key);
    }

    fn remove_recursive(node: &mut Option<Box<Node>>, key: i32) {
        if let Some(current_node) = node {
            match key.cmp(&current_node.key) {
                Ordering::Less => Self::remove_recursive(&mut current_node.left, key),
                Ordering::Greater => Self::remove_recursive(&mut current_node.right, key),
                Ordering::Equal => {
                    if current_node.left.is_none() && current_node.right.is_none() {
                        *node = None;
                    } else if current_node.left.is_none() {
                        *node = current_node.right.take();
                    } else if current_node.right.is_none() {
                        *node = current_node.left.take();
                    } else {
                        if let Some(successor_key) = Self::find_min_key(&current_node.right) {
                            current_node.key = successor_key;
                            Self::remove_recursive(&mut current_node.right, successor_key);
                        }
                    }
                }
            }
        }
    }

    fn find_min_key(node: &Option<Box<Node>>) -> Option<i32> {
        let mut current = node;
        while let Some(n) = current {
            if n.left.is_none() {
                return Some(n.key);
            }
            current = &n.left;
        }
        None
    }
}



fn main() {
    let mut bst = BinarySearchTree::new();

    loop {
        println!("\n--- Árvore Binária de Busca (BST) em Rust ---");
        println!("1. Inserir valor");
        println!("2. Buscar valor");
        println!("3. Mostrar percursos (pré, in, pós-ordem)");
        println!("4. Mostrar árvore");
        println!("5. Remover valor");
        println!("6. Apagar árvore (reiniciar)");
        println!("7. Sair");
        print!("Escolha uma opção: ");
        
        io::stdout().flush().unwrap(); 

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Falha ao ler a linha.");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Opção inválida. Por favor, insira um número.");
                continue;
            }
        };

        match choice {
            1 => { // Inserir
                print!("Digite o valor a ser inserido: ");
                io::stdout().flush().unwrap();
                let mut value = String::new();
                io::stdin().read_line(&mut value).expect("Falha ao ler o valor.");
                match value.trim().parse::<i32>() {
                    Ok(val) => {
                        bst.insert(val);
                        println!("Valor {} inserido.", val);
                    },
                    Err(_) => println!("Valor inválido."),
                }
            }
            2 => { // Buscar
                print!("Digite o valor a ser buscado: ");
                io::stdout().flush().unwrap();
                let mut value = String::new();
                io::stdin().read_line(&mut value).expect("Falha ao ler o valor.");
                match value.trim().parse::<i32>() {
                    Ok(val) => {
                        let (path, found) = bst.search(val);
                        println!("Caminho percorrido: {:?}", path);
                        if found {
                            println!("Valor {} ENCONTRADO na árvore.", val);
                        } else {
                            println!("Valor {} NÃO ENCONTRADO na árvore.", val);
                        }
                    },
                    Err(_) => println!("Valor inválido."),
                }
            }
            3 => {
                if bst.root.is_none() {
                    println!("A árvore está vazia.");
                } else {
                    bst.print_pre_order();
                    bst.print_in_order();
                    bst.print_post_order();
                }
            }
            4 => { // Mostrar árvore
                println!("\n--- Visualização da Árvore ---");
                bst.print_tree();
                println!("----------------------------");
            }
            5 => {
                 print!("Digite o valor a ser removido: ");
                io::stdout().flush().unwrap();
                let mut value = String::new();
                io::stdin().read_line(&mut value).expect("Falha ao ler o valor.");
                match value.trim().parse::<i32>() {
                    Ok(val) => {
                        bst.remove(val);
                        println!("Tentativa de remoção do valor {} concluída.", val);
                    },
                    Err(_) => println!("Valor inválido."),
                }
            }
            6 => { // Apagar
                bst = BinarySearchTree::new();
                println!("Árvore apagada e reiniciada.");
            }
            7 => { // Sair
                println!("Encerrando o programa.");
                break;
            }
            _ => {
                println!("Opção desconhecida. Tente novamente.");
            }
        }
    }
}

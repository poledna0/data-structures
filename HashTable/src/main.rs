use std::option::Option;

struct No{
    valor: String,
    proximo: Option<u8>,
}
impl No{
    fn new(valor: String, proximo: Option<u8>)-> Self{
        No { valor: (valor), proximo: (proximo) }
    }
    
}

struct Lista {
    nos: Vec<No>,
    inicio: Option<usize>,
    fim: Option<usize>,
}


impl Lista {
    fn new() -> Self {
        Lista {
            nos: Vec::new(),
            inicio: None,
            fim: None,
        }
    }
}











fn main(){

}
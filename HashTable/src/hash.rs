pub fn devolve_hash(frase: &String) -> u32{

    let mut valor_final: u32 = 0;
    
    for c in frase.chars(){
        valor_final += c as u32;
    }

    valor_final = valor_final / frase.len() as u32;

    if valor_final < 10 || valor_final > 25{
        valor_final %= 25;
    }
    valor_final
}

use std::{collections::HashMap, io::Read};
use std::fs::File;

/*
El objetivo es hacer un compilador para brainfuck
Pasos de un compilador: 
1.-Analizador Lexico
2.-Analizador Sintactico
3.-Analizador Semantico
4.-Codigo intermedio
5.-Optimizador
6.-Codigo
7.-STONKS!!!

Bonus:
+ Manejador de errores
+ Admin de tabla de simbolos
*/

/*
    brainfck 	C 	
    >           ++ptr; 	
    < 	        --ptr; 	
    + 	        ++*ptr; 	
    - 	        --*ptr; 	
    . 	        putchar(*ptr); 	
    , 	        *ptr=getchar(); 	
    [ 	        while (*ptr) { 	
    ] 	        }
*/
/* 
Codigo de ejemplo en brainfck

++++++++++[>+++++++>++++++++++>+++++++++++>+++>+<<<<<-]>++.>>+.---.<---.>>++.<+.++++++++.-------.<+++.>+.>+.>.
*/
#[derive(Debug)]
struct Token{
    map: HashMap<u32, String>,
    table: HashMap<String, String>
}

impl Token {
    fn new(entries:String) -> Result<Token, std::io::Error>{
        
        let mut id:u32 = 0;
        
        let mut map: HashMap<u32, String> = HashMap::new();
        let mut table: HashMap<String, String> = HashMap::new();

        for c in entries.split("'") {
            if c.is_empty(){
                break;
            }
            else{
                let mut values = c.split(':');

                let key = values.next().expect("No Key");
                let val = values.next().expect("No Value");
                
                table.insert(String::from(key), String::from(val));
                map.insert(id, c.to_owned());
                
                id = id + 1;
            }
        }
        Ok(Token {map, table})
    }
    
}

//Fases de compilacion
fn comp(doc:String, _exe:String){

    let mut cont = String::new();
    let mut arch = File::open(doc).unwrap();
    let mut nmb: String = String::new();
    let mut nmb_prov: String = String::new();

    arch.read_to_string(&mut cont).ok();

    for character in cont.chars(){
        nmb_prov = analizer(character);
        nmb = format!("{}{}'",nmb,nmb_prov);   
    }
    let tokens = Token::new(nmb).expect("Fallo en el analisis lexico");
    println!("\nTokenisado: {:#?}", tokens)
}


fn analizer(character:char) -> String{

    let mut nmb_prov: String = String::new();

    match character{
        '>' | '<' => nmb_prov = format!("{}{}","literal:".to_owned(), character),
        '+' | '-' => nmb_prov = format!("{}{}","operador:".to_owned(), character),
        '.' | ',' => nmb_prov = format!("{}{}","identificador:".to_owned(), character),
        '[' | ']' => nmb_prov = format!("{}{}","separador:".to_owned(), character),
        _ => nmb_prov = format!("{}{}","comentario:".to_owned(), character)
    }

    return nmb_prov;
}

fn main() {

    let doc_entrada = std::env::args().nth(2).expect("Especifique una archivo porfavor");
    let action: String = std::env::args().nth(1).expect("Especifique una accion porfavor");

    match action.as_ref() {
        "-c" => {
            comp(doc_entrada, "a".to_owned()); 
        },
        "-o" => {
            let doc_salida = std::env::args().nth(3).expect("Especifique una nombre porfavor");
            comp(doc_entrada, doc_salida);
        },
        _ => println!("Instruccion invalida") 
    }
}
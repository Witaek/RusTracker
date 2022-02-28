use std::str;

const ALPHABET: &str= "#ABCDEFGHIJKLMNOPQRSTUVWXYZ#####_###############0123456789######";

//Découpe le message de 56 bits en sections
fn decoupage(&bin: &u64) -> [String;10] {               // Entrée : référence à un nombre, Sortie : Array de 10 String
    let bin_str: String = format!("{bin:056b}");        // Convertion du binaire en entrée vers un String

    let mut arr: [String;10] = Default::default();      // Définition de l'array que l'on va retourner
    arr[0] = bin_str[..5].to_owned();                   // 5 premiers bits c'est le CA
    arr[1] = bin_str[5..8].to_owned();                  // 3 prochains c'est le 

    let mut index = 2;
    for i in [8, 14, 20, 26, 32, 38, 44, 50] {          // découpe les sections de 6 bits restants
        arr[index] = bin_str[i..i+6].to_owned();
        
        index += 1;
    }

    arr
}

//Traduit les sections de message binaire à l'aide de l'alphabet
pub fn callsign(&bin: &u64) -> String {
    let arr = decoupage(&bin);                                                          // découpage du message binaire
    let mut flight_number: String = String::from("");                                   // création string mutable vide
    for elem in &arr[2..] {                                                             // On parcours les éléments de arr à partir du 2 ème (on ne se sert pas des deux premiers)
        let index: usize = usize::from_str_radix(elem, 2).expect("Not binary");         // convertit les sections de 6 bits binaire en décimal
        let letter = &ALPHABET[index..index+1];                                         // cherche la lettre correspondante dans l'alphabet
        flight_number.push_str(letter)                                                  // insert la lettre dans le string final
    };
    flight_number                                                                       // retourne le numéro du vol
}
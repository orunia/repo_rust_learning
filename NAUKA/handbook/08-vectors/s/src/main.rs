/*ZAD1 
fn main() {
    let mut v = vec!["Banan", "Jablko", "Pomarancza"];
    v.push("Winogrono");
    
    //wektor przechowuje elementy typu &str, a get zwraca referencje
    //do elementu &T, co w rezultacie daje &&str
    let second: Option<&&str> = v.get(1);
    match second{
        Some(second) => println!("{second}"),
        None => println!("Brak"),
    }
}*/

/*ZAD2
fn main(){
    let mut st = String::from("Programowanie");
    st.push_str(" w Rust");
    for v in st.bytes() {
        println!("{v} \n");
    }
}*/

/*ZAD3
use std::collections::HashMap;
fn main(){
    let mut osoba = HashMap::new();
    osoba.insert(String::from("Igor"), 20);
    osoba.insert(String::from("Karol"), 21);

    osoba.entry(String::from("Kacper")).or_insert(50);

    println!("{osoba:?}");
}*/

/*ZAD4
use std::collections::HashMap;

fn main(){
    let tekst = "niebieski czerwony niebieski zolty";
    let mut n = HashMap::new();

    for v in tekst.split_whitespace() {
        let count = n.entry(v).or_insert(0);
        *count += 1;
    }
    println!("{n:?}");
}*/

/*ZAD5
fn main(){
    let st = vec![String::from("drzewo"), String::from("ptak")];
    let n = Vec::new();

    for slowo in st {
        //bezpieczne odczytanie znaku
        let pierwszy_znak = slowo.chars().next().unwrap();
        //aby usunac pierwszy znak uzywamy chars ponownie, razem ze skip i zwracam z powrotem do typu string za pomoca collect()
        let reszta_slowa: String = slowo.chars().skip(1).collect();

        let nowe_slowo = format!("{}{}ay", reszta_slowa, pierwszy_znak);
        n.push(nowe_slowo);
    }
}*/

use std::collections::HashMap;

fn main(){
    let mut firma: HashMap<String, Vec<String>> = HashMap::new();

    //sprawdzamy czy IT istnieje, jezeli nie to daje mu now pusty wektor, insert zwraca mutowalna referencje do wyciagnietej 
    //lub nowo utworzonej wartosci, mozemy od razu zrobic push
    firma.entry(String::from("IT")).or_insert(Vec::new()).push(String::from("Igor"));
    firma.entry(String::from("IT")).or_insert(Vec::new()).push(String::from("Pawel"));
    firma.entry(String::from("Sprzedaż")).or_insert(Vec::new()).push(String::from("Kamil"));

    let szukany_dzial = String::from("IT");

    match firma.get(&szukany_dzial){
        Some(pracownicy) => {
            //Ponieważ nie możemy modyfikować danych przez niemutowalną referencję, używamy metody clone(), 
            //aby wyodrębnić te dane i stworzyć ich mutowalną kopię w nowej zmiennej o nazwie posortowani
            let mut posortowani = pracownicy.clone();
            posortowani.sort();
            println!("Pracownicy działu {}: {:?}", szukany_dzial, posortowani);
        },
        None => println!("Dzial: {}, nie istnieje w strukturze firmy", szukany_dzial),
    }
}
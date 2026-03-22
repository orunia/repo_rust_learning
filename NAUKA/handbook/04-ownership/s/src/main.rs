/*ZAD1 I 2
fn main() {
    
    let s1 = String::from("Rust");
    wez_wlasnosc(s1.clone());

    println!("{}", s1);
}

fn wez_wlasnosc(s: String) -> String {
    s
}
*/

/*ZAD 3
fn main(){
    let zmienna = String::from("chuuj");
    let dlugosc = policz_znaki(&zmienna);

    println!("{zmienna} i {dlugosc}");
}
fn policz_znaki(tekst: &String) -> usize {
    tekst.len()
}
*/

/*ZAD4
fn main(){
    let mut zmienna = String::from("chuj");
    dodaj_wykrzyknik(&mut zmienna);
    println!("{}", zmienna);
}

fn dodaj_wykrzyknik(tekst: &mut String){
    tekst.push_str("!")
}
*/

/*ZAD5 I 6
fn main(){
    let mut tekst = String::from("Chuj");
    let r1 = &tekst;
    
    println!("{r1}");
    let r2 = &mut tekst;

    println!("{r2}");
}
*/

/*ZAD 7
fn main(){
    let zdanie = String::from("Programowanie jest super");
    let prog = &zdanie[0..14];
    let spr = &zdanie[19..24];
    println!("{prog}");
    println!("{spr}");
}
    */

/*ZAD 8
 fn main(){
    let liczby = [10, 20, 30, 40, 50, 60];

    let srodek = &liczby[1..4];
    assert_eq!(srodek, &[20, 30, 40]);
}
*/

/*ZAD 9
fn main(){
    let tekst = String::from("rudy to chuj");
    wypisz_fragment(&tekst);
    wypisz_fragment(&tekst[0..5]);
    wypisz_fragment("rudy to chuj");
}
fn wypisz_fragment(tekst: &str){
    println!("{}",tekst)
}*/

/*ZAD 10 
fn main(){
    let mut tekst = String::from("rudy to chuj");
    let pierwsze_slowo = &tekst[0..5];
    tekst.clear();
    println!("{pierwsze_slowo}");
}
    */

/*ZAD 11
fn main(){
    let mut tekst = String::from("Rust to przyszlosc");
    let pierwsze = &tekst[0..5];
     println!("{pierwsze}");
    tekst.push_str(" IT");
    println!("{tekst}");
}*/

/*ZAD 12
fn main(){ 
    let tekst = String::from("Elo elo elooo");
    let mut nowy_tekst = przetworz_i_zwroc(tekst);
    dodaj_kropke(&mut nowy_tekst);
    println!("{nowy_tekst}");
}

fn dodaj_kropke(s: &mut String){
    s.push_str("!");
}

fn przetworz_i_zwroc(s: String) -> String {
    let pierwsze = &s[0..4];
    println!("{pierwsze}");
    s
}
*/
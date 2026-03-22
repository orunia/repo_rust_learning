/*ZAD1
#[derive(Debug)]
struct Zamowienie {
    produkt: String,
    ilosc: u32,
    cena_jednostkowa: f64,
}

fn main() {
    let produkt1 = Zamowienie {
        produkt: String::from("Chuj"),
        ilosc: 3,
        cena_jednostkowa: 65.99,
    };

    let produkt2 = Zamowienie {
        ilosc: 5,
        ..produkt1
    };

    println!("{:?}", produkt2);
}*/

/*ZAD2
#[derive(Debug)]
struct Wektor3d(f64, f64, f64);

struct PustySygnal;

fn main(){
    let wektor = Wektor3d(4.3, 4.4, 4.5);
    let sygnal = PustySygnal;

    let Wektor3d(x, y, z) = wektor;

    println!("{} {}  {}", x, y, z);
}*/

/*ZAD3
#[derive(Debug)]
struct KontoBankowe {
    wlasciciel: String,
    saldo: i32,
}

impl KontoBankowe {
    fn wplac(&mut self, kwota: i32){
        self.saldo += kwota
    }

    fn wypiszsaldo(&self){
        println!("{:?}", self)
    }
}

fn main(){
    let mut konto = KontoBankowe{
        wlasciciel: String::from("Igor"),
        saldo: 44,
    };

    //dobry zapis, ale nadmiarowy. w rust metody ktore sa w 
    //impl wywolujemy za pomoca kropki

    //let noweSaldo = KontoBankowe::wplac(&mut konto, 10);
    //let wypisz = KontoBankowe::wypiszsaldo(&mut konto);

    konto.wplac(10);
    konto.wypiszsaldo();
}*/

/*ZAD4
#[derive(Debug)]
struct KontoBankowe {
    wlasciciel: String,
    saldo: i32,
}

impl KontoBankowe {
    fn wplac(&mut self, kwota: i32){
        self.saldo += kwota
    }

    fn wypiszsaldo(&self){
        println!("{:?}", self)
    }

    fn otworz(imie: String) -> Self {
        Self{
         wlasciciel: imie,
         saldo: 0,
       }
    }
}

fn main(){
    let mut konto = KontoBankowe{
        wlasciciel: String::from("Igor"),
        saldo: 44,
    };

    konto.wplac(10);
    konto.wypiszsaldo();
    
    let im: String = "Chuj".to_string();
    let mut nowa = KontoBankowe::otworz(im);

    println!("{:?}", nowa);
}*/

/*ZAD5
#[derive(Debug)]
struct Silnik {
    model: String,
    moc: u32,
}

fn main(){
    let mnoznik: u32 = 2;

    let instancja = Silnik {
        model: "okej".to_string(),
        moc: dbg!(150 * mnoznik)
    };

    println!("{:#?}", instancja);
}*/

/*ZAD6
struct Magazyn {
    pojemnosc: u32,
    zajete_miejsce: u32
}

impl Magazyn {
    fn czy_zmiesci(&self, towar: u32) -> bool {
        self.pojemnosc >= self.zajete_miejsce &&
        self.pojemnosc >= towar
    }
}

fn main(){
    let mag1 = Magazyn {
        pojemnosc: 44,
        zajete_miejsce: 22,
    };

    let prawda: bool = mag1.czy_zmiesci(50);
    println!("{prawda}");
}*/

/*ZAD7
#[derive(Debug)]
struct TajnyDokument {
    tresc: String,
}

impl TajnyDokument {
    fn zniszcz_i_przeczytaj(&self) -> String {
        self.tresc.clone()
    }
}

fn main(){
    let instancja = TajnyDokument {
        tresc: "Chuuuuuuj".to_string(),
    };

    instancja.zniszcz_i_przeczytaj();
    println!("{:?}", instancja);
}*/



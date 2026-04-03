pub mod dzial_it {
    fn napraw_serwer(){}

    pub fn zresetuj_haslo(){}
}

mod dzial_hr{
    pub struct Pracownik{
        pub imie: String,
        wynagrodzenie: u32,
    }

    impl Pracownik{
        pub fn nowy(imie: &str) -> Pracownik {
            Pracownik{
                imie: imie.to_string(),
                wynagrodzenie: 100
            }
        }
    }
}

pub fn zglos_problem(){
    crate::dzial_it::zresetuj_haslo();
}

fn wyslij_raport(){}

mod dzial_sprzedazy{
    mod raportowanie{
        fn zakoncz_miesiac(){
             super::super::wyslij_raport();
         }
    }
}


fn main(){}
use firma::dzial_it::zresetuj_haslo;
use std::fmt::Result;
use std::io::Result as IoResult;

fn f1() -> IoResult<()> {
    Ok(())
}

fn f2() -> Result {
    Ok(())
}

fn main() {
    zresetuj_haslo();
}

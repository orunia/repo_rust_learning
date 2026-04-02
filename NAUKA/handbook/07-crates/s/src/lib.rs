// Moduł główny - Przód restauracji
mod front_of_house {
    
    // Submoduł - Obsługa gości na wejściu
    mod hosting {
        fn add_to_waitlist() {} // dodaj do listy oczekujących

        fn seat_at_table() {} // posadź przy stoliku
    }

    // Submoduł - Obsługa kelnerska
    mod serving {
        fn take_order() {} // przyjmij zamówienie

        fn serve_order() {} // wydaj zamówienie

        fn take_payment() {} // przyjmij płatność
    }
}
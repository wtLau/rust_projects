use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort()
    }
}

fn show_non_ref(table: Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn show_w_ref(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madriglas".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    // show_non_ref(table);
    // Sort table by works with "mutable reference"
    sort_works(&mut table);

    // Display tables with "Shared Reference"
    show_w_ref(&table)
}

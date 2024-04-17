use core::time;
use std::thread;

#[derive(Debug)]
struct Tomagochi {
    name: String,
}

impl Tomagochi {
    fn new_tomagochi(name: String) -> Tomagochi {
        Tomagochi { name }
    }
    fn display(&self) {
        let frames: Vec<Vec<&str>> = vec![
            vec![
                "   /\\~~/\\    _~,",
                "  / @  @ \\  // U",
                " >   ^    < ||",
                "  \\  UU  / //",
                "   C()_()D_/",
                //"  {}",
            ],
            vec![
                "   /\\~~/\\     _~,",
                "  / -  - \\   // U",
                " >   ^    <  ||",
                "  \\  UU  /  //",
                "   C()_()D_ /",
                //"  {}",
            ],
        ];

        for frame in frames.iter().cycle() {
            for line in frame {
                println!("{}", line.replace("{}", &self.name));
            }
            thread::sleep(time::Duration::from_millis(2000));

            print!("\x1B[2J\x1B[H");
            //print!("{}[2J", 27 as char);
            //print!("{}[H", 27 as char);
        }
    }
}

fn main() {
    let my_pet = Tomagochi::new_tomagochi(String::from("Rustity"));

    my_pet.display();
}

fn main() {
    let test = "Bonjour comment allez-vous monsieur?";
    println!("{}", get_string_between(&test, 'B', 'o', 1, 3));
    let test2 = separate_string_after_n(&test, ' ', 4);
    println!("{}", test2);

    let worker: Worker = Worker {
        name : String::from("Roger"),
        department : String::from("research"),
        entreprise : String::from("Nasa"),
        mail: String::from("roger-research-1@nasa.com"),
        id: 1,
    };
    worker.introduce();

    let worker_by_mail = Worker::instanciate_with_mail("edmond-cofee-0@inra.com");
    worker_by_mail.introduce();
}

/*
    Créer une fonction qui retourne la partie d'une string avant séparateur 

*/
/*
fn get_first_word(full_string: &str, c: char) -> &str{
    let bytes = full_string.as_bytes();
    let c = c as u8;
    for (i, &item) in bytes.iter().enumerate() {
        if item == c {
            return &full_string[0..i];
        }
    }
    &full_string
}
*/

/*Créer une fonction qui renvoie un vecteur de strings en spécifiant une string source et un séparateur */
/*
fn separate_string(full_string: &str, c: char) -> Vec<&str> {
    let mut last_index = 0;
    let bytes = full_string.as_bytes();
    let c = c as u8;
    let mut vec_test: Vec<&str> = Vec::new();
    for(i, &item) in bytes.iter().enumerate() {
        if item == c {
            vec_test.push(&full_string[last_index..i]);
            last_index = i+1;
        } else if i == bytes.len() - 1 {
            vec_test.push(&full_string[last_index..i+1]);
        }
    }
    vec_test
}
*/

/* Créer une fonction pour splice une string après n itérations du séparateur

    Todo : Rajouter un caractère limite
           Gérer après le dernier separateur
 */ 
fn separate_string_after_n(full_string: &str, c: char, nb: u64) -> &str {
    let mut last_index = 0;
    let mut counter = 0;
    let bytes = full_string.as_bytes();
    let c = c as u8;
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == c {
            counter += 1;
            if counter == nb {
                return &full_string[last_index..i];
            } 
            last_index = i+1;
        } else if i == bytes.len() - 1 {
            return &full_string[last_index..i+1];
        }
    }
    "Not found"
}

/* Créer une fonction qui renvoie une String comprise entre deux caractères (et le nombre d'occurences des deux caractères) */
fn get_string_between(full_string: &str, begin_char: char, ending_char: char, nb_begin_char: u64, nb_ending_char: u64) -> &str {
    let mut last_index = 0;
    let mut counter_begin_char = 0;
    let mut counter_ending_char = 0;
    let bytes = full_string.as_bytes();
    let begin_char = begin_char as u8;
    let ending_char = ending_char as u8;

    // If characters are different
    // if ending_char != begin_char {
        for (i, &item) in bytes.iter().enumerate() {
            if item == begin_char {
                // If we are not at the nb of char, we inc.
                if counter_begin_char != nb_begin_char {
                    counter_begin_char += 1;
                }
            } else if item == ending_char {
                counter_ending_char += 1;
                if counter_ending_char == nb_ending_char {
                    return &full_string[last_index..i];
                }
            }
        }
    // }
    return full_string;
}

/* Créer une struct Worker avec les attributs name, department, id, entreprise et mail */
struct Worker {
    name: String,
    department: String,
    id: u64,
    entreprise: String,
    mail: String,
}

impl Worker {
    fn introduce(&self) {
        println!("Hello, my name is {}, i'm working at {} in the {} department", self.name, self.entreprise, self.department)
    }
    fn instanciate_with_mail(mail: &str) -> Worker {
        let w_name = separate_string_after_n(mail, '-', 1);
        let w_department = separate_string_after_n(mail, '-', 2);
        let w_entreprise = "Inrae";
        let w_id = 0;
        let worker: Worker = Worker { 
            name: String::from(w_name), 
            department: String::from(w_department), 
            id: w_id, 
            entreprise: String::from(w_entreprise), 
            mail: String::from(mail) 
        };

        worker
    }
}


// /* Créer une fonction pour instancier une struct uniquement grâce au mail sous la forme nom-department-id@entreprise.com */
// fn create_worker(mail: String) -> Worker {
//     let name: &str
// }
fn main() {
    let test = "Bonjour comment allez-vous monsieur?";
    println!("{}", get_string_between(&test, 'B', 'o', 1, 3));
    let test2 = separate_string_after_n(&test, ' ', 4);
    println!("{}", test2);
    
    let me = Worker::instanciate_with_mail("erwann-validation-598@stim.com");
    me.introduce();
    println!("{:?}", me);
    let tony: Worker = Worker {
        mail: String::from("tony-validation-599@stim.com"),
        id: 599,
        name: String::from("Tony"),
        ..me
    };
    tony.introduce();
    println!("{:?}", tony);

    let home = IpAddress::V4(127,0,0,1);
    println!()
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
                counter_begin_char += 1;
                // If we are not at the nb of char, we inc.
                if counter_begin_char == nb_begin_char {
                    last_index = i+1;
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


/* Première lettre en maj */
fn capitalize(s: &str) -> String{
    match s.chars().next() {
        Some(c) => c.to_uppercase().to_string() + &s[1..],
        None => "".to_string(),
    }
}



/* Créer une struct Worker avec les attributs name, department, id, entreprise et mail */
#[derive(Debug)]
struct Worker {
    name: String,
    department: String,
    id: u64,
    entreprise: String,
    mail: String,
}

impl Worker {
    fn introduce(&self) {
        println!("Hello, my name is {}, i'm working at {} in the {} department, I'm identified with ID {} and my mail is {}",
            self.name, 
            self.entreprise, 
            self.department,
            self.id,
            self.mail
        );
    }
    fn instanciate_with_mail(mail: &str) -> Worker {
        let name = capitalize(separate_string_after_n(mail, '-', 1));
        let department = separate_string_after_n(mail, '-', 2);
        let entreprise = get_string_between(mail, '@', '.', 1, 1);
        let id = get_string_between(mail, '-', '@', 2, 1).parse().expect("Error converting string to integer");
        let worker: Worker = Worker { 
            name: String::from(name), 
            department: String::from(department), 
            id: id, 
            entreprise: String::from(entreprise), 
            mail: String::from(mail) 
        };

        worker
    }
}


enum IpAddress {
    V4(u8,u8,u8,u8),
    V6(String),
}
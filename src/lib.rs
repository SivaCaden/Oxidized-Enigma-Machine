use std::collections::HashMap;


const B: &str = "YRUHQSLDPXNGOKMIEBFZCWVJAT";
const C: &str = "FVPJIAOYEDRZXWGCTKUQSBNMHL";

#[derive(Debug)]
pub struct Enigma {
    pub plugboard: Plugboard,
    pub houseing: Houseing,
}

impl Enigma {
    pub fn new(houseing: Houseing, plugboard: Plugboard) -> Enigma {
        Enigma {
            plugboard,
            houseing,
        }
    }

    fn is_valid_msg(&self, msg: &str) -> bool {
        for c in msg.chars() {
            match c {
                'A'..='Z' => return true,
                _ => return false,
            }
        }
        false
    }

    pub fn encode(&mut self, msg: &str) -> String {
        if !self.is_valid_msg(msg) {
            panic!("Invalid message");
        }
        let mut output = String::new();

        for c in msg.chars() {
            let mut encoded = self.plugboard.encode(c);
            encoded = self.houseing.encode(encoded);
            encoded = self.plugboard.encode(encoded);
            output.push(encoded);
        }

        output
    }

}



#[derive(Debug)]
pub struct Plugboard {
    input: HashMap<char, char>,
}

impl Plugboard {
    pub fn new() -> Plugboard {
        Plugboard {
            input: HashMap::new(),
        }
    }

    pub fn connect(&mut self, a: char, b: char) {
        match self.input.get(&a) {
            Some(_) => {
                println!("{} is already connected", a);
            }
            None => {
                self.input.insert(a, b);
                self.input.insert(b, a);
            }
        }
    }

    pub fn encode(&self, c: char) -> char {
        match self.input.get(&c) {
            Some(&c) => c,
            None => c,
        }
    }

 

}

#[derive(Debug)]
pub struct Rotor {
    input: Vec<char>, 
    output: Vec<char>,
    notch: char,
}

impl Rotor {
    pub fn new(output: Vec<char>, rotor_number: &str) -> Rotor {
        Rotor {
            input: ('A'..='Z').collect(),
            output,
            notch: match rotor_number {
                "I" => 'Q',
                "II" => 'E',
                "III" => 'V',
                "IV" => 'J',
                "V" => 'Z',
                _ => 'M',
            },
        }
    }
    pub fn encode_foward(&self, c: char) -> char {
        let index = self.input.iter().position(|&x| x == c).unwrap();
        self.output[index]
    }
    pub fn encode_backward(&self, c: char) -> char {
        let index = self.output.iter().position(|&x| x == c).unwrap();
        self.input[index]
    }

    pub fn rotate(&mut self) {
        let last = self.output.pop().unwrap();
        self.output.insert(0, last);
    }
    pub fn at_notch(&self) -> bool {
        self.input[0] == self.notch
    }

    pub fn clone(&self) -> Rotor {
        let rotor_number = {
            match self.notch {
                'Q' => "I",
                'E' => "II",
                'V' => "III",
                'J' => "IV",
                'Z' => "V",
                _ => "I",
            }
        };
        let output = self.output.clone();
        Rotor::new(output, rotor_number)
    }
}

#[derive(Debug)]
pub struct Reflector {
    wireing: HashMap<char, char>,
    id: char,
}

impl Reflector {
    pub fn new(output: Vec<char>, index: i32) -> Reflector {
        Reflector {
            wireing: {
                let mut wireing = HashMap::new();
                let mut index = 0;
                while index < output.len() -1 {
                    wireing.insert(output[index], output[index + 1]);
                    wireing.insert(output[index + 1], output[index]);
                    index += 2;
                }
                wireing
            },
            id: 
                match index {
                    0 => 'B',
                    _ => 'C',
                },
        }
    }

    pub fn encode(&self, c: char) -> char {
        match self.wireing.get(&c) {
            Some(&c) => c,
            None => c,
        }
    }
    pub fn get_id(&self) -> char {
        self.id
    }

    pub fn copy(&self) -> Reflector {
        

        let b_vec = B.chars().collect();
        let c_vec = C.chars().collect();

        let output =  {
            match self.id {
                'B' => b_vec,
                _ => c_vec
            }
        };
        let index = {
            match self.id {
                'B' => 0,
                _ => 1,
            }
        };
        Reflector::new(output,index)

    }


}


#[derive(Debug)]
pub struct Houseing {
    r_one: Rotor,
    r_two: Rotor,
    r_three: Rotor,
    reflector: Reflector,
}
impl Houseing {
    pub fn new(r_one: Rotor, r_two: Rotor, r_three: Rotor, reflector: Reflector) -> Houseing {
        Houseing {
            r_one,
            r_two,
            r_three,
            reflector,
        }
    }
    fn rotate(&mut self) {
        if self.r_two.at_notch() {
            self. r_three.rotate()
        }
        if self.r_one.at_notch() {
            self.r_two.rotate()
        }
        self.r_one.rotate()
    }


    fn encode_foward(&mut self, letter : char) -> char {
        let mut output = letter;
        output = self.r_one.encode_foward(output);
        output = self.r_two.encode_foward(output);
        self.r_three.encode_foward(output)
    }
    fn encode_backward(&mut self, letter : char) -> char {
        let mut output = letter;
        output = self.r_three.encode_backward(output);
        output = self.r_two.encode_backward(output);
        self.r_one.encode_backward(output)
    }


    pub fn encode(&mut self, letter: char) -> char {
        self.rotate();
        let encoded_pass_one = self.encode_foward(letter);
        let encoded_pass_two = self.reflector.encode(encoded_pass_one);
        let encoded_pass_three = self.encode_backward(encoded_pass_two);
        
        encoded_pass_three
    }

}
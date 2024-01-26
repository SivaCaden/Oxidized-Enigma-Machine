use std::collections::HashMap;
use std::fs;
use enigma::Houseing;
use enigma::Plugboard;
use enigma::Rotor;
use enigma::Reflector;
use enigma::Enigma;


fn main() {

    man_test();

    let mut enigma = setup();
    let message = "CRINGE";
    let encoded = enigma.encode(message);
    println!("Encoded message: {}", encoded);

}

fn setup() -> Enigma {

    println!("please supply 5 plugboard connections");
    println!("Ex: AB UG ED QW CL");
    let mut input = String::new();
    // read user input
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let plug_vec: Vec<&str> = input.split_whitespace().collect();

    let mut plugboard = Plugboard::new();
    for connection in plug_vec {
        let connection = connection.trim();
        let mut chars = connection.chars();
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        plugboard.connect(first, second);
    }


    println!("Please select 3 Rotor numbers from 1-8");
    println!("Rotor will be placed in order from left to right");
    println!("Example: 1 5 3");
    print!(">>: ");
    let mut input = String::new();
    // read user input
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // parse rotors
    let input_vec = input.split_whitespace();
    let mut rotor_numbers: Vec<usize> = Vec::new();
    for number in input_vec {
        let number: usize = number.parse().unwrap();
        rotor_numbers.push(number);
    }


    // select rotors from rotors
    let rotors: Vec<Rotor> = create_rotors();
    let rotor_one: Rotor = rotors[rotor_numbers[0] - 1].clone();
    let rotor_two: Rotor = rotors[rotor_numbers[1] - 1].clone();
    let rotor_three: Rotor = rotors[rotor_numbers[2] - 1].clone();

    println!("please select a relector configuration");
    println!("1. B");
    println!("2. C");
    // read user input
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let input: &str = input.trim();

    let reflectors: Vec<Reflector> = create_relfectors();
    let reflector = {
        match input {
            "B" => reflectors[0].copy(),
            _ => reflectors[1].copy(),
        }
    };


    let houseing: Houseing = Houseing::new(rotor_one, rotor_two, rotor_three, reflector);

    Enigma::new(houseing, plugboard)

}

fn man_test() {
    hand_testing_of_rotors_and_reflectors();
    
    let rotors = create_rotors();
    let reflectors = create_relfectors();
    let mut plugboard = Plugboard::new();

    plugboard.connect('V', 'G');
    plugboard.connect('T', 'L');
    plugboard.connect('A', 'B');

    let rotor_one = rotors[0].clone();
    let rotor_two = rotors[1].clone();
    let rotor_three = rotors[2].clone();

    let reflector = reflectors[0].copy(); 

    let houseing: Houseing = Houseing::new(rotor_one, rotor_two, rotor_three, reflector);

    let message = "HELLOWORLD";
    let encoded = "FBKWVNXJFW";

    let mut enigma = Enigma::new(houseing, plugboard);

    let encode = false;

    let outmessage: String = {
        match encode {
            true => enigma.encode(message),
            false => enigma.encode(encoded),
        }
    };


    println!("Encoded message: {}", outmessage);



}

fn hand_testing_of_rotors_and_reflectors() {
    let rotor_one = Rotor::new(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'], "I");
    let rotor_two = rotor_one.clone();
    let rotor_three = rotor_one.clone();

    let reflector: HashMap<char, char> = {
        let mut wireing = HashMap::new();
        wireing.insert('A', 'Y');
        wireing.insert('Y', 'A');
        wireing.insert('B', 'R');
        wireing.insert('R', 'B');
        wireing.insert('C', 'U');
        wireing.insert('U', 'C');
        wireing.insert('D', 'H');
        wireing.insert('H', 'D');
        wireing.insert('E', 'Q');
        wireing.insert('Q', 'E');
        wireing.insert('F', 'S');
        wireing.insert('S', 'F');
        wireing.insert('G', 'L');
        wireing.insert('L', 'G');
        wireing.insert('I', 'P');
        wireing.insert('P', 'I');
        wireing.insert('J', 'X');
        wireing.insert('X', 'J');
        wireing.insert('K', 'N');
        wireing.insert('N', 'K');
        wireing.insert('M', 'O');
        wireing.insert('O', 'M');
        wireing.insert('T', 'Z');
        wireing.insert('Z', 'T');
        wireing.insert('V', 'W');
        wireing.insert('W', 'V');
        wireing
    };

    let encode_test_letter: char = 'A';
    let expected_encoded_letter: char = 'U';

    let decode_test_letter: char = 'U';
    let expected_decoded_letter: char = 'A';

    test_encode_slash_decode(rotor_one.clone(), rotor_two.clone(), rotor_three.clone(), &reflector, encode_test_letter, expected_encoded_letter);
    println!("Test Passed: encode");
    test_encode_slash_decode(rotor_one.clone(), rotor_two.clone(), rotor_three.clone(), &reflector, decode_test_letter, expected_decoded_letter);
    println!("Test Passed: decode");
    

}

fn test_encode_slash_decode(r_one: Rotor, r_two: Rotor, r_three: Rotor, reflector: &HashMap<char, char>, test: char, expected: char) {
    let mut rotor_one = r_one;
    let mut rotor_two = r_two;
    let mut rotor_three = r_three;

    { // Rotate code
        if rotor_two.at_notch() {
            rotor_three.rotate()
        }
        if rotor_one.at_notch() {
            rotor_two.rotate()
        }
        rotor_one.rotate()
    }
    let letter_one = rotor_one.encode_foward(test);
    let letter_two = rotor_two.encode_foward(letter_one);
    let letter_three = rotor_three.encode_foward(letter_two);
        
    let reflected = reflector.get(&letter_three).unwrap();

    let letter_four = rotor_three.encode_backward(*reflected);
    let letter_five = rotor_two.encode_backward(letter_four);
    let letter_six = rotor_one.encode_backward(letter_five);
   

    assert_eq!(expected, letter_six);
}



fn create_rotors() -> Vec<Rotor> {
    // let mut rotors: Vec<Rotor> = Vec::new();

    let mut all_rotors: Vec<Rotor> = Vec::new();

    let file = read_file("src/rotors.txt");
    let lines = file.lines();
    let mut index: usize = 0;

    for line in lines.enumerate() {
        let roman = to_roman(index + 1);
        let mut output = Vec::new();
        for c in line.1.chars() {
            output.push(c);
        }
        
        let rotor = Rotor::new(output, &roman);
        all_rotors.push(rotor);
        index += 1;
    }
    all_rotors

}

fn create_relfectors() -> Vec<Reflector> {
    let mut reflectors: Vec<Reflector> = Vec::new();
    let file = read_file("src/reflectors.txt");

    let lines = file.lines();
    let mut index = 0;

    for line in lines.enumerate() {
        let mut output = Vec::new();
        for c in line.1.chars() {
            output.push(c);
        }
        let reflector = Reflector::new(output, index);
        reflectors.push(reflector);
        index += 1;
    }
    reflectors
}

fn to_roman(number: usize) -> String {
    let mut result = String::new();
    let mut n = number;

    let arabic = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let roman = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

    for (i, arabic) in arabic.iter().enumerate() {
        while n >= *arabic {
            result.push_str(roman[i]);
            n -= arabic;
        }
    }
    result
}

fn read_file(filename: &str) -> String { fs::read_to_string(filename).expect("Could not read file") }
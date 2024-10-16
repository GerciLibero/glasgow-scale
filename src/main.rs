use::std::io;

fn main() {
    glasgow_scale_sum(eye_opening(), motor_response(), verbal_response(), pupillary_reactivity());
}

fn glasgow_scale_sum(eye_opening: u32, motor_response: u32, verbal_response: u32, pupillary_reactivity: u32) {
    let sum_galsgow_scale_values: u32 = eye_opening + motor_response + verbal_response + pupillary_reactivity;

    let injury_degree_according_glasgow_scale = match sum_galsgow_scale_values {
        13..=15 => "leve",
        9..=12 => "moderada",
        3..=8 => "grave",
        0..=2 => "coma",
        _ => "erro ao calcular os valores da escala de glasgow"
    };

    println!("\n O cálculo final da escala de glasgow é {sum_galsgow_scale_values}.\n O grau de lesão de acordo com a pontuação da escala é {injury_degree_according_glasgow_scale}.")
}

fn user_input_value() -> u32 { 
    let input_invalid_number_error: u32 = 9;
    
    let mut input_value_string: String = String::new();

    io::stdin()
        .read_line(&mut input_value_string)
        .expect("Failed to read input value");

    let input_value_number: u32 = match input_value_string.trim().parse() {
        Ok(number) => number,
        Err(_) => input_invalid_number_error,
    };

    input_value_number
}

fn eye_opening() -> u32 {    
    println!("\n Digite o número que corresponde à classificação da resposta de abertura ocular do paciente: \n Espontânea - 4 \n À voz - 3 \n À dor - 2 \n Nenhuma - 1");
    
    let eye_opening_user_input: u32 = user_input_value();

    let is_valid: bool = match eye_opening_user_input {
       1..=4 => true,
        _ => false
     };
    
    if is_valid {
        println!("\n {eye_opening_user_input} é um número válido na escala de abertura ocular.");
        eye_opening_user_input
    } else {
        println!("\n {eye_opening_user_input} não é um número válido, a escala de abertura ocular aceita apenas valores entre 1 e 4.\n Por favor, insira um valor válido.");
        eye_opening()
    }
}

fn verbal_response() -> u32 {
        println!("\n Digite o número que corresponde à classificação da resposta verbal do paciente: \n Orientada - 5 \n Confusa - 4 \n Palavras - 3 \n Sons - 2 \n Ausente - 1");
    
        let verbal_response_user_input: u32 = user_input_value();
    
        let is_valid: bool = match verbal_response_user_input {
            1..=5 => true,
            _ => false
         };
        
        if is_valid {
            println!("\n {verbal_response_user_input} é um número válido na escala de resposta verbal.");
            verbal_response_user_input
        } else {
            println!("\n {verbal_response_user_input} não é um número válido, a escala de resposta verbal aceita apenas valores entre 1 e 5. \n Por favor, insira um valor válido.");
            verbal_response()
        }
}

fn motor_response() -> u32 {
    println!("\n Digite o número que corresponde à classificação da resposta motora do paciente: \n A ordens - 6 \n Localizadora - 5 \n Flexão normal - 4 \n Flexão anormal - 3 \n Extensão - 2 \n Ausente - 1");

    let motor_response_user_input: u32 = user_input_value();

    let is_valid: bool = match motor_response_user_input {
        1..=6 => true,
        _ => false
     };
    
    if is_valid {
        println!("\n {motor_response_user_input} é um número válido na escala de resposta verbal.");
        motor_response_user_input
    } else {
        println!("\n {motor_response_user_input} não é um número válido, a escala de resposta verbal aceita apenas valores entre 1 e 5. \n Por favor, insira um valor válido.");
        motor_response()
    }
}

fn pupillary_reactivity() -> u32 {
    println!("\n Digite o número que corresponde à classificação da reatividade pupilar do paciente: \n Bilateral - 0 \n Unilateral - 1 \n Inexistente - 2");

    let pupillary_reactivity_user_input: u32 = user_input_value();

    let is_valid: bool = match pupillary_reactivity_user_input {
        0..2 => true,
        _ => false
     };
    
    if is_valid {
        println!("\n {pupillary_reactivity_user_input} é um número válido na escala de reatividade pupilar.");
        pupillary_reactivity_user_input
    } else {
        println!("\n {pupillary_reactivity_user_input} não é um número válido, a escala de reatividade pupilar aceita apenas valores entre 0 e 5. \n Por favor, insira um valor válido.");
        pupillary_reactivity()
    }
} 

use std::io;

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

    println!("O cálculo final da escala de glasgow é {sum_galsgow_scale_values}.\n O grau de lesão de acordo com a pontuação da escala é {injury_degree_according_glasgow_scale}.")
}

fn eye_opening() -> u32 {    
    println!("Digite o número que corresponde à classificação da resposta de abertura ocular do paciente: \n Espontânea - 4 \n À voz - 3 \n À dor - 2 \n Nenhuma - 1");
    
    let mut eye_opening_value: String = String::new();

    io::stdin()
        .read_line(&mut eye_opening_value)
        .expect("Falha na leitura do valor inserido na escala de abertura ocular.");

    let eye_opening_value: u32 = match eye_opening_value.trim().parse() {
    Ok(num) => num,
    Err(_) => 0,
    };

    let is_valid: bool = match eye_opening_value {
        1..=4 => true,
        _ => false
     };
    
    if is_valid {
        println!("{eye_opening_value} é um número válido na escala de abertura ocular.");
        eye_opening_value
    } else {
        println!("{eye_opening_value} não é um número válido, a escala de abertura ocular aceita apenas valores entre 1 e 4. \n Por favor, insira um valor válido.");
        eye_opening()
    }
}

fn verbal_response() -> u32 {
        println!(
            "Digite o número que corresponde à classificação da resposta verbal do paciente: \n Orientada - 5 \n Confusa - 4 \n Palavras - 3 \n Sons - 2 \n Ausente - 1");
    
        let mut verbal_response_value: String = String::new();
    
        io::stdin()
            .read_line(&mut verbal_response_value)
            .expect("Falha na leitura do valor inserido na escala de resposta verbal.");
    
        let verbal_response_value: u32 = match verbal_response_value.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
        };
    
        let is_valid: bool = match verbal_response_value {
            1..=5 => true,
            _ => false
         };
        
        if is_valid {
            println!("{verbal_response_value} é um número válido na escala de resposta verbal.");
            verbal_response_value
        } else {
            println!("{verbal_response_value} não é um número válido, a escala de resposta verbal aceita apenas valores entre 1 e 5. \n Por favor, insira um valor válido.");
            verbal_response()
        }
}

fn motor_response() -> u32 {
    println!("Digite o número que corresponde à classificação da resposta motora do paciente: \n A ordens - 6 \n Localizadora - 5 \n Flexão normal - 4 \n Flexão anormal - 3 \n Extensão - 2 \n Ausente - 1");

    let mut motor_reponse_value: String = String::new();

    io::stdin()
        .read_line(&mut motor_reponse_value)
        .expect("Falha na leitura do valor inserido na escala de resposta verbal.");

    let motor_reponse_value: u32 = match motor_reponse_value.trim().parse() {
    Ok(num) => num,
    Err(_) => 0,
    };

    let is_valid: bool = match motor_reponse_value {
        1..=6 => true,
        _ => false
     };
    
    if is_valid {
        println!("{motor_reponse_value} é um número válido na escala de resposta verbal.");
        motor_reponse_value
    } else {
        println!("{motor_reponse_value} não é um número válido, a escala de resposta verbal aceita apenas valores entre 1 e 5. \n Por favor, insira um valor válido.");
        motor_response()
    }
}

fn pupillary_reactivity() -> u32 {
    println!("Digite o número que corresponde à classificação da reatividade pupilar do paciente: \n Bilateral - 0 \n Unilateral - 1 \n Inexistente - 2");

    let mut pupillary_reactivity_value: String = String::new();

    io::stdin()
        .read_line(&mut pupillary_reactivity_value)
        .expect("Falha na leitura do valor inserido na escala de reatividade pupilar.");

    let err_input_value: u32 = 9;

    let pupillary_reactivity_value: u32 = match pupillary_reactivity_value.trim().parse() {
    Ok(num) => num,
    Err(_) => err_input_value,
    };

    let is_valid: bool = match pupillary_reactivity_value {
        0..2 => true,
        _ => false
     };
    
    if is_valid {
        println!("{pupillary_reactivity_value} é um número válido na escala de reatividade pupilar.");
        pupillary_reactivity_value
    } else {
        println!("{pupillary_reactivity_value} não é um número válido, a escala de reatividade pupilar aceita apenas valores entre 0 e 5. \n Por favor, insira um valor válido.");
        pupillary_reactivity()
    }
} 


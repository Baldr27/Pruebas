use derivative::Derivative;

#[derive(Derivative,Clone)]
#[derivative(Debug,Default)]
pub struct Test{
    pub no_modelos: i32,
    pub no_preguntas: i32,
    pub respuestas: Vec<String>,
    #[derivative(Default(value="3"))]
    pub val_correcta: i32,
    #[derivative(Default(value="-1"))]
    pub val_incorrecta: i32,
}

impl Test{
    pub fn new(no_modelos: i32, no_preguntas: i32, respuestas: Vec<String>, val_correcta: i32, val_incorrecta: i32)-> Test{
        Test{no_modelos, no_preguntas, respuestas, val_correcta, val_incorrecta}
    }
    pub fn corrige_modelo(&self, modelo: usize, respuestas: &str)-> i32{
        let correctas: Vec<char> = self.respuestas.get(modelo-1).unwrap().chars().collect();
        let resp: Vec<char> = respuestas.to_owned().chars().collect();
        let mut puntuacion: i32 = 0;

        for i in 0..self.no_preguntas{
            if resp[i as usize].eq(&correctas[i as usize]){
                puntuacion+=self.val_correcta;
            }else{
                if resp[i as usize].eq(&' '){
                    puntuacion+=0;
                }else{
                    puntuacion+=self.val_incorrecta;
                }
            }
        }
        return puntuacion;
    }
}

#[derive(Derivative,Eq, Hash, PartialEq)]
#[derivative(Debug,Default)]
pub struct Alumno{
    pub nombre: String,
    pub modelo: i32,
    pub respuesta: String,
}

impl Alumno {
    pub fn new(nombre: String, modelo: i32, respuesta: String)-> Alumno{
        Alumno{nombre, modelo, respuesta}
    }
    pub fn corrige(&self, test: Test)-> i32{
        test.corrige_modelo(self.modelo as usize, self.respuesta.as_str())
    }
}

use rand::Rng;
use random_string::generate;
use std::collections::HashMap;
pub struct SimTest{
    pub test: Test,
}

impl SimTest{
    pub fn new(test: Test)-> SimTest{
        SimTest{test}
    }
    pub fn simula(&self, no_alumnos: i32){
        let mut alumnos: Vec<Alumno> = Vec::new();
        let mut resultados: HashMap<i32, Vec<Alumno>> = HashMap::new();
        let mut rng = rand::thread_rng();
        let charset = "ABCD ";
        for i in 0..no_alumnos{
            let nombre = format!("Alumno {}",i+1);
            let modelo = rng.gen_range(1..self.test.no_modelos+1);
            let respuesta = generate(self.test.no_preguntas as usize,charset);
            let alumno = Alumno::new(nombre,modelo,respuesta);
            let puntuacion = alumno.corrige(self.test.clone());
            resultados.entry(puntuacion).or_default().push(alumno);
        }
        println!("{:?}",resultados);
    }
}
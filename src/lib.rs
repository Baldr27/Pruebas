use derivative::Derivative;
use rand::Rng;
use random_string::generate;
use std::{collections::HashMap, fs::{File, read_link}, path::Path, io::Write};
use itertools::Itertools;
use std::fs;
use std::io::{BufReader,Read};
use pad::{self, PadStr};

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
        let respuestas_alumno = respuestas.pad_to_width(self.no_preguntas as usize);
        let correctas: Vec<char> = self.respuestas.get(modelo-1).unwrap().chars().collect();
        let resp: Vec<char> = respuestas_alumno.to_owned().chars().collect();
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


pub struct SimTest{
    pub test: Test,
    pub resultados: HashMap<i32,Vec<Alumno>>,
}

impl SimTest{
    pub fn new(test: Test)-> SimTest{
        SimTest{test, resultados: HashMap::new()}
    }
    pub fn simula(&mut self, no_alumnos: i32){
        let mut rng = rand::thread_rng();
        let charset = "ABCD ";
        for i in 0..no_alumnos{
            let nombre = format!("Alumno {}",i+1);
            let modelo = rng.gen_range(1..self.test.no_modelos+1);
            let respuesta = generate(self.test.no_preguntas as usize,charset);
            let alumno = Alumno::new(nombre,modelo,respuesta);
            let puntuacion = alumno.corrige(self.test.clone());
            self.resultados.entry(puntuacion).or_default().push(alumno);
        }
    }
    pub fn listado(&self){
        println!("{0: <10}        {1: <10}","Puntuación","Num.Alumnos");
        for (p,a) in self.resultados.iter().sorted_by_key(|x| x.0){
            println!("    {0: <10}         {1: <10}",p, a.len());
        }
    }
}

pub struct ListaTest{
    pub entrada: String,
    pub salida: String,
}

impl ListaTest{
    pub fn new(entrada: String, salida: String)-> ListaTest{
        ListaTest{entrada, salida}
    }
    pub fn generar_simulacion(&self){
        let test = self.generar_test();
        let alumnos: Vec<Alumno> = self.generar_alumnos();

        self.generar_salida(test, alumnos);
    }
    fn generar_test(&self)-> Test{
        let archivo_entrada = File::open(&self.entrada).expect("Error al leer.");
        let mut info_entrada = String::new();
        let mut br = BufReader::new(archivo_entrada);
        br.read_to_string(&mut info_entrada).expect("No se pudo leer.");
        let mut content: Vec<&str> = Vec::new();
        for line in info_entrada.lines(){
            content.push(line);
        }

        let no_modelos = content[2].parse::<i32>().unwrap();
        let no_preguntas = content[3].parse::<i32>().unwrap();
        let val_correcta = content[0].parse::<i32>().unwrap();
        let val_incorrecta = content[1].parse::<i32>().unwrap();
        let respuestas: Vec<String> = content[4..(no_modelos+4) as usize].iter().map(|x| x.to_string()).collect();//Vector de respuestas
        Test::new(no_modelos, no_preguntas, respuestas, val_correcta, val_incorrecta)
    }
    fn generar_alumnos(&self)-> Vec<Alumno>{
        let archivo_entrada = File::open(&self.entrada).expect("Error al leer.");
        let mut info_entrada = String::new();
        let mut br = BufReader::new(archivo_entrada);
        br.read_to_string(&mut info_entrada).expect("No se pudo leer.");
        let mut content: Vec<&str> = Vec::new();
        for line in info_entrada.lines(){
            content.push(line);
        }
        
        let info_alumnos: Vec<String> = content[8..].iter().map(|x| x.to_string()).collect();
        let mut alumnos: Vec<Alumno> = Vec::new();//Vector de alumnos
        for i in 0..info_alumnos.len(){
            alumnos.push(Alumno::new(info_alumnos[i].split(":").collect::<Vec<&str>>()[2].to_string(),
            info_alumnos[i].split(":").collect::<Vec<&str>>()[0].to_string().parse::<i32>().unwrap(),
            info_alumnos[i].split(":").collect::<Vec<&str>>()[1].to_string()));
        }
        return alumnos;
    }
    fn generar_salida(&self, test: Test, alumnos: Vec<Alumno>){
        let mut archivo_salida = File::create(&self.salida).expect("Error al crear.");
        let header = format!("{0: <10}                  {1: <10}","Nombre Alumno","Puntos");
        archivo_salida.write_all(header.as_bytes()).expect("Error al escribir.");
        let sorted_alumnos: Vec<&Alumno> = alumnos.iter().sorted_by(|a,b| a.nombre.cmp(&b.nombre)).collect_vec();
        for alumno in sorted_alumnos{
            let test1 = test.clone();
            let formato = format!("\n{0: <10}              {1: <10}",alumno.nombre, alumno.corrige(test1));
            archivo_salida.write_all(formato.as_bytes()).expect("Error al escribir.");
        }
    }
}
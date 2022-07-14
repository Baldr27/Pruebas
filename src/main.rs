//Erick Martinez
//German Jara
use rand::Rng;
use random_string::generate;
fn main() {
    let test2=newTest(2, 4, vec!["ADSADAD".to_string()]);
    let test1 = Test{modelos:3,preguntas:4, respuestas:vec!["ABBACDDCABCD".to_string(),"BBBAAACCCDDD".to_string(),"CDCDACACBBAA".to_string(),"AABBAACCAADD".to_string()],acierto:3,fallo:-1};
    println!("{}",test2.modelos);
    println!("{}",test1.CorrigueModelo(3, "CDC ABAAB AA".to_string()));
    let a1=newAlumno("pepo".to_string(), 2, "BBBAAACCCAAA".to_string());
    println!("{}",a1.Corrige(test1));
    let mut t3= newTest(3, 10, vec!["ASD".to_string()]);
    t3.generar_respuestas()
}
fn newTest(modelos:i32,preguntas:i32,respuestas:Vec<String>)->Test{
    Test { modelos: modelos, preguntas: preguntas, respuestas: respuestas, acierto: 3, fallo: 1 }
}
fn newAlumno(nombre:String,modelo:i32,respuesta:String)->Alumno{
    Alumno { nombre: nombre, modelo: modelo ,respuesta: respuesta }
}
struct Test{
    modelos:i32,
    preguntas:i32,
    respuestas:Vec<String>,
    acierto:i32,
    fallo:i32,
}
struct Alumno{
    nombre:String,
    modelo:i32,
    respuesta:String,
}
impl Test {
    fn CorrigueModelo(&self,mut modelo:i32 ,respuestas:String,)->i32{
        let mut contador=0;
        modelo=modelo-1;
        let correctas=self.respuestas.get(modelo as usize).unwrap();
        let mut puntaje = 0;
        loop{
            if correctas.chars().nth(contador).unwrap()==respuestas.chars().nth(contador).unwrap() {
                puntaje+=self.acierto;
                contador=contador+1;
            }
            else if ' '==respuestas.chars().nth(contador).unwrap() {
                puntaje+=0;
                contador=contador+1;
            }
            else {
                puntaje+=self.fallo;
                contador=contador+1;
            }
            if contador==respuestas.len(){
                break;
            }
        }
        return puntaje;
        
    }
    fn generar_respuestas(& mut self){
       let charset="ABCD";
       let mut vector=vec![];
       let mut contador=0;
       loop {
            vector.push(generate((self.preguntas) as usize,charset).to_string());
            contador+=1;
            println!("{}",generate((self.preguntas) as usize,charset).to_string());
            if contador==self.modelos{
                break;
            }
       }
       self.respuestas=vector;
    }
}
impl Alumno {
    fn Corrige(&self,test:Test)->i32{
        let mut puntaje=0;
        let mut contador=0;
        let correctas=test.respuestas.get((self.modelo-1) as usize).unwrap();
        loop {
            if correctas.chars().nth(contador).unwrap()==self.respuesta.chars().nth(contador).unwrap() {
                puntaje+=test.acierto;
            }
            else if self.respuesta.chars().nth(contador).unwrap()==' '{
                puntaje+=0;
            }
            else {
                puntaje+=test.fallo;
            }
            contador+=1;
            if contador==self.respuesta.len() {
                break;
            }
        }
        return puntaje;
        
    }
}

use lib::*;

mod lib;

fn main() {
    let respuestas = vec!["ABBACDDCABCD".to_owned(),"BBBAAACCCDDD".to_owned(),"CDCDACACBBAA".to_owned(),"AABBAACCAADD".to_owned()];

    let test1 = Test::new(4,12,respuestas.clone(),Test::default().val_correcta,Test::default().val_incorrecta);
    let alu1 = Alumno::new("Juan".to_owned(),3,"CDC ABAAB AA".to_owned());
    println!("metodo de test: {}",test1.corrige_modelo(3, "CDC ABAAB AA"));
    println!("metodo de alumno: {}",alu1.corrige(test1));
    let sim1 = SimTest::new(Test::new(4,12,respuestas.clone(),Test::default().val_correcta,Test::default().val_incorrecta));
    sim1.simula(20);
}
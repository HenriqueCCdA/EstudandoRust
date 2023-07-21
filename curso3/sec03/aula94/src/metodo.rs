pub struct Pessoa {
    pub nome: String,
    pub sobrenome: String
}

impl Pessoa {
    pub fn qual_nome(&self) {
        println!("Meu nome é {}", self.nome);
    }

    pub fn nome_completo(&self) {
        println!("Meu nome completo é {} {}", self.nome, self.sobrenome);
    }
}

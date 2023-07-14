struct Produto {
    nome: String,
    preco: f32,
    quantidade: i32,
}

impl Produto {
    fn mudar_nome(&mut self, novo_nome: &str) {
        self.nome = novo_nome.to_string();
    }

    fn mudar_preco(&mut self, novo_preco: f32){
        self.preco = novo_preco;
    }

    fn mudar_quantidade(&mut self, nova_quantidade: i32) {
        self.quantidade = nova_quantidade;
    }

    fn imprimir_dados(&self) {
        println!("Nome: {}", self.nome);
        println!("Prreco: {}", self.preco);
        println!("Quantidade: {}", self.quantidade);
    }
}


fn main() {
    let mut meu_produto = Produto{
        nome: "Produto X".to_string(),
        preco: 10.99,
        quantidade: 20,
    };
    meu_produto.imprimir_dados();
    meu_produto.mudar_nome("Produto Y");
    meu_produto.mudar_preco(14.99);
    meu_produto.mudar_quantidade(15);
    meu_produto.imprimir_dados();
}

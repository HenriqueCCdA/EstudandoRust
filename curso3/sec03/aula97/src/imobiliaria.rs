pub struct Imobiliaria {
    pub nome: String,
    pub endereco: String,
    pub imoveis: Vec<Imovel>,
}

pub struct Imovel {
    pub endereco: String,
    pub preco: f32,
    pub num_quartos: u8,
    pub num_banheiros: u8,
    pub metragem: f32,
}

impl Imobiliaria {
    pub fn novo_imovel(&mut self, endereco: String, preco: f32, num_quartos: u8, num_banheiros: u8, metragem: f32) {
        let imovel = Imovel {
            endereco,
            preco,
            num_quartos,
            num_banheiros,
            metragem,
        };
        self.imoveis.push(imovel);
    }

    pub fn listar_imoveis(&self) {
        for imovel in &self.imoveis {
            println!("Endereco: {} | Preco: {} | Quartos: {} | Banheiros {} | Metragem: {}",
                imovel.endereco,
                imovel.preco,
                imovel.num_quartos,
                imovel.num_banheiros,
                imovel.metragem,
            )
        }
    }

}

struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    hash: String,
    prev_hash: String,
}

impl Block {
    fn new(index: u64, timestamp: u64, data: String, hash: String, prev_hash: String) -> Block {

        Block {
            index,
            timestamp,
            data,
            hash,
            prev_hash,
        }

    }
    // Método que retorna o tamanho da dado (data) do bloco
    fn data_size(&self) -> usize{
        self.data.len()
    }
    // MMétodo que retorna o tempo de criação do bloco em segundos
    fn creation_time(&self) -> u64 {
        self.timestamp / 1000
    }
}


fn main() {

    let my_block = Block::new(
        0,
        1605991464000,
        "dados do bloco".to_string(),
        "hash".to_string(),
        "hash anterior".to_string(),
    );
    let size = my_block.data_size();
    let time = my_block.creation_time();

    println!("O tamanho do dado do bloco é: {} e foi criado em {} segundos", size, time);

}

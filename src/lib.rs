#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

pub struct Catalogo {
    produtos: Vec<Produto>,
}

impl Catalogo {
    pub fn new() -> Self {
        Catalogo { produtos: Vec::new() }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.produtos.push(produto);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Vec<&Produto> {
        let nome_lower = nome.to_lowercase();
        self.produtos
            .iter()
            .filter(|p| p.nome.to_lowercase().contains(&nome_lower))
            .collect()
    }
}


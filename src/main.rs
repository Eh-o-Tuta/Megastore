use megastore::{Catalogo, Produto};

fn main() {
    let mut catalogo = Catalogo::new();

    catalogo.adicionar_produto(Produto {
        id: 1,
        nome: "Teclado".to_string(),
        categoria: "Eletrônicos".to_string(),
        preco: 150.0,
    });

    let resultados = catalogo.buscar_por_nome("teclado");

    println!("Resultados da busca por 'teclado':");
    for produto in resultados {
        println!("{:?}", produto);
    }
}

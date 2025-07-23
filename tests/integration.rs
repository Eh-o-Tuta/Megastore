use megastore::{Catalogo, Produto};

#[test]
fn teste_busca_produto() {
    let mut catalogo = Catalogo::new();

    catalogo.adicionar_produto(Produto {
        id: 1,
        nome: "Teclado".to_string(),
        categoria: "Eletrônicos".to_string(),
        preco: 150.0,
    });

    let resultados = catalogo.buscar_por_nome("teclado");
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].nome, "Teclado");
}

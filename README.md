# 🦀 Contagem de Pares em Vetor - Rust

Este projeto implementa uma função em **Rust** para contar quantos pares de cada número existem em um vetor de inteiros.

---

## 📌 Índice
- [Objetivo do Projeto](#-objetivo-do-projeto)
- [Tecnologias Utilizadas](#-tecnologias-utilizadas)
- [Funcionalidades](#-funcionalidades)
- [Instalação e Configuração](#-instalacao-e-configuracao)
- [Executando o Código](#-executando-o-codigo)
- [Executando os Testes](#-executando-os-testes)
- [Contato](#-contato)

---

## 🎯 Objetivo do Projeto

O objetivo deste projeto é implementar uma função que recebe um array de inteiros e retorna um **HashMap** contendo a contagem de pares completos de cada número presente no vetor.

---

## 🛠 Tecnologias Utilizadas

- **Linguagem:** Rust
- **Bibliotecas:**
  - `std::collections::HashMap`
- **Gerenciador de Dependências:** Cargo

---

## ✨ Funcionalidades

- `is_vetor_par(value: [i8; 5]) -> HashMap<i8, u8>` →
  - Conta a frequência de cada número no vetor.
  - Retorna um `HashMap` onde a chave é o número e o valor representa quantos pares completos ele tem.

Exemplo de saída:
```rust
let list = [1, 2, 2, 3, 4];
let pares = is_vetor_par(list);
assert_eq!(pares[&2], 1); // O número 2 aparece duas vezes, formando um par.
```

---

## 🛠 Instalação e Configuração

1. Instale o **Rust** e o **Cargo** se ainda não tiver:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone o repositório:
   ```bash
   git clone https://github.com/seu-usuario/seu-repositorio.git
   cd seu-repositorio
   ```
3. Compile o código:
   ```bash
   cargo build
   ```

---

## ▶️ Executando o Código

Para rodar o código principal:
```bash
cargo run
```

---

## 🧪 Executando os Testes

Para rodar os testes automatizados:
```bash
cargo test
```
Isso executará todos os testes dentro do módulo `tests`.

---

## 📩 Contato

Se tiver dúvidas ou sugestões, entre em contato:
📧 **E-mail:** [asilvaperoba@gmail.com](mailto:asilvaperoba@gmail.com)


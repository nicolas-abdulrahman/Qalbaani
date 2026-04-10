# [Qalbaani]

### 1. Descrição do Projeto
Este repositório contém um projeto de caráter estritamente **educacional e experimental**. O objetivo principal é o estudo da integração entre uma interface mobile nativa e um servidor de alta performance utilizando a linguagem **Rust**.

A aplicação funciona como um chat em tempo real, servindo como laboratório para práticas de arquitetura de sistemas, gerenciamento de estado e comunicação via rede.

---

### 2. Arquitetura do Sistema
O projeto utiliza uma estrutura de **monorepo**, facilitando o versionamento sincronizado entre as duas frentes de desenvolvimento:

* **/android**: Frontend nativo desenvolvido especificamente para a plataforma **Android** via Android Studio. (Nota: Atualmente, o suporte é exclusivo para Android).
* **/backend**: Servidor de alta performance desenvolvido em **Rust**, focado em segurança de memória e concorrência.

---

### 3. Pilha Tecnológica
* **Mobile**: Kotlin / Android SDK
* **Servidor**: Rust (Tokio / Axum ou Actix-web)
* **Protocolo de Comunicação**: JSON via HTTP / WebSockets
* **Gerenciamento de Dependências**: Cargo (Rust) e Gradle (Android)

---

### 4. Status de Desenvolvimento
> **Nota de Observação**: Este é um **projeto de aprendizado**.
> * O código não é destinado a ambientes de produção.
> * As funcionalidades são implementadas para fins de teste de lógica e exploração de ferramentas.
> * O foco atual é a estabilidade da comunicação entre o cliente Android e o servidor Rust.

---

### 5. Configuração e Execução

#### 5.1. Backend (Rust)
É necessário ter o `rustc` e o `cargo` instalados.
1. Acesse o diretório: `cd backend`
2. Execute o servidor em modo de desenvolvimento:
   ```bash
   cargo run
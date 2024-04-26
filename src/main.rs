struct User
{
    username: String,
    email: String,
    ativo: bool,
    genero: String
}

fn user(usuario: &User)
{
    println!("O username da pessoa é: {}", usuario.username);
}

fn main()
{
    let mut pessoa = User {username: String::from("fercosmig"), email: String::from("fercosmig@gmail.com"), ativo: true, genero: String::from("masculino") };

    println!("username: {}", pessoa.username);
    println!("e-mail: {}", pessoa.email);
    println!("ativo: {}", pessoa.ativo);
    println!("genero: {}", pessoa.genero);   

    pessoa.username  = String::from("fomigliorini");
    pessoa.email = String::from("fomigliorini@gmail.com");
    pessoa.ativo = false;
    pessoa.genero = String::from("feminino");

    println!("username: {}", pessoa.username);
    println!("e-mail: {}", pessoa.email);
    println!("ativo: {}", pessoa.ativo);
    println!("genero: {}", pessoa.genero);

    user(&pessoa);
    user(&pessoa);
}

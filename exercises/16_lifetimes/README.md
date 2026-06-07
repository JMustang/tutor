# Lifetimes

Os tempos de vida (lifetimes) informam ao compilador como verificar se as referências têm tempo de vida suficiente para serem válidas em qualquer situação. Por exemplo, os tempos de vida dizem:
"certifique-se de que o parâmetro 'a' tenha o mesmo tempo de vida que o parâmetro 'b' para que o valor de retorno seja válido".

Eles são necessários apenas em empréstimos (borrows), ou seja, referências,
já que parâmetros copiados ou movimentações pertencem ao seu escopo e não podem ser referenciados fora dele. Os tempos de vida permitem que o código que chama funções, por exemplo, seja verificado para garantir que seus argumentos sejam válidos. Os tempos de vida são restritivos para quem os chama.

Se você quiser saber mais sobre anotações de tempo de vida, consulte a documentação.
[lifetimekata](https://tfpk.github.io/lifetimekata/) O projeto
tem um estilo de exercícios semelhante ao Rustlings, mas é totalmente voltado para
aprender a escrever anotações ao longo da vida.

## Further information

- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

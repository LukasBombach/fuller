use std::str::Chars;

pub fn program(source_code: Chars) {

  /*
   * Can contain
   * (Statement | Declaration)[]
   *
   * Statement
   *   BlockStatement                {
   *   VariableStatement             var
   *   EmptyStatement                ;
   *   IfStatement                   if
   *     DoWhileStatement            do
   *     WhileStatement              while
   *     ForStatement                for
   *     ForInOfStatement            for
   *   ContinueStatement             continue
   *   BreakStatement                break
   *   ReturnStatement               return
   *   WithStatement                 with
   *   LabelledStatement             [identifier]:
   *   ThrowStatement                throw
   *   TryStatement                  try
   *   DebuggerStatement             debugger
   *
   *
   * Declaration
   *   FunctionDeclaration           function
   *   GeneratorDeclaration          function *
   *   AsyncFunctionDeclaration      async function
   *   AsyncGeneratorDeclaration     async function *
   *   ClassDeclaration              class
   *   LexicalDeclaration            let
   *   LexicalDeclaration            const
   *
   *
   *
   *
   * todo
   *   ExpressionStatement??
   */
}

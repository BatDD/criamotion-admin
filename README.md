CriaMotion Instalador - REBUILD ADMIN
====================================
O .exe original e Tauri v2 + Rust. Os textos da tela vao comprimidos dentro do binario, entao recriamos o fonte.

COMO BUILDAR (Windows):
1. Instale Node 20 + Rust (rustup) + tauri-cli
2. npm install
3. npx tauri build -> gera o novo .exe em src-tauri/target/release/bundle/nsis/

MELHORIAS ADMIN:
- Botao Admin no rodape, login padrao Admin/Admin
- Apos login, use chave ADMIN -> bypass local sem gastar token
- Alunos usam chave normal via POST https://api.criamotion.com/api/activation/installer-access
- Arquivos: CriaLUT.aex, CriaParallax.aex, CriaMotionFast.aex, CEP com.criamotion.motionpanel

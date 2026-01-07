# 📦 Guia de Deployment - MCPDHR

## Railway.app Deployment

### ✅ Passo 1: Preparar o Repositório

- [x] Arquivo `Dockerfile` criado
- [x] Arquivo `railway.json` criado
- [x] `.env.example` com documentação
- [x] `.gitignore` configurado
- [x] Código compilável

### ✅ Passo 2: Criar Projeto no Railway

1. Acesse https://railway.app/dashboard
2. Clique em "New Project"
3. Selecione "Deploy from GitHub"
4. Autorize o GitHub
5. Selecione `JCDaher/mcpdhr`

### ✅ Passo 3: Configurar Variáveis

No dashboard Railway:
1. Clique no serviço
2. Vá para "Variables"
3. Adicione:
```
PINECONE_API_KEY=your_key
PINECONE_ASSISTANT_HOST=https://prod-1-data.ke.pinecone.io
PORT=8080
LOG_LEVEL=info
```

### ✅ Passo 4: Monitorar Deploy

- Build status na aba "Build Logs"
- Deploy status na aba "Deployment Logs"
- Domínio público em "Settings" → "Domains"

### ✅ Passo 5: Testar Produção
```bash
# Substitua YOUR_RAILWAY_URL com a URL do seu projeto
curl https://YOUR_RAILWAY_URL/health
```

## 🐛 Troubleshooting

### Build falha com erro Rust

Verifique:
- [ ] `Cargo.toml` está correto
- [ ] `src/main.rs` compila localmente
- [ ] Dependências são compatíveis com Linux

### Container não inicia

Verifique:
- [ ] Dockerfile está correto
- [ ] Porta está correta (8080)
- [ ] Variáveis de ambiente estão setadas

### Conexão Pinecone falha

Verifique:
- [ ] API Key é válida
- [ ] Host está correto
- [ ] Rede permite acesso a Pinecone

## 📊 Monitoramento

### Logs no Railway

1. Acesse seu projeto
2. Clique em "Logs"
3. Veja logs em tempo real

### Health Endpoint

Chame regularmente:
```bash
curl https://your-railway-url/health
```

Status esperado: `{"status":"healthy",...}`

## 🔄 Updates

Para atualizar após mudar o código:

1. Faça push para `main`
2. Railway detecta automaticamente
3. Rebuild e redeploy acontecem
4. Novo código vai ao vivo

Tempo típico: 3-5 minutos
```

Clique em **"Commit changes"**

---

## ✅ **RESUMO DO QUE FOI CRIADO**

Seus repositório agora tem:
```
mcpdhr/
├── src/
│   └── main.rs              # Código principal (Rust)
├── .github/
│   └── workflows/
│       └── docker.yml       # CI/CD automático
├── .gitignore              # Ignora arquivos sensíveis
├── Cargo.toml              # Dependências Rust
├── Dockerfile              # Container da aplicação
├── docker-compose.yml      # Para testes locais
├── railway.json            # Config Railway
├── .env.example            # Template variáveis
├── README.md               # Documentação
└── DEPLOYMENT.md           # Guia deploy

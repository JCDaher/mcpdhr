# MCPDHR - Pinecone MCP Server

MCP (Model Context Protocol) server para integração com Pinecone Vector Database.

## 🚀 Características

- ✅ Busca semântica em documentos
- ✅ Integração com Pinecone
- ✅ API REST moderna
- ✅ Health checks automáticos
- ✅ Logs estruturados
- ✅ Deployment automático no Railway

## 📋 Pré-requisitos

- Rust 1.75+ (para desenvolvimento local)
- Docker (para containerização)
- Conta Pinecone com API Key
- Conta Railway (para deploy)

## 🛠️ Desenvolvimento Local

### 1. Clone o repositório
```bash
git clone https://github.com/JCDaher/mcpdhr.git
cd mcpdhr
```

### 2. Configure variáveis de ambiente
```bash
cp .env.example .env
# Edite .env com suas credenciais Pinecone
```

### 3. Execute com Docker
```bash
docker-compose up
```

Ou com Cargo:
```bash
cargo run
```

### 4. Teste a API
```bash
curl http://localhost:8080/health
```

## 🚢 Deploy no Railway

### 1. Conecte seu repositório
1. Acesse [Railway.app](https://railway.app)
2. Clique "New Project" → "Deploy from GitHub"
3. Selecione `JCDaher/mcpdhr`

### 2. Configure variáveis
No painel Railway, vá para Variables e adicione:
```
PINECONE_API_KEY=sua_chave_aqui
PINECONE_ASSISTANT_HOST=https://prod-1-data.ke.pinecone.io
PORT=8080
LOG_LEVEL=info
```

### 3. Deploy automático
Seu servidor será deployado automaticamente a cada push!

## 🔌 Integração com Claude Desktop

### Edite `claude_desktop_config.json`:

**macOS/Linux:**
```bash
~/.config/Claude/claude_desktop_config.json
```

**Windows:**
```
%APPDATA%\\Claude\\claude_desktop_config.json
```

### Adicione sua configuração:
```json
{
  "mcpServers": {
    "mcpdhr": {
      "command": "docker",
      "args": [
        "run",
        "-i",
        "--rm",
        "-e", "PINECONE_API_KEY",
        "-e", "PINECONE_ASSISTANT_HOST",
        "seu-usuario/mcpdhr:latest"
      ],
      "env": {
        "PINECONE_API_KEY": "${PINECONE_API_KEY}",
        "PINECONE_ASSISTANT_HOST": "${PINECONE_ASSISTANT_HOST}"
      }
    }
  }
}
```

## 📚 API Endpoints

### Health Check
```bash
GET /health
Response: {"status": "healthy", "version": "0.1.0"}
```

### Search Documents
```bash
POST /api/search
Content-Type: application/json

{
  "query": "buscar sobre inteligência artificial",
  "top_k": 5
}

Response:
{
  "results": [
    {
      "id": "doc1",
      "score": 0.95,
      "text": "..."
    }
  ]
}
```

## 🔒 Segurança

- Nunca commite o arquivo `.env` com credenciais reais
- Use variáveis de ambiente em produção
- Mantenha a API Key Pinecone segura
- Revise regularmente os logs no Railway

## 📝 Licença

MIT

## 🤝 Contribuindo

1. Fork o projeto
2. Crie uma branch (`git checkout -b feature/xyz`)
3. Commit suas mudanças (`git commit -am 'Add xyz'`)
4. Push para a branch (`git push origin feature/xyz`)
5. Abra um Pull Request

## 📧 Contato

jcdaher - [@JCDaher](https://github.com/JCDaher)

# 🛡️ SecurityShield

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Android-green.svg)]()
[![Version](https://img.shields.io/badge/Version-1.0.0-orange.svg)]()

> 🔒 Ferramenta de segurança defensiva open-source multiplataforma

## ⚠️ IMPORTANTE: USO EXCLUSIVAMENTE DEFENSIVO

Esta é uma ferramenta de **PROTEÇÃO E DEFESA**. Uso para ataques, invasão de privacidade ou qualquer atividade ilegal é **ESTRITAMENTE PROIBIDO** e pode resultar em ação legal.

---

## ✨ Funcionalidades

### 🦠 Antivírus
- Scanner de arquivos (SHA256, MD5)
- Banco de assinaturas local
- Integração VirusTotal (70+ engines)
- Detecção em tempo real

### ⌨️ Detector de Keylogger
- Monitora processos suspeitos
- Análise comportamental
- Detecção de acesso a dispositivos de entrada

### 📶 Analisador Wi-Fi
- Scanner de redes disponíveis
- Análise de segurança (WEP, WPA, WPA2, WPA3)
- Detecção de Evil Twin (AP falso)
- Score de segurança por rede

### 🤖 IA Offline
- Machine Learning (Random Forest)
- Análise comportamental de processos
- Predição de ameaças
- 100% offline (sem servidor)

---

## 📥 Download

### Linux (Debian/Ubuntu)
```bash
# Download
wget https://github.com/Loicram7/securityshield/releases/download/v1.0.0/securityshield_1.0.0_amd64.deb

# Verificar checksum (recomendado)
sha256sum securityshield_1.0.0_amd64.deb
# Compare com: releases/securityshield_1.0.0_amd64.deb.sha256

# Instalar
sudo dpkg -i securityshield_1.0.0_amd64.deb

# Se houver dependências faltando:
sudo apt install -f
```

### Android
```bash
# Download APK
wget https://github.com/Loicram7/securityshield/releases/download/v1.0.0/securityshield_v1.0.0_android_release.apk

# Instalar (habilite "Fontes desconhecidas" nas configurações)
adb install securityshield_v1.0.0_android_release.apk
```

---

## 🚀 Uso

### Linux
```bash
# Linha de comando
securityshield

# Interface gráfica
securityshield
```

### Android
- Abra o app "SecurityShield" no menu de aplicativos

---

## 🛠️ Desenvolvimento

### Requisitos
- **Rust:** 1.75+
- **Python:** 3.11+
- **Flutter:** 3.19+
- **Debian 13** (ou derivados)

### Compilar do código-fonte
```bash
# Clonar repositório
git clone https://github.com/Loicram7/securityshield.git
cd securityshield

# Backend Rust
cd src
cargo build --release

# Módulo Python (IA)
cd ../python
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python ml/trainer.py  # Treinar modelo

# UI Flutter
cd ../ui
flutter pub get
flutter build linux --release  # Para Linux
flutter build apk --release    # Para Android

# Build completo (.deb)
cd ../scripts/build
chmod +x build_linux.sh
./build_linux.sh
```

---

## 📚 Tecnologias

- **Backend:** Rust 🦀 (segurança e performance)
- **IA:** Python + scikit-learn 🤖
- **UI:** Flutter + Dart 🎨
- **Rede:** libpcap, nmcli
- **ML:** Random Forest (offline)

---

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Fork o projeto
2. Crie uma branch (`git checkout -b feature/MinhaFeature`)
3. Commit suas mudanças (`git commit -m 'Adiciona MinhaFeature'`)
4. Push para a branch (`git push origin feature/MinhaFeature`)
5. Abra um Pull Request

**Leia:** [CONTRIBUTING.md](CONTRIBUTING.md) e [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)

---

## 📄 Licença

Este projeto é licenciado sob **GPL-3.0** - veja [LICENSE](LICENSE) para detalhes.

### O que isso significa:
- ✅ Você pode usar, modificar e distribuir
- ✅ Deve manter o código open-source
- ✅ Deve usar a mesma licença GPL-3.0
- ❌ **Uso para ataques é PROIBIDO**

---

## ⚖️ Código de Ética

Este projeto segue princípios éticos estritos:

- 🛡️ **Defesa, não ataque**
- 🔒 **Privacidade acima de tudo**
- 🌍 **Open-source para transparência**
- ⚠️ **Sem coleta de dados**
- 🚫 **Contra uso malicioso**

**Leia:** [ETHICS.md](ETHICS.md)

---

## 🙏 Agradecimentos

- Comunidade Rust 🦀
- Projeto scikit-learn 🤖
- Flutter Team 🎨
- Contribuidores open-source 💙

---

## 📞 Contato

- **Issues:** [GitHub Issues](https://github.com/Loicram7/securityshield/issues)
- **Discussões:** [GitHub Discussions](https://github.com/Loicram7/securityshield/discussions)
- **Security:** Veja [SECURITY.md](SECURITY.md) para reportar vulnerabilidades

---

## ⭐ Mostre seu apoio

Se este projeto te ajudou, dê uma ⭐ no GitHub!

---

**SecurityShield v1.0.0** | Criado com ❤️ e Rust 🦀

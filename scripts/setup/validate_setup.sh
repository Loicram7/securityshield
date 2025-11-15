#!/bin/bash

echo "🔍 Validando instalação..."
echo

errors=0

# Verificar Rust
if command -v rustc &> /dev/null; then
    echo "✅ Rust instalado: $(rustc --version)"
else
    echo "❌ Rust NÃO encontrado"
    ((errors++))
fi

# Verificar Python
if command -v python3 &> /dev/null; then
    echo "✅ Python instalado: $(python3 --version)"
else
    echo "❌ Python NÃO encontrado"
    ((errors++))
fi

# Verificar Flutter
if command -v flutter &> /dev/null; then
    echo "✅ Flutter instalado"
else
    echo "❌ Flutter NÃO encontrado"
    ((errors++))
fi

# Verificar libpcap
if pkg-config --exists libpcap; then
    echo "✅ libpcap instalado"
else
    echo "❌ libpcap NÃO encontrado"
    ((errors++))
fi

echo
if [ $errors -eq 0 ]; then
    echo "🎉 Todos os componentes instalados com sucesso!"
    echo "   Você está pronto para começar o desenvolvimento!"
else
    echo "⚠️  $errors erro(s) encontrado(s). Revise a instalação."
fi

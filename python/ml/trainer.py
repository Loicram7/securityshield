"""
Script para treinar modelo de IA
"""

import numpy as np
from threat_model import ThreatDetectionModel
from feature_extractor import FeatureExtractor

def generate_synthetic_data(n_samples: int = 1000):
    """
    Gera dados sintéticos para treinamento inicial
    
    Na prática, você coletaria dados reais de processos
    """
    np.random.seed(42)
    
    # 70% processos normais, 30% ameaças
    n_safe = int(n_samples * 0.7)
    n_threat = n_samples - n_safe
    
    # PROCESSOS NORMAIS (label=0)
    safe_processes = []
    for _ in range(n_safe):
        safe_processes.append({
            'name': np.random.choice(['chrome', 'firefox', 'python', 'code']),
            'cpu_usage': np.random.uniform(0, 30),  # CPU baixo/médio
            'memory': int(np.random.uniform(100_000_000, 2_000_000_000)),  # 100MB-2GB
            'threads': int(np.random.uniform(1, 50)),
            'connections': int(np.random.uniform(0, 20)),
        })
    
    # PROCESSOS MALICIOSOS (label=1)
    threat_processes = []
    for _ in range(n_threat):
        threat_processes.append({
            'name': np.random.choice(['keylogger', 'spyware', 'backdoor', 'trojan']),
            'cpu_usage': np.random.uniform(0, 10),  # CPU muito baixo (stealth)
            'memory': int(np.random.uniform(10_000_000, 500_000_000)),  # 10-500MB
            'threads': int(np.random.uniform(1, 100)),  # Muitos threads
            'connections': int(np.random.uniform(10, 100)),  # Muitas conexões
        })
    
    # Extrair features
    extractor = FeatureExtractor()
    
    X_safe = extractor.extract_batch(safe_processes)
    X_threat = extractor.extract_batch(threat_processes)
    
    X = np.vstack([X_safe, X_threat])
    y = np.array([0] * n_safe + [1] * n_threat)
    
    # Embaralhar
    indices = np.random.permutation(len(X))
    X = X[indices]
    y = y[indices]
    
    return X, y

def main():
    print("🤖 Treinando modelo de detecção de ameaças\n")
    
    # Gerar dados
    print("→ Gerando dados sintéticos...")
    X_train, y_train = generate_synthetic_data(n_samples=1000)
    print(f"  Dataset: {len(X_train)} amostras")
    print(f"  Safe: {sum(y_train == 0)}, Threats: {sum(y_train == 1)}\n")
    
    # Treinar modelo
    print("→ Treinando Random Forest...")
    model = ThreatDetectionModel()
    model.train(X_train, y_train)
    print()
    
    # Importância das features
    print("→ Importância das features:")
    feature_names = ['CPU', 'Memory', 'Threads', 'Connections', 'Suspicious Name']
    importances = model.get_feature_importance()
    for name, importance in zip(feature_names, importances):
        print(f"  {name}: {importance:.3f}")
    print()
    
    # Salvar modelo
    model_path = "../models/threat_detector.pkl"
    print(f"→ Salvando modelo em: {model_path}")
    
    # Criar diretório se não existir
    from pathlib import Path
    Path("../models").mkdir(exist_ok=True)
    
    model.save(model_path)
    print("\n✓ Treinamento concluído!")

if __name__ == "__main__":
    main()

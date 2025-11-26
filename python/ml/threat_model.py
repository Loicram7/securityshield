"""
Modelo de IA para detecção de ameaças
"""

import pickle
import numpy as np
from pathlib import Path
from typing import Optional, Tuple
from sklearn.ensemble import RandomForestClassifier

class ThreatDetectionModel:
    """Modelo de ML para detectar ameaças baseado em comportamento"""
    
    def __init__(self, model_path: Optional[str] = None):
        self.model: Optional[RandomForestClassifier] = None
        self.is_trained = False
        
        if model_path and Path(model_path).exists():
            self.load(model_path)
    
    def train(self, X_train: np.ndarray, y_train: np.ndarray):
        """
        Treina o modelo
        
        Args:
            X_train: Features (N x 5)
            y_train: Labels (N,) - 0=safe, 1=threat
        """
        self.model = RandomForestClassifier(
            n_estimators=100,
            max_depth=10,
            random_state=42,
            class_weight='balanced'  # Lidar com desbalanceamento
        )
        
        self.model.fit(X_train, y_train)
        self.is_trained = True
        
        # Calcular acurácia no treino
        train_acc = self.model.score(X_train, y_train)
        print(f"✓ Modelo treinado. Acurácia no treino: {train_acc:.2%}")
    
    def predict(self, features: np.ndarray) -> float:
        """
        Prediz probabilidade de ser ameaça
        
        Args:
            features: Array (5,) com features extraídas
        
        Returns:
            Probabilidade de ser ameaça (0.0 a 1.0)
        """
        if not self.is_trained:
            # Modelo não treinado, usar heurística simples
            return self._heuristic_predict(features)
        
        # Reshape para (1, 5) se necessário
        if features.ndim == 1:
            features = features.reshape(1, -1)
        
        # Predição
        proba = self.model.predict_proba(features)[0]
        
        # Retornar probabilidade da classe "threat" (1)
        return proba[1] if len(proba) > 1 else 0.0
    
    def _heuristic_predict(self, features: np.ndarray) -> float:
        """Predição heurística quando modelo não está treinado"""
        # features: [cpu, memory, threads, connections, suspicious_name]
        
        score = 0.0
        
        # Nome suspeito = +0.5
        if features[4] > 0.5:
            score += 0.5
        
        # Muitas conexões + baixo CPU = suspeito (pode ser backdoor)
        if features[3] > 0.3 and features[0] < 0.1:
            score += 0.3
        
        # Muitos threads = pode ser malware multi-thread
        if features[2] > 0.5:
            score += 0.2
        
        return min(score, 1.0)
    
    def save(self, path: str):
        """Salva modelo em disco"""
        if not self.is_trained:
            raise ValueError("Modelo não foi treinado ainda")
        
        with open(path, 'wb') as f:
            pickle.dump(self.model, f)
        
        print(f"✓ Modelo salvo em: {path}")
    
    def load(self, path: str):
        """Carrega modelo de disco"""
        with open(path, 'rb') as f:
            self.model = pickle.load(f)
        
        self.is_trained = True
        # print(f"✓ Modelo carregado de: {path}")
    
    def get_feature_importance(self) -> Optional[np.ndarray]:
        """Retorna importância das features"""
        if not self.is_trained:
            return None
        
        return self.model.feature_importances_

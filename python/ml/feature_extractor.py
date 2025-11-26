"""
Extrator de features para análise de ameaças
"""

import numpy as np
from typing import Dict, List

class FeatureExtractor:
    """Extrai features de processos para análise de IA"""
    
    def __init__(self):
        self.feature_names = [
            'cpu_percent',
            'memory_percent', 
            'num_threads',
            'num_connections',
            'has_suspicious_name',
        ]
    
    def extract_from_process(self, process_info: Dict) -> np.ndarray:
        """
        Extrai features de informações de processo
        
        Args:
            process_info: Dict com info do processo
                - name: str
                - cpu_usage: float (0-100)
                - memory: int (bytes)
                - threads: int
                - connections: int
        
        Returns:
            Array numpy com 5 features normalizadas
        """
        features = []
        
        # Feature 1: CPU usage normalizado (0-1)
        cpu = process_info.get('cpu_usage', 0.0)
        features.append(min(cpu / 100.0, 1.0))
        
        # Feature 2: Memory usage normalizado (0-1)
        # Assumindo 16GB RAM = 16 * 1024 * 1024 * 1024 bytes
        memory_bytes = process_info.get('memory', 0)
        memory_gb = memory_bytes / (1024 ** 3)
        features.append(min(memory_gb / 16.0, 1.0))
        
        # Feature 3: Número de threads normalizado
        threads = process_info.get('threads', 1)
        features.append(min(threads / 100.0, 1.0))
        
        # Feature 4: Número de conexões de rede
        connections = process_info.get('connections', 0)
        features.append(min(connections / 50.0, 1.0))
        
        # Feature 5: Nome suspeito (0 ou 1)
        name = process_info.get('name', '').lower()
        suspicious_keywords = ['key', 'log', 'capture', 'spy', 'hack']
        has_suspicious = any(kw in name for kw in suspicious_keywords)
        features.append(1.0 if has_suspicious else 0.0)
        
        return np.array(features, dtype=np.float32)
    
    def extract_batch(self, processes: List[Dict]) -> np.ndarray:
        """Extrai features de múltiplos processos"""
        return np.array([
            self.extract_from_process(proc) 
            for proc in processes
        ])

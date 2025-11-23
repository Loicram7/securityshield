"""
Interface Python para ser chamada pelo Rust
"""

import json
import sys
from pathlib import Path
from ml.threat_model import ThreatDetectionModel
from ml.feature_extractor import FeatureExtractor

# Carregar modelo
MODEL_PATH = Path(__file__).parent / "models" / "threat_detector.pkl"
model = ThreatDetectionModel(str(MODEL_PATH))
extractor = FeatureExtractor()

def predict_threat(process_info: dict) -> dict:
    """
    Prediz se processo é ameaça
    
    Args:
        process_info: Dict com info do processo
    
    Returns:
        Dict com score e classificação
    """
    try:
        # Extrair features
        features = extractor.extract_from_process(process_info)
        
        # Predizer
        score = model.predict(features)
        
        # Classificar
        if score >= 0.7:
            classification = "threat"
        elif score >= 0.4:
            classification = "suspicious"
        else:
            classification = "safe"
        
        return {
            "success": True,
            "score": float(score),
            "classification": classification
        }
    
    except Exception as e:
        return {
            "success": False,
            "error": str(e)
        }

def main():
    """Entry point quando chamado via linha de comando"""
    if len(sys.argv) < 2:
        print(json.dumps({"success": False, "error": "No input provided"}))
        sys.exit(1)
    
    # Ler JSON do stdin ou argumento
    try:
        process_info = json.loads(sys.argv[1])
        result = predict_threat(process_info)
        print(json.dumps(result))
    except Exception as e:
        print(json.dumps({"success": False, "error": str(e)}))
        sys.exit(1)

if __name__ == "__main__":
    main()

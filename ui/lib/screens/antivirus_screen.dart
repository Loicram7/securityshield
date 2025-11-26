import 'package:flutter/material.dart';
import 'dart:async';

class AntivirusScreen extends StatefulWidget {
  const AntivirusScreen({super.key});

  @override
  State<AntivirusScreen> createState() => _AntivirusScreenState();
}

class _AntivirusScreenState extends State<AntivirusScreen> {
  bool _isScanning = false;
  double _progress = 0.0;
  int _filesScanned = 0;
  int _totalFiles = 0;
  List<Map<String, dynamic>> _threats = [];
  String _currentFile = '';

  Future<void> _startScan() async {
    setState(() {
      _isScanning = true;
      _progress = 0.0;
      _filesScanned = 0;
      _totalFiles = 500; // Simulação
      _threats = [];
    });

    // Simulação de scan progressivo
    for (int i = 0; i <= 100; i++) {
      if (!_isScanning) break;
      
      await Future.delayed(const Duration(milliseconds: 50));
      
      setState(() {
        _progress = i / 100;
        _filesScanned = (i * 5).toInt();
        _currentFile = '/home/user/arquivo_$i.txt';
      });
    }

    setState(() {
      _isScanning = false;
      // Simulação - nenhuma ameaça encontrada
      _threats = [];
    });

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('✅ Scan completo! Nenhuma ameaça detectada.'),
          backgroundColor: Colors.green,
        ),
      );
    }
  }

  void _stopScan() {
    setState(() => _isScanning = false);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('🦠 Antivírus'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Status Card
            Card(
              color: _isScanning 
                ? Colors.blue.withOpacity(0.1) 
                : (_threats.isEmpty ? Colors.green.withOpacity(0.1) : Colors.red.withOpacity(0.1)),
              child: Padding(
                padding: const EdgeInsets.all(20),
                child: Column(
                  children: [
                    Icon(
                      _isScanning 
                        ? Icons.radar 
                        : (_threats.isEmpty ? Icons.verified_user : Icons.warning),
                      size: 64,
                      color: _isScanning 
                        ? Colors.blue 
                        : (_threats.isEmpty ? Colors.green : Colors.red),
                    ),
                    const SizedBox(height: 12),
                    Text(
                      _isScanning 
                        ? 'Escaneando...' 
                        : (_threats.isEmpty ? 'Sistema Limpo' : '${_threats.length} Ameaças'),
                      style: const TextStyle(
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      _isScanning 
                        ? '$_filesScanned de $_totalFiles arquivos'
                        : 'Pronto para escanear',
                      style: const TextStyle(color: Colors.grey),
                    ),
                    if (_isScanning) ...[
                      const SizedBox(height: 16),
                      LinearProgressIndicator(value: _progress),
                      const SizedBox(height: 8),
                      Text(
                        '${(_progress * 100).toInt()}%',
                        style: const TextStyle(fontWeight: FontWeight.bold),
                      ),
                      const SizedBox(height: 8),
                      Text(
                        _currentFile,
                        style: const TextStyle(
                          fontSize: 10,
                          color: Colors.grey,
                          fontFamily: 'monospace',
                        ),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ],
                  ],
                ),
              ),
            ),
            
            const SizedBox(height: 24),
            
            // Controles
            Row(
              children: [
                Expanded(
                  child: ElevatedButton.icon(
                    onPressed: _isScanning ? _stopScan : _startScan,
                    icon: Icon(_isScanning ? Icons.stop : Icons.play_arrow),
                    label: Text(_isScanning ? 'Parar Scan' : 'Iniciar Scan'),
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.all(16),
                      backgroundColor: _isScanning ? Colors.red : null,
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                OutlinedButton.icon(
                  onPressed: _isScanning ? null : () {
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(content: Text('Scan rápido em desenvolvimento')),
                    );
                  },
                  icon: const Icon(Icons.speed),
                  label: const Text('Rápido'),
                  style: OutlinedButton.styleFrom(
                    padding: const EdgeInsets.all(16),
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 24),
            
            // Resultados
            const Text(
              'Resultados do Scan',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 12),
            
            Expanded(
              child: _threats.isEmpty && !_isScanning
                ? const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(Icons.shield_outlined, size: 64, color: Colors.green),
                        SizedBox(height: 16),
                        Text(
                          'Nenhuma ameaça detectada',
                          style: TextStyle(
                            color: Colors.green,
                            fontSize: 16,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: _threats.length,
                    itemBuilder: (context, index) {
                      final threat = _threats[index];
                      return Card(
                        color: Colors.red.withOpacity(0.1),
                        child: ListTile(
                          leading: const Icon(Icons.warning, color: Colors.red),
                          title: Text(threat['name']),
                          subtitle: Text(threat['path']),
                          trailing: IconButton(
                            icon: const Icon(Icons.delete),
                            onPressed: () {
                              // Remover ameaça
                            },
                          ),
                        ),
                      );
                    },
                  ),
            ),
          ],
        ),
      ),
    );
  }
}

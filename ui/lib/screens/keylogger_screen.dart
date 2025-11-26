import 'package:flutter/material.dart';
import 'dart:async';

class KeyloggerScreen extends StatefulWidget {
  const KeyloggerScreen({super.key});

  @override
  State<KeyloggerScreen> createState() => _KeyloggerScreenState();
}

class _KeyloggerScreenState extends State<KeyloggerScreen> {
  bool _isScanning = false;
  bool _isManualScanning = false;
  List<Map<String, dynamic>> _suspiciousProcesses = [];
  Timer? _scanTimer;
  int _scanCount = 0;
  DateTime? _lastScan;

  @override
  void initState() {
    super.initState();
    _startMonitoring();
  }

  @override
  void dispose() {
    _scanTimer?.cancel();
    super.dispose();
  }

  void _startMonitoring() {
    setState(() => _isScanning = true);
    
    // Scan inicial
    _scanForKeyloggers();
    
    // Scan a cada 10 segundos
    _scanTimer = Timer.periodic(const Duration(seconds: 10), (timer) {
      if (_isScanning) {
        _scanForKeyloggers();
      }
    });
  }

  void _stopMonitoring() {
    _scanTimer?.cancel();
    setState(() => _isScanning = false);
  }

  Future<void> _scanForKeyloggers() async {
    setState(() {
      _isManualScanning = true;
      _scanCount++;
    });

    // TODO: Chamar backend Rust para detectar keyloggers
    // Por enquanto, simulação com processos fictícios
    await Future.delayed(const Duration(seconds: 2));
    
    setState(() {
      _lastScan = DateTime.now();
      _isManualScanning = false;
      
      // Simulação - substitua pela chamada real ao backend
      _suspiciousProcesses = [
        {
          'name': 'systemd',
          'pid': 1,
          'status': 'safe',
          'description': 'Processo do sistema - Normal'
        },
        {
          'name': 'gnome-shell',
          'pid': 2341,
          'status': 'safe',
          'description': 'Interface gráfica - Normal'
        },
        {
          'name': 'firefox',
          'pid': 5678,
          'status': 'safe',
          'description': 'Navegador - Normal'
        },
        {
          'name': 'code',
          'pid': 9012,
          'status': 'safe',
          'description': 'Editor de código - Normal'
        },
      ];
    });

    // Mostrar notificação de scan concluído
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('✅ Scan #$_scanCount concluído! ${_suspiciousProcesses.length} processos analisados'),
          backgroundColor: Colors.green,
          duration: const Duration(seconds: 2),
        ),
      );
    }
  }

  Future<void> _manualScan() async {
    await _scanForKeyloggers();
  }

  String _formatTime(DateTime? time) {
    if (time == null) return 'Nunca';
    final now = DateTime.now();
    final diff = now.difference(time);
    
    if (diff.inSeconds < 60) return 'Há ${diff.inSeconds}s';
    if (diff.inMinutes < 60) return 'Há ${diff.inMinutes}m';
    return 'Há ${diff.inHours}h';
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('⌨️ Detector de Keylogger'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Status Card
            Card(
              color: _isScanning ? Colors.green.withOpacity(0.1) : Colors.grey.withOpacity(0.1),
              child: Padding(
                padding: const EdgeInsets.all(20),
                child: Column(
                  children: [
                    Row(
                      children: [
                        Icon(
                          _isScanning ? Icons.radar : Icons.shield,
                          size: 48,
                          color: _isScanning ? Colors.green : Colors.grey,
                        ),
                        const SizedBox(width: 16),
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                _isScanning ? 'Monitoramento Ativo' : 'Monitoramento Pausado',
                                style: const TextStyle(
                                  fontSize: 20,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                              const SizedBox(height: 4),
                              Text(
                                _isScanning 
                                  ? 'Escaneando processos a cada 10s'
                                  : 'Clique em Iniciar para ativar',
                                style: const TextStyle(color: Colors.grey),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 16),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceAround,
                      children: [
                        _buildStatItem('$_scanCount', 'Scans', Colors.blue),
                        _buildStatItem('${_suspiciousProcesses.length}', 'Processos', Colors.orange),
                        _buildStatItem(_formatTime(_lastScan), 'Último Scan', Colors.green),
                      ],
                    ),
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
                    onPressed: _isScanning ? _stopMonitoring : _startMonitoring,
                    icon: Icon(_isScanning ? Icons.pause : Icons.play_arrow),
                    label: Text(_isScanning ? 'Pausar' : 'Iniciar'),
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.all(16),
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: OutlinedButton.icon(
                    onPressed: _isManualScanning ? null : _manualScan,
                    icon: _isManualScanning 
                      ? const SizedBox(
                          width: 16,
                          height: 16,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        )
                      : const Icon(Icons.refresh),
                    label: Text(_isManualScanning ? 'Escaneando...' : 'Scan Manual'),
                    style: OutlinedButton.styleFrom(
                      padding: const EdgeInsets.all(16),
                    ),
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 24),
            
            // Resultados
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text(
                  'Processos Monitorados',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                if (_suspiciousProcesses.isNotEmpty)
                  Chip(
                    label: Text('${_suspiciousProcesses.length}'),
                    backgroundColor: Colors.blue.withOpacity(0.2),
                  ),
              ],
            ),
            const SizedBox(height: 12),
            
            Expanded(
              child: _suspiciousProcesses.isEmpty
                ? const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(Icons.search, size: 64, color: Colors.grey),
                        SizedBox(height: 16),
                        Text(
                          'Execute um scan para ver resultados',
                          style: TextStyle(color: Colors.grey, fontSize: 16),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: _suspiciousProcesses.length,
                    itemBuilder: (context, index) {
                      final process = _suspiciousProcesses[index];
                      return Card(
                        margin: const EdgeInsets.only(bottom: 8),
                        child: ListTile(
                          leading: CircleAvatar(
                            backgroundColor: Colors.green.withOpacity(0.2),
                            child: const Icon(Icons.check_circle, color: Colors.green),
                          ),
                          title: Text(
                            process['name'],
                            style: const TextStyle(fontWeight: FontWeight.bold),
                          ),
                          subtitle: Text(process['description']),
                          trailing: Text(
                            'PID: ${process['pid']}',
                            style: const TextStyle(
                              fontFamily: 'monospace',
                              fontSize: 12,
                              color: Colors.grey,
                            ),
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

  Widget _buildStatItem(String value, String label, Color color) {
    return Column(
      children: [
        Text(
          value,
          style: TextStyle(
            fontSize: 18,
            fontWeight: FontWeight.bold,
            color: color,
          ),
        ),
        Text(
          label,
          style: const TextStyle(color: Colors.grey, fontSize: 12),
        ),
      ],
    );
  }
}

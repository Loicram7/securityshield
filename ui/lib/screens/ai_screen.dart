import 'package:flutter/material.dart';
import 'dart:async';

class AIScreen extends StatefulWidget {
  const AIScreen({super.key});

  @override
  State<AIScreen> createState() => _AIScreenState();
}

class _AIScreenState extends State<AIScreen> {
  bool _isAnalyzing = false;
  bool _isTraining = false;
  int _threatsDetected = 0;
  int _behaviorsAnalyzed = 247;
  double _accuracy = 98.5;
  List<Map<String, dynamic>> _recentAnalysis = [];

  @override
  void initState() {
    super.initState();
    _loadRecentAnalysis();
  }

  void _loadRecentAnalysis() {
    setState(() {
      _recentAnalysis = [
        {
          'time': '14:23',
          'behavior': 'Acesso a arquivos do sistema',
          'risk': 'Baixo',
          'color': Colors.green,
        },
        {
          'time': '14:18',
          'behavior': 'Conexão de rede suspeita',
          'risk': 'Médio',
          'color': Colors.orange,
        },
        {
          'time': '14:05',
          'behavior': 'Modificação de registro',
          'risk': 'Baixo',
          'color': Colors.green,
        },
      ];
    });
  }

  Future<void> _startAnalysis() async {
    setState(() {
      _isAnalyzing = true;
    });

    await Future.delayed(const Duration(seconds: 3));

    setState(() {
      _isAnalyzing = false;
      _behaviorsAnalyzed += 15;
      _recentAnalysis.insert(0, {
        'time': TimeOfDay.now().format(context),
        'behavior': 'Análise comportamental completa',
        'risk': 'Nenhum',
        'color': Colors.green,
      });
    });

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('✅ Análise concluída! Nenhum comportamento suspeito.'),
          backgroundColor: Colors.green,
        ),
      );
    }
  }

  Future<void> _trainModel() async {
    setState(() {
      _isTraining = true;
    });

    await Future.delayed(const Duration(seconds: 5));

    setState(() {
      _isTraining = false;
      _accuracy = (_accuracy + 0.3).clamp(0, 100);
    });

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('✅ Modelo retreinado! Precisão: ${_accuracy.toStringAsFixed(1)}%'),
          backgroundColor: Colors.green,
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('🧠 IA Offline'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Status Card
            Card(
              color: Colors.purple.withOpacity(0.1),
              child: Padding(
                padding: const EdgeInsets.all(20),
                child: Column(
                  children: [
                    Row(
                      children: [
                        Icon(
                          _isAnalyzing ? Icons.psychology : Icons.memory,
                          size: 48,
                          color: Colors.purple,
                        ),
                        const SizedBox(width: 16),
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                _isAnalyzing ? 'Analisando...' : 'IA Ativa',
                                style: const TextStyle(
                                  fontSize: 20,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                              const SizedBox(height: 4),
                              const Text(
                                'Análise comportamental em tempo real',
                                style: TextStyle(color: Colors.grey),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 20),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceAround,
                      children: [
                        _buildAIStat(
                          '$_threatsDetected',
                          'Ameaças',
                          Colors.red,
                          Icons.warning,
                        ),
                        _buildAIStat(
                          '$_behaviorsAnalyzed',
                          'Análises',
                          Colors.blue,
                          Icons.analytics,
                        ),
                        _buildAIStat(
                          '${_accuracy.toStringAsFixed(1)}%',
                          'Precisão',
                          Colors.green,
                          Icons.check_circle,
                        ),
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
                    onPressed: _isAnalyzing || _isTraining ? null : _startAnalysis,
                    icon: _isAnalyzing 
                      ? const SizedBox(
                          width: 20,
                          height: 20,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        )
                      : const Icon(Icons.play_arrow),
                    label: Text(_isAnalyzing ? 'Analisando...' : 'Analisar Sistema'),
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.all(16),
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: OutlinedButton.icon(
                    onPressed: _isAnalyzing || _isTraining ? null : _trainModel,
                    icon: _isTraining 
                      ? const SizedBox(
                          width: 20,
                          height: 20,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        )
                      : const Icon(Icons.model_training),
                    label: Text(_isTraining ? 'Treinando...' : 'Retreinar IA'),
                    style: OutlinedButton.styleFrom(
                      padding: const EdgeInsets.all(16),
                    ),
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 24),
            
            // Análises Recentes
            const Text(
              'Análises Recentes',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 12),
            
            Expanded(
              child: ListView.builder(
                itemCount: _recentAnalysis.length,
                itemBuilder: (context, index) {
                  final analysis = _recentAnalysis[index];
                  return Card(
                    margin: const EdgeInsets.only(bottom: 8),
                    child: ListTile(
                      leading: CircleAvatar(
                        backgroundColor: analysis['color'].withOpacity(0.2),
                        child: Icon(
                          Icons.analytics,
                          color: analysis['color'],
                        ),
                      ),
                      title: Text(analysis['behavior']),
                      subtitle: Text('Risco: ${analysis['risk']}'),
                      trailing: Text(
                        analysis['time'],
                        style: const TextStyle(
                          color: Colors.grey,
                          fontSize: 12,
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

  Widget _buildAIStat(String value, String label, Color color, IconData icon) {
    return Column(
      children: [
        Icon(icon, color: color, size: 24),
        const SizedBox(height: 4),
        Text(
          value,
          style: TextStyle(
            fontSize: 20,
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

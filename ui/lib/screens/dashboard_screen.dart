import 'package:flutter/material.dart';

class DashboardScreen extends StatelessWidget {
  const DashboardScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('🛡️ SecurityShield'),
        actions: [
          IconButton(
            icon: const Icon(Icons.settings),
            onPressed: () {
              // TODO: Navegar para configurações
            },
          ),
        ],
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Status Card
            _buildStatusCard(context),
            const SizedBox(height: 16),
            
            // Módulos
            const Text(
              'Módulos de Segurança',
              style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 12),
            
            _buildModuleCard(
              context,
              icon: Icons.bug_report,
              title: 'Antivírus',
              description: 'Scanner de malware',
              color: Colors.red,
              onTap: () {
                // TODO: Navegar para antivírus
              },
            ),
            
            _buildModuleCard(
              context,
              icon: Icons.keyboard,
              title: 'Detector de Keylogger',
              description: 'Monitora processos suspeitos',
              color: Colors.orange,
              onTap: () {
                // TODO: Navegar para keylogger
              },
            ),
            
            _buildModuleCard(
              context,
              icon: Icons.wifi,
              title: 'Analisador Wi-Fi',
              description: 'Verifica segurança de redes',
              color: Colors.blue,
              onTap: () {
                // TODO: Navegar para wifi
              },
            ),
            
            _buildModuleCard(
              context,
              icon: Icons.psychology,
              title: 'IA Offline',
              description: 'Análise comportamental',
              color: Colors.purple,
              onTap: () {
                // TODO: Navegar para IA
              },
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          _runFullScan(context);
        },
        icon: const Icon(Icons.play_arrow),
        label: const Text('Scan Completo'),
      ),
    );
  }

  Widget _buildStatusCard(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          children: [
            const Icon(
              Icons.shield_outlined,
              size: 64,
              color: Colors.green,
            ),
            const SizedBox(height: 12),
            const Text(
              'Sistema Protegido',
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: Colors.green,
              ),
            ),
            const SizedBox(height: 8),
            const Text(
              'Última verificação: Agora',
              style: TextStyle(color: Colors.grey),
            ),
            const SizedBox(height: 16),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceAround,
              children: [
                _buildStat('0', 'Ameaças', Colors.red),
                _buildStat('4', 'Módulos', Colors.blue),
                _buildStat('100%', 'Seguro', Colors.green),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildStat(String value, String label, Color color) {
    return Column(
      children: [
        Text(
          value,
          style: TextStyle(
            fontSize: 24,
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

  Widget _buildModuleCard(
    BuildContext context, {
    required IconData icon,
    required String title,
    required String description,
    required Color color,
    required VoidCallback onTap,
  }) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: ListTile(
        leading: CircleAvatar(
          backgroundColor: color.withOpacity(0.1),
          child: Icon(icon, color: color),
        ),
        title: Text(
          title,
          style: const TextStyle(fontWeight: FontWeight.bold),
        ),
        subtitle: Text(description),
        trailing: const Icon(Icons.arrow_forward_ios, size: 16),
        onTap: onTap,
      ),
    );
  }

  void _runFullScan(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Scan Completo'),
        content: const Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            CircularProgressIndicator(),
            SizedBox(height: 16),
            Text('Escaneando sistema...'),
          ],
        ),
      ),
    );

    // Simular scan (substituir por chamada real ao backend)
    Future.delayed(const Duration(seconds: 3), () {
      Navigator.of(context).pop();
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('✅ Scan concluído! Nenhuma ameaça detectada.'),
          backgroundColor: Colors.green,
        ),
      );
    });
  }
}

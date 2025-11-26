import 'package:flutter/material.dart';
import 'dart:async';

class WifiScreen extends StatefulWidget {
  const WifiScreen({super.key});

  @override
  State<WifiScreen> createState() => _WifiScreenState();
}

class _WifiScreenState extends State<WifiScreen> {
  bool _isScanning = false;
  List<Map<String, dynamic>> _networks = [];
  Map<String, dynamic>? _connectedNetwork;

  @override
  void initState() {
    super.initState();
    _loadConnectedNetwork();
  }

  void _loadConnectedNetwork() {
    // Simulação de rede conectada
    setState(() {
      _connectedNetwork = {
        'ssid': 'MinhaRedeWiFi',
        'security': 'WPA2',
        'signal': -45,
        'channel': 6,
        'safe': true,
      };
    });
  }

  Future<void> _scanNetworks() async {
    setState(() {
      _isScanning = true;
      _networks = [];
    });

    await Future.delayed(const Duration(seconds: 2));

    setState(() {
      _isScanning = false;
      // Simulação de redes encontradas
      _networks = [
        {
          'ssid': 'MinhaRedeWiFi',
          'security': 'WPA2',
          'signal': -45,
          'channel': 6,
          'safe': true,
          'connected': true,
        },
        {
          'ssid': 'Vizinho_WiFi',
          'security': 'WPA2',
          'signal': -67,
          'channel': 11,
          'safe': true,
          'connected': false,
        },
        {
          'ssid': 'RedeAberta',
          'security': 'Aberta',
          'signal': -72,
          'channel': 1,
          'safe': false,
          'connected': false,
        },
        {
          'ssid': 'Empresa_Guest',
          'security': 'WPA',
          'signal': -58,
          'channel': 6,
          'safe': false,
          'connected': false,
        },
      ];
    });

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('✅ ${_networks.length} redes encontradas'),
          backgroundColor: Colors.green,
        ),
      );
    }
  }

  Color _getSignalColor(int signal) {
    if (signal > -50) return Colors.green;
    if (signal > -60) return Colors.orange;
    return Colors.red;
  }

  IconData _getSignalIcon(int signal) {
    if (signal > -50) return Icons.wifi;
    if (signal > -60) return Icons.wifi_2_bar;
    if (signal > -70) return Icons.wifi_1_bar;
    return Icons.wifi_off;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('📡 Analisador Wi-Fi'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Rede Conectada
            if (_connectedNetwork != null) ...[
              Card(
                color: Colors.blue.withOpacity(0.1),
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    children: [
                      Row(
                        children: [
                          Icon(
                            _getSignalIcon(_connectedNetwork!['signal']),
                            size: 48,
                            color: Colors.blue,
                          ),
                          const SizedBox(width: 16),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                const Text(
                                  'Rede Conectada',
                                  style: TextStyle(
                                    fontSize: 12,
                                    color: Colors.grey,
                                  ),
                                ),
                                Text(
                                  _connectedNetwork!['ssid'],
                                  style: const TextStyle(
                                    fontSize: 20,
                                    fontWeight: FontWeight.bold,
                                  ),
                                ),
                              ],
                            ),
                          ),
                          Icon(
                            _connectedNetwork!['safe'] ? Icons.lock : Icons.lock_open,
                            color: _connectedNetwork!['safe'] ? Colors.green : Colors.red,
                          ),
                        ],
                      ),
                      const SizedBox(height: 16),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceAround,
                        children: [
                          _buildNetworkStat(
                            _connectedNetwork!['security'],
                            'Segurança',
                            _connectedNetwork!['safe'] ? Colors.green : Colors.red,
                          ),
                          _buildNetworkStat(
                            'Canal ${_connectedNetwork!['channel']}',
                            'Frequência',
                            Colors.blue,
                          ),
                          _buildNetworkStat(
                            '${_connectedNetwork!['signal']} dBm',
                            'Sinal',
                            _getSignalColor(_connectedNetwork!['signal']),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
              const SizedBox(height: 24),
            ],
            
            // Controles
            Row(
              children: [
                Expanded(
                  child: ElevatedButton.icon(
                    onPressed: _isScanning ? null : _scanNetworks,
                    icon: _isScanning 
                      ? const SizedBox(
                          width: 20,
                          height: 20,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        )
                      : const Icon(Icons.search),
                    label: Text(_isScanning ? 'Escaneando...' : 'Escanear Redes'),
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.all(16),
                    ),
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 24),
            
            // Lista de Redes
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text(
                  'Redes Disponíveis',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                if (_networks.isNotEmpty)
                  Chip(
                    label: Text('${_networks.length}'),
                    backgroundColor: Colors.blue.withOpacity(0.2),
                  ),
              ],
            ),
            const SizedBox(height: 12),
            
            Expanded(
              child: _networks.isEmpty
                ? const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(Icons.wifi_find, size: 64, color: Colors.grey),
                        SizedBox(height: 16),
                        Text(
                          'Clique em "Escanear Redes"',
                          style: TextStyle(color: Colors.grey, fontSize: 16),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: _networks.length,
                    itemBuilder: (context, index) {
                      final network = _networks[index];
                      return Card(
                        margin: const EdgeInsets.only(bottom: 8),
                        color: network['connected'] 
                          ? Colors.blue.withOpacity(0.1) 
                          : null,
                        child: ListTile(
                          leading: CircleAvatar(
                            backgroundColor: _getSignalColor(network['signal']).withOpacity(0.2),
                            child: Icon(
                              _getSignalIcon(network['signal']),
                              color: _getSignalColor(network['signal']),
                            ),
                          ),
                          title: Row(
                            children: [
                              Expanded(child: Text(network['ssid'])),
                              if (network['connected'])
                                const Chip(
                                  label: Text('Conectada', style: TextStyle(fontSize: 10)),
                                  padding: EdgeInsets.symmetric(horizontal: 8),
                                ),
                            ],
                          ),
                          subtitle: Text('${network['security']} • Canal ${network['channel']}'),
                          trailing: Icon(
                            network['safe'] ? Icons.check_circle : Icons.warning,
                            color: network['safe'] ? Colors.green : Colors.orange,
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

  Widget _buildNetworkStat(String value, String label, Color color) {
    return Column(
      children: [
        Text(
          value,
          style: TextStyle(
            fontSize: 14,
            fontWeight: FontWeight.bold,
            color: color,
          ),
        ),
        Text(
          label,
          style: const TextStyle(color: Colors.grey, fontSize: 10),
        ),
      ],
    );
  }
}

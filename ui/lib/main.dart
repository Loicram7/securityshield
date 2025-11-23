import 'package:flutter/material.dart';
import 'theme/app_theme.dart';
import 'screens/dashboard_screen.dart';

void main() {
  runApp(const SecurityShieldApp());
}

class SecurityShieldApp extends StatefulWidget {
  const SecurityShieldApp({super.key});

  @override
  State<SecurityShieldApp> createState() => _SecurityShieldAppState();
}

class _SecurityShieldAppState extends State<SecurityShieldApp> {
  bool _isDarkMode = true; // Tema escuro por padrão

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SecurityShield',
      debugShowCheckedModeBanner: false,
      theme: AppTheme.lightTheme,
      darkTheme: AppTheme.darkTheme,
      themeMode: _isDarkMode ? ThemeMode.dark : ThemeMode.light,
      home: const DashboardScreen(),
    );
  }
}

import 'package:flutter/material.dart';
import 'package:home_now_mobile/src/rust/api/simple.dart';
import 'package:home_now_mobile/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  Future<void> trn () async {
    String dd = transact();
    print(dd);
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Column(
            children: [
              Text(
                  'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
              TextButton(onPressed: () {
                trn();
              }, child: const Text("Transact"))
            ],
          ),
        ),
      ),
    );
  }
}

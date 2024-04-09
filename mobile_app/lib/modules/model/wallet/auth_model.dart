import 'package:solana/solana.dart';

class WalletAuthModel {
  final Ed25519HDPublicKey publicKey;
  final String authToken;

  WalletAuthModel({
    required this.publicKey,
    required this.authToken,
  });
}

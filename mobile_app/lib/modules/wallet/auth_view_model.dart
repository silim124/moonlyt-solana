import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:moonlyt/modules/model/wallet/auth_model.dart';
import 'package:solana/solana.dart';
import 'package:solana_mobile_client/solana_mobile_client.dart';

class WalletConnectionNotifier extends StateNotifier<WalletAuthModel?> {
  WalletConnectionNotifier() : super(null);

  void setConnectionResult(WalletAuthModel result) {
    state = result;
  }

  void clearConnectionResult() {
    state = null;
  }

  Future<bool> connectWallet() async {
    try {
      final session = await LocalAssociationScenario.create();
      session.startActivityForResult(null).ignore();
      final client = await session.start();
      final result = await client.authorize(
        identityUri: Uri.parse('https://avatars.githubusercontent.com'),
        iconUri: Uri.parse('u/72037442?s=48&v=4'),
        identityName: 'Moonlyt',
        cluster: 'testnet',
      );
      session.close();
      debugPrint("Wallet connection result: $result");
      if (result != null) {
        final connectionResult = WalletAuthModel(
            publicKey: Ed25519HDPublicKey(result.publicKey.toList()),
            authToken: result.authToken);
        setConnectionResult(connectionResult);
        debugPrint("Wallet connection result: ${connectionResult.publicKey}");
        final balance = await getBalance(connectionResult.publicKey.toString());
        if (balance < 1.0) {
          airdrop();
        }
        return true;
      }
      return false;
    } catch (error) {
      debugPrint("Error connecting wallet: $error");
      return false;
    }
  }

  Future<void> airdrop() async {
    final solanaClient = SolanaClient(
      rpcUrl: Uri.parse("https://api.devnet.solana.com"),
      websocketUrl: Uri.parse("wss://api.devnet.solana.com"),
    );
    await solanaClient.requestAirdrop(
      address: state!.publicKey,
      lamports: 1000,
    );
  }

  Future<double> getBalance(String walletAddress,
      {Commitment commitment = Commitment.finalized}) async {
    final solanaClient = SolanaClient(
      rpcUrl: Uri.parse("https://api.devnet.solana.com"),
      websocketUrl: Uri.parse("wss://api.devnet.solana.com"),
    );
    final lamports = await solanaClient.rpcClient
        .getBalance(walletAddress, commitment: commitment)
        .then((result) {
      return result.value;
    });
    return lamports / 1e9;
  }
}

final walletConnectionProvider =
    StateNotifierProvider<WalletConnectionNotifier, WalletAuthModel?>((ref) {
  return WalletConnectionNotifier();
});

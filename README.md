# SuperDev Quiz Task

Deployed below, using Fly.io:
    https://http-server-polished-paper-9309.fly.dev/

## Test

### Keypair Generation
```bash
curl -X POST https://http-server-polished-paper-9309.fly.dev/keypair
```

### Message Signing
```bash
curl -X POST https://http-server-polished-paper-9309.fly.dev/message/sign \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello World","secret":"5hN8HUoyn1u8BWUiUp6eNJJNX66zuFh1eq4zgZGCajiyBuhPCVY1BntjC72YjEds9LNbXaZTRjFszrUPMsbRsSF9"}'
```

### Message Verification
```bash
curl -X POST https://http-server-polished-paper-9309.fly.dev/message/verify \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello World","signature":"otlSTpCz+FTnhfvvJpv0Uth3uNUPV/byuOBt5zGVMjXuf1pB35sMJJowmprtFC3K4Ckxb3CsYhc4Zydp2RreAw==","pubkey":"7mYdWLU8XAfLCMn6aCiNKT5xXBUyjoKZhx8WQTMtngGo"}'
```

### SOL Transfer
```bash
curl -X POST https://http-server-polished-paper-9309.fly.dev/send/sol \
  -H "Content-Type: application/json" \
  -d '{"from":"7mYdWLU8XAfLCMn6aCiNKT5xXBUyjoKZhx8WQTMtngGo","to":"11111111111111111111111111111112","lamports":1000000}'
```

### Token Creation
```bash
curl -X POST https://http-server-polished-paper-9309.fly.dev/token/create \
  -H "Content-Type: application/json" \
  -d '{"mint":"11111111111111111111111111111113","mintAuthority":"7mYdWLU8XAfLCMn6aCiNKT5xXBUyjoKZhx8WQTMtngGo","decimals":9}'
```

### Token Minting
```bash
curl -X POST https://http-server-polished-paper-9309.fly.dev/token/mint \
  -H "Content-Type: application/json" \
  -d '{"mint":"11111111111111111111111111111113","destination":"11111111111111111111111111111114","authority":"7mYdWLU8XAfLCMn6aCiNKT5xXBUyjoKZhx8WQTMtngGo","amount":1000000000}'
```

### Token Transfer
```bash
curl -X POST https://http-server-polished-paper-9309.fly.dev/send/token \
  -H "Content-Type: application/json" \
  -d '{"mint":"11111111111111111111111111111113","destination":"11111111111111111111111111111115","owner":"7mYdWLU8XAfLCMn6aCiNKT5xXBUyjoKZhx8WQTMtngGo","amount":500000000}'
```

## Quiz Task
    https://chambray-couch-746.notion.site/Solana-Fellowship-Assignment-Rust-HTTP-Server-21f9fc39caaa805488b6ef50473472d9?pvs=74
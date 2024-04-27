solana program dump -u m metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metaplex.so

solana-test-validator \
    --reset \
    --url https://api.devnet.solana.com \
    --clone metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s \
    --ledger ./.anchor/test-ledger \
    --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metaplex.so

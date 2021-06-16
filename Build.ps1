cross build --release --target x86_64-unknown-linux-musl && `
	Compress-Archive .\target\x86_64-unknown-linux-musl\release\bootstrap -DestinationPath lambda.zip -Update

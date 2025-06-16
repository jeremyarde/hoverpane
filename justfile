set dotenv-load

version := `cd hoverpane-app && cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version'`
arch := `uname -m | sed 's/arm64/aarch64/;s/x86_64/x86_64/'`

info:
    xcrun notarytool log 591ac8bd-e88e-4eb1-8c7b-e98ae883b1c5 --apple-id $APPLE_ID --password $APPLE_PASSWORD --team-id 4JJPCY2A78

build:
    cd hoverpane-app && cargo packager --release --formats app

sign-app:
    xattr -cr ./target/release/hoverpane.app
    codesign --deep --force --verify \
    --timestamp \
    --options runtime \
    --entitlements hoverpane-app/build_assets/entitlements.plist \
    --sign "$CERTIFICATE_TYPE: $DEVELOPER_ID" \
    ./target/release/hoverpane.app
    codesign --verify --deep --strict --verbose=2 ./target/release/hoverpane.app

create-dmg:
    rm -f ./target/release/hoverpane_{{version}}_{{arch}}.dmg
    create-dmg \
      --volname "HoverPane Installer" \
      --window-pos 200 120 \
      --window-size 800 400 \
      --icon-size 120 \
      --icon "hoverpane.app" 200 190 \
      --hide-extension "hoverpane.app" \
      --app-drop-link 600 185 \
      ./target/release/hoverpane_{{version}}_{{arch}}.dmg \
      ./target/release/hoverpane.app

macos:
    xcrun notarytool submit --apple-id $APPLE_ID --password $APPLE_PASSWORD --team-id 4JJPCY2A78 target/release/hoverpane_{{version}}_{{arch}}.dmg --wait
    xcrun stapler staple target/release/hoverpane_{{version}}_{{arch}}.dmg

pkg-sign:
    # only pkg and sign the app, the dmg can stay as .dmg
    tar -czvf ./target/release/hoverpane_{{version}}_{{arch}}.app.tar.gz -C ./target/release hoverpane.app
    # sign the app, because dmg does not need to be signed (in theory)
    cargo packager signer sign --private-key $CARGO_PACKAGER_SIGN_PRIVATE_KEY ./target/release/hoverpane_{{version}}_{{arch}}.app.tar.gz

build-release:
    mkdir -p ./release/{{version}}
    date -u +"%Y-%m-%dT%H:%M:%SZ" > ./release/{{version}}/published_at.txt
    [ -f ./notes-{{version}}.md ] && cp ./notes-{{version}}.md ./release/{{version}}/
    cp ./target/release/hoverpane_{{version}}_{{arch}}.dmg ./release/{{version}}/
    cp ./target/release/hoverpane_{{version}}_{{arch}}.app.tar.gz ./release/{{version}}/
    cp ./target/release/hoverpane_{{version}}_{{arch}}.app.tar.gz.sig ./release/{{version}}/

uploadnew:
    aws s3 cp ./release/{{version}} s3://hoverpane-app/release/{{version}}/ --recursive

notarize-info:
    xcrun notarytool info 1c737b6f-3439-4801-a1f6-9a402efa8d1e --apple-id $APPLE_ID --team-id 4JJPCY2A78 --password $APPLE_PASSWORD
    spctl -a -t exec -v /Users/jarde/Documents/code/hoverpane/target/release/hoverpane.app

landingpage:
    cd astro-landingpage && npm run build 
    wrangler pages deploy astro-landingpage/dist --project-name widget-maker-landing

logs:
    # cat ~/Library/Application\ Support/com.jarde.hoverpane/hoverpane.log
    tail -f /Users/jarde/Library/Application Support/com.jarde.hoverpane/hoverpane.log
    
all: build sign-app create-dmg macos pkg-sign build-release



